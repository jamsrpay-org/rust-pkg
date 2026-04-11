use tonic::{Code, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub struct ErrorBuilder {
    pub code: Code,
    pub message: String,
    pub details: ErrorDetails,
}

impl ErrorBuilder {
    // ── Precondition ─────────────────────────

    pub fn with_precondition(
        mut self,
        violation_type: &str,
        subject: &str,
        description: &str,
    ) -> Self {
        self.details
            .add_precondition_failure_violation(violation_type, subject, description);
        self
    }

    // ── Validation (BadRequest) ─────────────

    pub fn with_field_violation(mut self, field: &str, description: &str) -> Self {
        self.details.add_bad_request_violation(field, description);
        self
    }

    // ── Resource info ───────────────────────

    pub fn with_resource(mut self, resource_type: &str, resource_name: &str) -> Self {
        self.details
            .set_resource_info(resource_type, resource_name, "", "");
        self
    }

    // ── Retry info ──────────────────────────

    pub fn with_retry_delay(mut self, seconds: u64) -> Self {
        self.details
            .set_retry_info(Some(std::time::Duration::from_secs(seconds)));
        self
    }

    // ── Quota ───────────────────────────────

    pub fn with_quota_violation(mut self, subject: &str, description: &str) -> Self {
        self.details
            .add_quota_failure_violation(subject, description);
        self
    }

    // ── Help links ──────────────────────────

    pub fn with_help(mut self, description: &str, url: &str) -> Self {
        self.details.add_help_link(description, url);
        self
    }

    // ── Finalize ────────────────────────────

    pub fn build(self) -> Status {
        Status::with_error_details(self.code, self.message, self.details)
    }
}
