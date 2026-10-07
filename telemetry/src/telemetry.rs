use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider, trace::SdkTracerProvider};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub log_level: String,
    pub otlp_endpoint: String,
    pub sampling_ratio: f64,
    pub metrics_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct TelemetryBuilder {
    service_name: String,
    otlp_endpoint: String,
    is_production: bool,
    metric_port: u16,
    log_level: String,
    sampling_ratio: f64,
    metrics_enabled: bool,
}

impl TelemetryBuilder {
    pub fn new(service_name: impl Into<String>, otlp_endpoint: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            otlp_endpoint: otlp_endpoint.into(),
            is_production: false,
            metric_port: 9090,
            log_level: "info".to_string(),
            sampling_ratio: 1.0,
            metrics_enabled: true,
        }
    }

    pub fn is_production(mut self, is_production: bool) -> Self {
        self.is_production = is_production;
        self
    }

    pub fn metric_port(mut self, metric_port: u16) -> Self {
        self.metric_port = metric_port;
        self
    }

    pub fn log_level(mut self, log_level: impl Into<String>) -> Self {
        self.log_level = log_level.into();
        self
    }

    pub fn otlp_endpoint(mut self, otlp_endpoint: impl Into<String>) -> Self {
        self.otlp_endpoint = otlp_endpoint.into();
        self
    }

    pub fn sampling_ratio(mut self, sampling_ratio: f64) -> Self {
        self.sampling_ratio = sampling_ratio;
        self
    }

    pub fn metrics_enabled(mut self, metrics_enabled: bool) -> Self {
        self.metrics_enabled = metrics_enabled;
        self
    }

    pub fn with_config(mut self, config: &TelemetryConfig) -> Self {
        self.log_level = config.log_level.clone();
        self.otlp_endpoint = config.otlp_endpoint.clone();
        self.sampling_ratio = config.sampling_ratio;
        self.metrics_enabled = config.metrics_enabled;
        self
    }

    pub fn build(self) -> Telemetry {
        Telemetry::init(
            self.service_name,
            self.otlp_endpoint,
            self.is_production,
            self.metric_port,
            self.log_level,
            self.sampling_ratio,
            self.metrics_enabled,
        )
    }
}

pub struct Telemetry {
    tracer_provider: SdkTracerProvider,
    logger_provider: SdkLoggerProvider,
}

impl Telemetry {
    pub fn builder(service_name: impl Into<String>, otlp_endpoint: impl Into<String>) -> TelemetryBuilder {
        TelemetryBuilder::new(service_name, otlp_endpoint)
    }

    /// Initializes production-grade telemetry with explicit configuration.
    pub fn with_config(
        service_name: String,
        is_production: bool,
        metric_port: u16,
        config: &TelemetryConfig,
    ) -> Self {
        Self::builder(service_name, &config.otlp_endpoint)
            .is_production(is_production)
            .metric_port(metric_port)
            .with_config(config)
            .build()
    }

    fn init(
        service_name: String,
        otlp_endpoint: String,
        is_production: bool,
        metric_port: u16,
        log_level: String,
        sampling_ratio: f64,
        metrics_enabled: bool,
    ) -> Self {
        // ── Resource ────────────────────────────────────────────────────────────
        let resource = Resource::builder()
            .with_service_name(service_name.to_owned())
            .build();

        // ── Trace exporter (OTLP/gRPC → Alloy → Tempo) ────────────────────────
        let span_exporter = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(&otlp_endpoint)
            .build()
            .expect("failed to create OTLP span exporter");

        let tracer_provider = SdkTracerProvider::builder()
            .with_batch_exporter(span_exporter)
            .with_sampler(opentelemetry_sdk::trace::Sampler::ParentBased(Box::new(
                opentelemetry_sdk::trace::Sampler::TraceIdRatioBased(sampling_ratio),
            )))
            .with_resource(resource.clone())
            .build();

        opentelemetry::global::set_tracer_provider(tracer_provider.clone());

        // ── Start Prometheus metrics server (background) ────────────────────────
        if metrics_enabled {
            println!("Starting Prometheus metrics server on port {}", metric_port);
            metrics_exporter_prometheus::PrometheusBuilder::new()
                .with_http_listener(([0, 0, 0, 0], metric_port))
                .install()
                .expect("failed to install Prometheus metrics exporter");
        }

        let tracer = tracer_provider.tracer(service_name);

        // ── Log exporter (OTLP/gRPC → Alloy → Loki) ───────────────────────────
        let log_exporter = LogExporter::builder()
            .with_tonic()
            .with_endpoint(&otlp_endpoint)
            .build()
            .expect("failed to create OTLP log exporter");

        let logger_provider = SdkLoggerProvider::builder()
            .with_batch_exporter(log_exporter)
            .with_resource(resource)
            .build();

        // ── Tracing layers ─────────────────────────────────────────────────────
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::try_new(&log_level).unwrap_or_else(|_| EnvFilter::new("info")));

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
