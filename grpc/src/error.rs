use crate::builder::ErrorBuilder;
use std::collections::HashMap;
use tonic::Code;
use tonic_types::ErrorDetails;

#[derive(Clone)]
pub struct GrpcErrorContext {
    domain: &'static str,
}

impl GrpcErrorContext {
    pub const fn new(domain: &'static str) -> Self {
        Self { domain }
    }

    // ─────────────────────────────────────────
    // Core builder
    // ─────────────────────────────────────────

    fn base_error(&self, code: Code, reason: &str, app_code: &str) -> ErrorBuilder {
        let mut details = ErrorDetails::new();

        let mut metadata = HashMap::new();
        metadata.insert("code".to_string(), app_code.to_string());

        details.set_error_info(reason, self.domain, metadata);

        ErrorBuilder {
            code,
            message: app_code.to_string(),
            details,
        }
    }

    // ─────────────────────────────────────────
    // Semantic helpers (ALL gRPC codes)
    // ─────────────────────────────────────────

    pub fn unauthenticated(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::Unauthenticated, "UNAUTHENTICATED", code)
    }

    pub fn permission_denied(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::PermissionDenied, "PERMISSION_DENIED", code)
    }

    pub fn not_found(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::NotFound, "NOT_FOUND", code)
    }

    pub fn already_exists(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::AlreadyExists, "ALREADY_EXISTS", code)
    }

    pub fn invalid_argument(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::InvalidArgument, "INVALID_ARGUMENT", code)
    }

    pub fn failed_precondition(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::FailedPrecondition, "FAILED_PRECONDITION", code)
    }

    pub fn aborted(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::Aborted, "ABORTED", code)
    }

    pub fn cancelled(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::Cancelled, "CANCELLED", code)
    }

    pub fn deadline_exceeded(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::DeadlineExceeded, "DEADLINE_EXCEEDED", code)
    }

    pub fn resource_exhausted(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::ResourceExhausted, "RESOURCE_EXHAUSTED", code)
    }

    pub fn unavailable(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::Unavailable, "UNAVAILABLE", code)
    }

    pub fn internal(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::Internal, "INTERNAL", code)
    }

    pub fn data_loss(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::DataLoss, "DATA_LOSS", code)
    }

    pub fn unimplemented(&self, code: &str) -> ErrorBuilder {
        self.base_error(Code::Unimplemented, "UNIMPLEMENTED", code)
    }
}
