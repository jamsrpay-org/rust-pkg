use tonic::{Extensions, Request, Status, metadata::MetadataMap, service::Interceptor};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthInterceptor {
    secret_key: String,
}

impl AuthInterceptor {
    pub fn new(secret_key: String) -> Self {
        AuthInterceptor { secret_key }
    }

    pub fn get_authed_user(&self, metadata: &MetadataMap) -> Result<AuthedUserContext, Status> {
        let authorization = metadata
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Missing authorization header"))?
            .to_str()
            .ok()
            .ok_or_else(|| Status::unauthenticated("Missing authorization header"))?;
        let token = authorization
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| Status::unauthenticated("Invalid authorization header"))?;
        // let decoded = JWTService::new(&self.secret_key, "access_token")
        //     .decode_token(token)
        //     .map_err(|err| Status::unauthenticated(err.to_string()))?;
        // let user_id = Uuid::parse_str(&decoded.sub)
        //     .map_err(|err| Status::unauthenticated(err.to_string()))?;

        let user_id = uuid::Uuid::nil();
        let authed_user = AuthedUserContext { user_id };

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
    pub user_id: Uuid,
}

impl AuthedUserContext {
    pub fn from_extensions(ctx: &Extensions) -> Result<Self, Status> {
        ctx.get::<Self>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authed user"))
    }
}
