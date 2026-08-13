use std::collections::HashMap;
use tonic::{Code, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub struct ErrorBuilder {
    pub code: Code,
    pub message: String,
    pub details: ErrorDetails,
    /// Extra metadata to merge into ErrorInfo at build time.
    extra_metadata: HashMap<String, String>,
}

impl ErrorBuilder {
    /// Create a new ErrorBuilder (used by GrpcErrorContext).
    pub(crate) fn new(code: Code, message: String, details: ErrorDetails) -> Self {
        Self {
            code,
            message,
            details,
            extra_metadata: HashMap::new(),
        }
    }

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

    // ── Custom metadata ─────────────────────

    /// Append a custom key-value pair to the ErrorInfo metadata map.
    ///
    /// This is useful for passing structured data alongside the error,
    /// e.g. a `challenge_token` for `SecurityChallengeRequired`.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.extra_metadata
            .insert(key.to_string(), value.to_string());
        self
    }

    // ── Finalize ────────────────────────────

    pub fn build(mut self) -> Status {
        // Merge extra metadata into ErrorInfo if any was added.
        if !self.extra_metadata.is_empty() {
            if let Some(error_info) = self.details.error_info() {
                let mut merged_metadata = error_info.metadata.clone();
                merged_metadata.extend(self.extra_metadata);
                self.details.set_error_info(
                    &error_info.reason.clone(),
                    &error_info.domain.clone(),
                    merged_metadata,
                );
            }
        }
        Status::with_error_details(self.code, self.message, self.details)
    }
}
