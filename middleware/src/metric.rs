use http::{Request, Response};
use metrics::{counter, histogram};
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use tonic::transport::Body;
use tower::{Layer, Service};

// ── Tower Layer ─────────────────────────────────────────────────────────────

/// Tower layer that automatically records `identity_grpc_requests_total` and
/// `identity_grpc_request_duration_seconds` for every gRPC request.
///
/// Usage in router:
/// ```rust,ignore
/// tonic::transport::Server::builder()
///     .layer(GrpcMetricsLayer)
///     .add_service(...)
/// ```
#[derive(Debug, Clone)]
pub struct GrpcMetricsLayer;

impl<S> Layer<S> for GrpcMetricsLayer {
    type Service = GrpcMetricsMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        GrpcMetricsMiddleware { inner }
    }
}

/// Tower service wrapper that measures gRPC request latency and status.
#[derive(Debug, Clone)]
pub struct GrpcMetricsMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for GrpcMetricsMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Body + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let path = req.uri().path().to_string();
        // Clone inner service (required by tower for concurrent calls)
        let mut inner = self.inner.clone();
        // Swap so the ready one is used
        std::mem::swap(&mut self.inner, &mut inner);

        Box::pin(async move {
            let start = Instant::now();
            let result = inner.call(req).await;

            // Parse /package.ServiceName/MethodName
            let (service, method) = parse_grpc_path(&path);
            let duration = start.elapsed().as_secs_f64();

            let grpc_code = match &result {
                Ok(response) => {
                    // Tonic puts grpc-status in response headers for errors
                    response
                        .headers()
                        .get("grpc-status")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<i32>().ok())
                        .map(code_i32_to_str)
                        .unwrap_or("OK")
                }
                Err(_) => "INTERNAL",
            };

            counter!(
                "identity_grpc_requests_total",
                "method" => method.to_owned(),
                "service" => service.to_owned(),
                "grpc_code" => grpc_code.to_owned(),
            )
            .increment(1);

            histogram!(
                "identity_grpc_request_duration_seconds",
                "method" => method.to_owned(),
                "service" => service.to_owned(),
            )
            .record(duration);

            result
        })
    }
}

/// Extracts service and method from a gRPC path like `/package.Service/Method`.
fn parse_grpc_path(path: &str) -> (&str, &str) {
    let path = path.strip_prefix('/').unwrap_or(path);
    match path.rsplit_once('/') {
        Some((service_path, method)) => {
            // Extract just the service name from "identity.auth.merchant.v1.MerchantAuthService"
            let service = service_path
                .rsplit_once('.')
                .map_or(service_path, |(_, s)| s);
            (service, method)
        }
        None => ("unknown", path),
    }
}

fn code_i32_to_str(code: i32) -> &'static str {
    match code {
        0 => "OK",
        1 => "CANCELLED",
        2 => "UNKNOWN",
        3 => "INVALID_ARGUMENT",
        4 => "DEADLINE_EXCEEDED",
        5 => "NOT_FOUND",
        6 => "ALREADY_EXISTS",
        7 => "PERMISSION_DENIED",
        8 => "RESOURCE_EXHAUSTED",
        9 => "FAILED_PRECONDITION",
        10 => "ABORTED",
        11 => "OUT_OF_RANGE",
        12 => "UNIMPLEMENTED",
        13 => "INTERNAL",
        14 => "UNAVAILABLE",
        15 => "DATA_LOSS",
        16 => "UNAUTHENTICATED",
        _ => "UNKNOWN",
    }
}
