use grpc::error::GrpcErrorContext;
use jwt::JwtDecoder;
use tonic::{Status, metadata::MetadataMap};
use uuid::Uuid;

const ERROR_CONTEXT: GrpcErrorContext = GrpcErrorContext::new("interceptor");

#[derive(Clone)]
pub struct RootAuthInterceptor {
    decoder: JwtDecoder,
    error_code: &'static str,
}

impl RootAuthInterceptor {
    pub fn new(decoder: JwtDecoder, error_code: &'static str) -> Self {
        Self {
            decoder,
            error_code,
        }
    }

    pub fn validate(&self, metadata: &MetadataMap) -> Result<Uuid, Status> {
        let authorization = metadata
            .get("x-root-auth")
            .ok_or_else(|| ERROR_CONTEXT.unauthenticated(self.error_code).build())?
            .to_str()
            .ok()
            .ok_or_else(|| ERROR_CONTEXT.unauthenticated(self.error_code).build())?;
        let token = authorization
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| ERROR_CONTEXT.unauthenticated(self.error_code).build())?;
        let decoded = self
            .decoder
            .decode(token)
            .map_err(|_| ERROR_CONTEXT.unauthenticated(self.error_code).build())?;
        let user_id = Uuid::parse_str(&decoded.sub)
            .map_err(|_| ERROR_CONTEXT.unauthenticated(self.error_code).build())?;

        Ok(user_id)
    }
}
