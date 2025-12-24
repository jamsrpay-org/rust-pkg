use std::panic::Location;

use tonic::Status;
use validator::Validate;

pub trait ValidateExt: Validate {
    #[track_caller]
    fn validate_schema(&self) -> Result<(), Status> {
        let caller = Location::caller();

        self.validate().map_err(|err| {
            let message = err
                .field_errors()
                .iter()
                .flat_map(|(field, errs)| {
                    errs.iter().map(move |err| {
                        err.message
                            .as_deref()
                            .map(|m| format!("{field}:{m}"))
                            .unwrap_or_else(|| format!("{field}: invalid"))
                    })
                })
                .collect::<Vec<String>>()
                .join(", ");

            Status::invalid_argument(format!(
                "{} (at {}:{})",
                message,
                caller.file(),
                caller.line()
            ))
        })
    }
}
