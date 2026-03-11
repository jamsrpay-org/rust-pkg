use tonic::{Status, service::Interceptor};

#[derive(Debug, Clone)]
pub struct StoreInterceptor {}

impl StoreInterceptor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interceptor for StoreInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let store_id = request
            .metadata()
            .get("x-store-id")
            .ok_or_else(|| Status::permission_denied("x-store-id is missing"))?
            .to_str()
            .ok()
            .ok_or_else(|| Status::permission_denied("x-store-id is missing"))?
            .to_string();
        request.extensions_mut().insert(StoreContext::new(store_id));
        Ok(request)
    }
}

#[derive(Debug, Clone)]
pub struct StoreContext {
    pub store_id: String,
}

impl StoreContext {
    pub fn new(store_id: String) -> Self {
        Self { store_id }
    }

    pub fn from_extensions(extensions: &tonic::Extensions) -> Result<Self, Status> {
        extensions
            .get::<Self>()
            .cloned()
            .ok_or_else(|| Status::permission_denied("x-store-id is missing"))
    }
}
