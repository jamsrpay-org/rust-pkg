use grpc::error::GrpcErrorContext;
use jamsrpay_types::store_id::StoreId;
use tonic::{Status, service::Interceptor};

const ERROR_CONTEXT: GrpcErrorContext = GrpcErrorContext::new("interceptor");

#[derive(Debug, Clone)]
pub struct StoreInterceptor {
    error_code: &'static str,
}

impl StoreInterceptor {
    pub fn new(error_code: &'static str) -> Self {
        Self { error_code }
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
            .ok_or_else(|| ERROR_CONTEXT.permission_denied(self.error_code).build())?
            .to_str()
            .ok()
            .ok_or_else(|| ERROR_CONTEXT.permission_denied(self.error_code).build())?
            .to_string();
        let store_id = StoreId::parse(&store_id)
            .map_err(|_| ERROR_CONTEXT.permission_denied(self.error_code).build())?;
        request.extensions_mut().insert(StoreContext::new(store_id));
        Ok(request)
    }
}

#[derive(Debug, Clone)]
pub struct StoreContext {
    pub store_id: StoreId,
}

impl StoreContext {
    pub fn new(store_id: StoreId) -> Self {
        Self { store_id }
    }

    pub fn from_extensions(
        extensions: &tonic::Extensions,
        error_code: &'static str,
    ) -> Result<Self, Status> {
        extensions
            .get::<Self>()
            .cloned()
            .ok_or_else(|| ERROR_CONTEXT.permission_denied(error_code).build())
    }
}
