use std::collections::HashMap;
use tonic::{Code, Status};
use tonic_types::{ErrorDetails, StatusExt};

#[derive(Clone)]
pub struct GrpcErrorContext {
    domain: &'static str,
}

impl GrpcErrorContext {
    pub const fn new(domain: &'static str) -> Self {
        Self { domain }
    }

    fn base_error(&self, code: Code, reason: &str, app_code: &str) -> Status {
        let mut details = ErrorDetails::new();

        let mut metadata = HashMap::new();
        metadata.insert("code".to_string(), app_code.to_string());

        details.set_error_info(reason, self.domain, metadata);

        Status::with_error_details(code, app_code, details)
    }

    // ── Semantic helpers ─────────────────────

    pub fn unauthenticated(&self, app_code: &str) -> Status {
        self.base_error(Code::Unauthenticated, "UNAUTHENTICATED", app_code)
    }

    pub fn permission_denied(&self, app_code: &str) -> Status {
        self.base_error(Code::PermissionDenied, "PERMISSION_DENIED", app_code)
    }

    pub fn not_found(&self, app_code: &str) -> Status {
        self.base_error(Code::NotFound, "NOT_FOUND", app_code)
    }

    pub fn already_exists(&self, app_code: &str) -> Status {
        self.base_error(Code::AlreadyExists, "ALREADY_EXISTS", app_code)
    }

    pub fn invalid_argument(&self, app_code: &str) -> Status {
        self.base_error(Code::InvalidArgument, "INVALID_ARGUMENT", app_code)
    }

    pub fn failed_precondition(
        &self,
        violation_type: &str,
        subject: &str,
        app_code: &str,
    ) -> Status {
        let mut details = ErrorDetails::new();

        details.add_precondition_failure_violation(violation_type, subject, app_code);

        let mut metadata = HashMap::new();
        metadata.insert("code".to_string(), app_code.to_string());

        details.set_error_info("FAILED_PRECONDITION", self.domain, metadata);

        Status::with_error_details(Code::FailedPrecondition, app_code, details)
    }
}
