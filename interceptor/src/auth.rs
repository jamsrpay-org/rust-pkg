use grpc::error::GrpcErrorContext;
use jamsrpay_types::{session_id::SessionId, user_id::UserId};
use jwt::JwtDecoder;
use tonic::{Extensions, Request, Status, metadata::MetadataMap, service::Interceptor};

const ERROR_CONTEXT: GrpcErrorContext = GrpcErrorContext::new("interceptor");

#[derive(Clone)]
pub struct AuthInterceptor {
    decoder: JwtDecoder,
    error_code: &'static str,
}

impl AuthInterceptor {
    pub fn new(decoder: JwtDecoder, error_code: &'static str) -> Self {
        AuthInterceptor {
            decoder,
            error_code,
        }
    }

    pub fn get_authed_user(&self, metadata: &MetadataMap) -> Result<AuthedUserContext, Status> {
        let authorization = metadata
            .get("authorization")
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
        let user_id = UserId::parse(&decoded.sub)
            .map_err(|_| ERROR_CONTEXT.unauthenticated(self.error_code).build())?;
        let session_id = SessionId::parse(&decoded.session_id)
            .map_err(|_| ERROR_CONTEXT.unauthenticated(self.error_code).build())?;

        let authed_user = AuthedUserContext {
            user_id,
            session_id,
        };
        Ok(authed_user)
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();
        let authed_user = self.get_authed_user(metadata)?;
        request.extensions_mut().insert(authed_user);
        Ok(request)
    }
}

#[derive(Debug, Clone)]
pub struct AuthedUserContext {
    pub user_id: UserId,
    pub session_id: SessionId,
}

impl AuthedUserContext {
    pub fn from_extensions(ctx: &Extensions, error_code: &'static str) -> Result<Self, Status> {
        ctx.get::<Self>()
            .cloned()
            .ok_or_else(|| ERROR_CONTEXT.unauthenticated(error_code).build())
    }
}
