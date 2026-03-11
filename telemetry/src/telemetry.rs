use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider, trace::SdkTracerProvider};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Telemetry {
    tracer_provider: SdkTracerProvider,
    logger_provider: SdkLoggerProvider,
}

impl Telemetry {
    /// Initializes production-grade telemetry:
    /// - **Traces**: OTLP/gRPC → Alloy → Tempo (via `tracing-opentelemetry`)
    /// - **Logs**: OTLP/gRPC → Alloy → Loki (via `opentelemetry-appender-tracing`)
    /// - **Console**: pretty or JSON logs to stdout
    ///
    /// Env vars:
    /// - `OTEL_EXPORTER_OTLP_ENDPOINT` (default: `http://localhost:4317`)
    /// - `OTEL_SERVICE_NAME` (default: `identity-service`)
    /// - `RUST_LOG` (default: `info`)
    /// - `DEPLOY_ENV` — if `production`, uses JSON stdout; otherwise pretty stdout.

    pub fn new(service_name: String, is_production: bool, metric_port: u16) -> Self {
        // ── Resource ────────────────────────────────────────────────────────────
        let resource = Resource::builder()
            .with_service_name(service_name.to_owned())
            .build();

        // ── Trace exporter (OTLP/gRPC → Alloy → Tempo) ────────────────────────
        let span_exporter = SpanExporter::builder()
            .with_tonic()
            .build()
            .expect("failed to create OTLP span exporter");

        let tracer_provider = SdkTracerProvider::builder()
            .with_batch_exporter(span_exporter)
            .with_resource(resource.clone())
            .build();

        opentelemetry::global::set_tracer_provider(tracer_provider.clone());

        // ── Start Prometheus metrics server (background) ────────────────────────
        metrics_exporter_prometheus::PrometheusBuilder::new()
            .with_http_listener(([0, 0, 0, 0], metric_port))
            .install()
            .expect("failed to install Prometheus metrics exporter");

        let tracer = tracer_provider.tracer("identity-service");

        // ── Log exporter (OTLP/gRPC → Alloy → Loki) ───────────────────────────
        let log_exporter = LogExporter::builder()
            .with_tonic()
            .build()
            .expect("failed to create OTLP log exporter");

        let logger_provider = SdkLoggerProvider::builder()
            .with_batch_exporter(log_exporter)
            .with_resource(resource)
            .build();

        // ── Tracing layers ─────────────────────────────────────────────────────
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        // Spans → OTel traces → Tempo
        let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        // Events (tracing::info!, etc.) → OTel logs → Loki
        let otel_log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(otel_trace_layer)
            .with(otel_log_layer);

        if is_production {
            // JSON stdout for machine consumption
            let json_layer = tracing_subscriber::fmt::layer()
                .json()
                .with_target(true)
                .with_span_list(false)
                .with_current_span(true);
            registry.with(json_layer).init();
        } else {
            // Pretty stdout for local development
            let fmt_layer = tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_target(true);
            registry.with(fmt_layer).init();
        }

        Self {
            tracer_provider,
            logger_provider,
        }
    }

    /// Gracefully shut down, flushing all pending spans and logs.
    pub fn shutdown(&self) {
        if let Err(e) = self.tracer_provider.shutdown() {
            eprintln!("Error shutting down tracer provider: {e}");
        }
        if let Err(e) = self.logger_provider.shutdown() {
            eprintln!("Error shutting down logger provider: {e}");
        }
    }
}
