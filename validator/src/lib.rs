use std::{fmt::format, panic::Location, vec};
use tonic::{Code, Status, metadata::MetadataMap};
use tonic_types::{BadRequest, ErrorDetails, FieldViolation, LocalizedMessage, StatusExt};
use validator::Validate;

pub trait ValidateExt: Validate {
    #[track_caller]
    fn validate_schema(&self) -> Result<(), Status> {
        let caller = Location::caller();

        self.validate().map_err(|err| {
            let mut error_details = ErrorDetails::new();

            for (field, errors) in err.field_errors() {
                for error in errors {
                    let code = error.code.as_ref();
                    error_details.add_bad_request_violation(field.to_string(), code.to_string());
                }
            }

            let err_message = err.to_string();

            error_details
                .add_help_link("Api Link", "https://api.jamsrpay.com")
                .set_localized_message("en-US", err_message.to_string());

            let mut metadata = MetadataMap::new();
            metadata.insert(
                "location",
                format!("{}:{}", caller.file(), caller.line())
                    .parse()
                    .unwrap(),
            );

            Status::with_error_details_and_metadata(
                Code::InvalidArgument,
                err_message,
                error_details,
                metadata,
            )
        })
    }
}

impl<T: Validate> ValidateExt for T {}
