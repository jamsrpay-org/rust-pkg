use jamsrpay_types::{session_id::SessionId, user_id::UserId};
use jwt::{Claims, JwtDecoder, Role, Scope};
use tonic::{Extensions, Request, Status, metadata::MetadataMap, service::Interceptor};
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// Token Extraction Sources
// ─────────────────────────────────────────────────────────────────────────────

/// Specifies where to extract the authentication token from request metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSource {
    /// Extract from a metadata/header key (e.g. "authorization", "x-authorization").
    /// Automatically strips "Bearer " or "bearer " prefix if present.
    Header(String),
    /// Extract from the "cookie" metadata header by matching the specified cookie name.
    Cookie(String),
}

impl TokenSource {
    /// Standard "authorization" header.
    pub fn authorization() -> Self {
        Self::Header("authorization".to_string())
    }

    /// "x-authorization" header.
    pub fn x_authorization() -> Self {
        Self::Header("x-authorization".to_string())
    }

    /// Custom header name.
    pub fn header(name: impl Into<String>) -> Self {
        Self::Header(name.into())
    }

    /// Cookie by cookie name/key.
    pub fn cookie(key: impl Into<String>) -> Self {
        Self::Cookie(key.into())
    }
}

/// Helper to strip "Bearer " or "bearer " prefix and trim whitespace.
fn extract_bearer_or_raw(val: &str) -> Option<&str> {
    let trimmed = val.trim();
    if let Some(token) = trimmed.strip_prefix("Bearer ") {
        let token = token.trim();
        if !token.is_empty() {
            return Some(token);
        }
    } else if let Some(token) = trimmed.strip_prefix("bearer ") {
        let token = token.trim();
        if !token.is_empty() {
            return Some(token);
        }
    } else if !trimmed.is_empty() {
        return Some(trimmed);
    }
    None
}

/// Helper to parse cookies from raw "cookie" header without allocating a HashMap.
fn extract_from_cookie<'a>(cookie_header: &'a str, target_key: &str) -> Option<&'a str> {
    for pair in cookie_header.split(';') {
        let trimmed = pair.trim();
        if let Some((key, value)) = trimmed.split_once('=')
            && key.trim() == target_key
        {
            let val = value.trim();
            let val = if val.starts_with('"') && val.ends_with('"') && val.len() >= 2 {
                &val[1..val.len() - 1]
            } else {
                val
            };
            return extract_bearer_or_raw(val);
        }
    }
    None
}

// ─────────────────────────────────────────────────────────────────────────────
// Role Requirement
// ─────────────────────────────────────────────────────────────────────────────

/// Role requirement for the interceptor.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum RoleRequirement {
    /// Requires merchant role (`Role::Merchant`).
    Merchant,
    /// Requires staff/admin role (`Role::Staff`).
    Staff,
    /// Allows any valid role (`Role::Merchant` or `Role::Staff`).
    #[default]
    Any,
    /// Explicit expected role.
    Exact(Role),
}

impl RoleRequirement {
    pub fn is_satisfied_by(&self, role: &Role) -> bool {
        match self {
            RoleRequirement::Merchant => *role == Role::Merchant,
            RoleRequirement::Staff => *role == Role::Staff,
            RoleRequirement::Any => true,
            RoleRequirement::Exact(expected) => role == expected,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Error Codes Configuration
// ─────────────────────────────────────────────────────────────────────────────

/// Error codes returned as tonic `Status` error messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthErrorCodes {
    pub missing_token: &'static str,
    pub invalid_token: &'static str,
    pub permission_denied: &'static str,
}

impl AuthErrorCodes {
    pub const fn new(
        missing_token: &'static str,
        invalid_token: &'static str,
        permission_denied: &'static str,
    ) -> Self {
        Self {
            missing_token,
            invalid_token,
            permission_denied,
        }
    }

    pub const fn from_single(code: &'static str) -> Self {
        Self {
            missing_token: code,
            invalid_token: code,
            permission_denied: code,
        }
    }
}

impl From<&'static str> for AuthErrorCodes {
    fn from(code: &'static str) -> Self {
        Self::from_single(code)
    }
}

impl From<(&'static str, &'static str)> for AuthErrorCodes {
    fn from((missing, invalid): (&'static str, &'static str)) -> Self {
        Self {
            missing_token: missing,
            invalid_token: invalid,
            permission_denied: invalid,
        }
    }
}

impl From<(&'static str, &'static str, &'static str)> for AuthErrorCodes {
    fn from((missing, invalid, denied): (&'static str, &'static str, &'static str)) -> Self {
        Self {
            missing_token: missing,
            invalid_token: invalid,
            permission_denied: denied,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Authenticated Context
// ─────────────────────────────────────────────────────────────────────────────

/// Authenticated user/staff identity extracted from JWT claims.
///
/// Uses `id: Uuid` so that it represents either `user_id` (merchant) or `staff_id` (staff).
#[derive(Debug, Clone, PartialEq)]
pub struct AuthedUserContext {
    /// Actor UUID: `user_id` for merchants or `staff_id` for staff members.
    pub id: Uuid,
    /// Login session identifier.
    pub session_id: SessionId,
    /// Authenticated role (`Role::Merchant` or `Role::Staff`).
    pub role: Role,
}

/// Type alias for modern naming.
pub type AuthContext = AuthedUserContext;

impl AuthedUserContext {
    pub fn new(id: Uuid, session_id: SessionId, role: Role) -> Self {
        Self {
            id,
            session_id,
            role,
        }
    }

    /// Converts the inner UUID to strongly-typed `UserId` (for merchant services).
    pub fn user_id(&self) -> UserId {
        UserId::from(self.id)
    }

    /// Returns the inner UUID as staff ID (for staff services).
    pub fn staff_id(&self) -> Uuid {
        self.id
    }

    /// Extracts context from tonic request `Extensions`.
    pub fn from_extensions(ctx: &Extensions, error_code: &'static str) -> Result<Self, Status> {
        ctx.get::<Self>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated(error_code))
    }

    /// Extracts context from a tonic `Request`.
    pub fn from_request<T>(request: &Request<T>, error_code: &'static str) -> Result<Self, Status> {
        Self::from_extensions(request.extensions(), error_code)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AuthInterceptor & Builder
// ─────────────────────────────────────────────────────────────────────────────

/// gRPC interceptor that extracts and validates JWT tokens from metadata headers or cookies,
/// validates roles, and injects [`AuthedUserContext`] and [`Claims`] into request extensions.
#[derive(Clone)]
pub struct AuthInterceptor {
    decoder: JwtDecoder,
    error_codes: AuthErrorCodes,
    sources: Vec<TokenSource>,
    role_requirement: RoleRequirement,
    expected_scope: Option<Scope>,
}

impl AuthInterceptor {
    /// Creates an interceptor for merchant verification.
    ///
    /// Defaults to `RoleRequirement::Merchant`, `Scope::AccessToken`, and `authorization` header.
    pub fn merchant(decoder: JwtDecoder, error_codes: impl Into<AuthErrorCodes>) -> Self {
        Self::builder(decoder, error_codes).for_merchant().build()
    }

    /// Creates an interceptor for staff verification.
    ///
    /// Defaults to `RoleRequirement::Staff`, `Scope::AccessToken`, and `authorization` header.
    pub fn staff(decoder: JwtDecoder, error_codes: impl Into<AuthErrorCodes>) -> Self {
        Self::builder(decoder, error_codes).for_staff().build()
    }

    /// Creates an interceptor accepting any valid role.
    pub fn any(decoder: JwtDecoder, error_codes: impl Into<AuthErrorCodes>) -> Self {
        Self::builder(decoder, error_codes).for_any().build()
    }

    /// Backwards-compatible constructor accepting any role and checking "authorization" header.
    pub fn new(decoder: JwtDecoder, error_code: &'static str) -> Self {
        Self::builder(decoder, error_code).for_any().build()
    }

    /// Creates a flexible interceptor builder.
    pub fn builder(
        decoder: JwtDecoder,
        error_codes: impl Into<AuthErrorCodes>,
    ) -> AuthInterceptorBuilder {
        AuthInterceptorBuilder::new(decoder, error_codes.into())
    }

    /// Extracts the token from request metadata according to configured sources in order.
    pub fn extract_token(&self, metadata: &MetadataMap) -> Result<String, Status> {
        for source in &self.sources {
            match source {
                TokenSource::Header(name) => {
                    if let Some(val) = metadata.get(name).and_then(|v| v.to_str().ok())
                        && let Some(token) = extract_bearer_or_raw(val)
                    {
                        return Ok(token.to_string());
                    }
                }
                TokenSource::Cookie(key) => {
                    if let Some(val) = metadata.get("cookie").and_then(|v| v.to_str().ok())
                        && let Some(token) = extract_from_cookie(val, key)
                    {
                        return Ok(token.to_string());
                    }
                }
            }
        }
        Err(Status::unauthenticated(self.error_codes.missing_token))
    }

    /// Authenticates metadata and returns both [`AuthedUserContext`] and [`Claims`].
    pub fn authenticate(
        &self,
        metadata: &MetadataMap,
    ) -> Result<(AuthedUserContext, Claims), Status> {
        let token = self.extract_token(metadata)?;

        let claims = if let Some(expected_scope) = &self.expected_scope {
            self.decoder
                .decode_with_scope(&token, expected_scope.clone())
                .map_err(|_| Status::unauthenticated(self.error_codes.invalid_token))?
        } else {
            self.decoder
                .decode(&token)
                .map_err(|_| Status::unauthenticated(self.error_codes.invalid_token))?
        };

        if !self.role_requirement.is_satisfied_by(&claims.role) {
            return Err(Status::permission_denied(
                self.error_codes.permission_denied,
            ));
        }

        let id = Uuid::parse_str(&claims.sub)
            .map_err(|_| Status::unauthenticated(self.error_codes.invalid_token))?;
        let session_id = SessionId::parse(&claims.session_id)
            .map_err(|_| Status::unauthenticated(self.error_codes.invalid_token))?;

        let authed_user = AuthedUserContext {
            id,
            session_id,
            role: claims.role.clone(),
        };

        Ok((authed_user, claims))
    }

    /// Backwards-compatible helper returning [`AuthedUserContext`].
    pub fn get_authed_user(&self, metadata: &MetadataMap) -> Result<AuthedUserContext, Status> {
        self.authenticate(metadata).map(|(ctx, _)| ctx)
    }

    /// Validates request metadata and returns [`AuthedUserContext`].
    pub fn validate(&self, metadata: &MetadataMap) -> Result<AuthedUserContext, Status> {
        self.get_authed_user(metadata)
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let (authed_user, claims) = self.authenticate(request.metadata())?;
        request.extensions_mut().insert(authed_user);
        request.extensions_mut().insert(claims);
        Ok(request)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Builder Pattern
// ─────────────────────────────────────────────────────────────────────────────

/// Builder for constructing [`AuthInterceptor`].
pub struct AuthInterceptorBuilder {
    decoder: JwtDecoder,
    error_codes: AuthErrorCodes,
    sources: Vec<TokenSource>,
    role_requirement: RoleRequirement,
    expected_scope: Option<Scope>,
}

impl AuthInterceptorBuilder {
    pub fn new(decoder: JwtDecoder, error_codes: AuthErrorCodes) -> Self {
        Self {
            decoder,
            error_codes,
            sources: Vec::new(),
            role_requirement: RoleRequirement::Any,
            expected_scope: Some(Scope::AccessToken),
        }
    }

    /// Appends a token source.
    pub fn token_source(mut self, source: TokenSource) -> Self {
        self.sources.push(source);
        self
    }

    /// Appends multiple token sources.
    pub fn token_sources(mut self, sources: Vec<TokenSource>) -> Self {
        self.sources.extend(sources);
        self
    }

    /// Appends a metadata header token source (e.g. "authorization", "x-authorization").
    pub fn from_header(mut self, name: impl Into<String>) -> Self {
        self.sources.push(TokenSource::Header(name.into()));
        self
    }

    /// Appends a cookie token source by cookie key (e.g. "access-token").
    pub fn from_cookie(mut self, key: impl Into<String>) -> Self {
        self.sources.push(TokenSource::Cookie(key.into()));
        self
    }

    /// Adds the standard "authorization" header.
    pub fn from_authorization(self) -> Self {
        self.from_header("authorization")
    }

    /// Adds the "x-authorization" header.
    pub fn from_x_authorization(self) -> Self {
        self.from_header("x-authorization")
    }

    /// Adds "authorization" header with cookie fallback.
    pub fn from_authorization_or_cookie(self, cookie_name: impl Into<String>) -> Self {
        self.from_authorization().from_cookie(cookie_name)
    }

    /// Requires merchant role (`Role::Merchant`).
    pub fn for_merchant(mut self) -> Self {
        self.role_requirement = RoleRequirement::Merchant;
        self
    }

    /// Requires staff/admin role (`Role::Staff`).
    pub fn for_staff(mut self) -> Self {
        self.role_requirement = RoleRequirement::Staff;
        self
    }

    /// Allows any valid role.
    pub fn for_any(mut self) -> Self {
        self.role_requirement = RoleRequirement::Any;
        self
    }

    /// Sets explicit role requirement.
    pub fn with_role_requirement(mut self, requirement: RoleRequirement) -> Self {
        self.role_requirement = requirement;
        self
    }

    /// Requires a specific JWT scope (e.g. `Scope::AccessToken`).
    pub fn with_scope(mut self, scope: Scope) -> Self {
        self.expected_scope = Some(scope);
        self
    }

    /// Disables scope check.
    pub fn without_scope_check(mut self) -> Self {
        self.expected_scope = None;
        self
    }

    /// Builds the [`AuthInterceptor`].
    /// Defaults to `TokenSource::authorization()` if no sources were specified.
    pub fn build(mut self) -> AuthInterceptor {
        if self.sources.is_empty() {
            self.sources.push(TokenSource::authorization());
        }
        AuthInterceptor {
            decoder: self.decoder,
            error_codes: self.error_codes,
            sources: self.sources,
            role_requirement: self.role_requirement,
            expected_scope: self.expected_scope,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use jwt::{Audience, Issuer, JwtEncoder, TokenParams};
    use tonic::metadata::MetadataValue;

    const TEST_PUB_KEY: &str = include_str!("../../jwt/jwt_public.pem");
    const TEST_PRIV_KEY: &str = include_str!("../../jwt/jwt_private.pem");

    const ERR_MISSING: &str = "shared.header.missing_authorization";
    const ERR_INVALID: &str = "shared.header.invalid_authorization";
    const ERR_FORBIDDEN: &str = "shared.auth.forbidden";

    fn create_test_decoder() -> JwtDecoder {
        JwtDecoder::new(TEST_PUB_KEY, Issuer::AuthService, Audience::ApiGateway).unwrap()
    }

    fn create_test_encoder() -> JwtEncoder {
        JwtEncoder::new(
            TEST_PRIV_KEY,
            Issuer::AuthService,
            Audience::ApiGateway,
            Duration::days(1),
        )
        .unwrap()
    }

    fn generate_token(sub: &str, role: Role, scope: Scope) -> String {
        let encoder = create_test_encoder();
        let params = TokenParams {
            sub: sub.to_string(),
            scope,
            role,
            session_id: Uuid::new_v4().to_string(),
            expires_in: None,
        };
        encoder.encode(params).unwrap()
    }

    #[test]
    fn test_extract_bearer_from_authorization_header() {
        let decoder = create_test_decoder();
        let interceptor = AuthInterceptor::any(decoder, (ERR_MISSING, ERR_INVALID, ERR_FORBIDDEN));

        let user_id = Uuid::new_v4();
        let token = generate_token(&user_id.to_string(), Role::Merchant, Scope::AccessToken);

        // 1. With "Bearer " prefix
        let mut meta = MetadataMap::new();
        meta.insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", token)).unwrap(),
        );
        let (ctx, claims) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, user_id);
        assert_eq!(ctx.role, Role::Merchant);
        assert_eq!(claims.sub, user_id.to_string());

        // 2. With lowercase "bearer " prefix
        let mut meta = MetadataMap::new();
        meta.insert(
            "authorization",
            MetadataValue::try_from(format!("bearer {}", token)).unwrap(),
        );
        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, user_id);

        // 3. Raw token
        let mut meta = MetadataMap::new();
        meta.insert("authorization", MetadataValue::try_from(&token).unwrap());
        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, user_id);
    }

    #[test]
    fn test_extract_from_x_authorization_header() {
        let decoder = create_test_decoder();
        let interceptor = AuthInterceptor::builder(decoder, (ERR_MISSING, ERR_INVALID))
            .from_x_authorization()
            .build();

        let user_id = Uuid::new_v4();
        let token = generate_token(&user_id.to_string(), Role::Merchant, Scope::AccessToken);

        let mut meta = MetadataMap::new();
        meta.insert(
            "x-authorization",
            MetadataValue::try_from(format!("Bearer {}", token)).unwrap(),
        );

        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, user_id);
    }

    #[test]
    fn test_extract_from_cookie() {
        let decoder = create_test_decoder();
        let interceptor = AuthInterceptor::builder(decoder, (ERR_MISSING, ERR_INVALID))
            .from_cookie("access-token")
            .build();

        let user_id = Uuid::new_v4();
        let token = generate_token(&user_id.to_string(), Role::Merchant, Scope::AccessToken);

        let mut meta = MetadataMap::new();
        meta.insert(
            "cookie",
            MetadataValue::try_from(format!(
                "other_cookie=123; access-token={}; theme=dark",
                token
            ))
            .unwrap(),
        );

        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, user_id);
    }

    #[test]
    fn test_token_source_fallback() {
        let decoder = create_test_decoder();
        // Fallback chain: check authorization header, then cookie "access-token"
        let interceptor = AuthInterceptor::builder(decoder, (ERR_MISSING, ERR_INVALID))
            .from_authorization_or_cookie("access-token")
            .build();

        let user_id = Uuid::new_v4();
        let token = generate_token(&user_id.to_string(), Role::Merchant, Scope::AccessToken);

        // Header missing, present in cookie
        let mut meta = MetadataMap::new();
        meta.insert(
            "cookie",
            MetadataValue::try_from(format!("access-token={}", token)).unwrap(),
        );

        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, user_id);

        // Header present -> takes precedence
        let other_id = Uuid::new_v4();
        let header_token =
            generate_token(&other_id.to_string(), Role::Merchant, Scope::AccessToken);
        meta.insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", header_token)).unwrap(),
        );

        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, other_id);
    }

    #[test]
    fn test_missing_token_returns_unauthenticated() {
        let decoder = create_test_decoder();
        let interceptor = AuthInterceptor::any(decoder, (ERR_MISSING, ERR_INVALID, ERR_FORBIDDEN));

        let meta = MetadataMap::new();
        let err = interceptor.authenticate(&meta).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
        assert_eq!(err.message(), ERR_MISSING);
    }

    #[test]
    fn test_invalid_token_returns_unauthenticated() {
        let decoder = create_test_decoder();
        let interceptor = AuthInterceptor::any(decoder, (ERR_MISSING, ERR_INVALID, ERR_FORBIDDEN));

        let mut meta = MetadataMap::new();
        meta.insert(
            "authorization",
            MetadataValue::try_from("Bearer not.a.valid.jwt").unwrap(),
        );
        let err = interceptor.authenticate(&meta).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
        assert_eq!(err.message(), ERR_INVALID);
    }

    #[test]
    fn test_merchant_verification_enforces_role() {
        let decoder = create_test_decoder();
        let interceptor =
            AuthInterceptor::merchant(decoder, (ERR_MISSING, ERR_INVALID, ERR_FORBIDDEN));

        let merchant_id = Uuid::new_v4();
        let merchant_token =
            generate_token(&merchant_id.to_string(), Role::Merchant, Scope::AccessToken);

        let mut meta = MetadataMap::new();
        meta.insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", merchant_token)).unwrap(),
        );

        // Merchant token succeeds
        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, merchant_id);
        assert_eq!(ctx.role, Role::Merchant);
        assert_eq!(ctx.user_id().into_inner(), merchant_id);

        // Staff token rejected with PermissionDenied
        let staff_id = Uuid::new_v4();
        let staff_token = generate_token(&staff_id.to_string(), Role::Staff, Scope::AccessToken);

        let mut staff_meta = MetadataMap::new();
        staff_meta.insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", staff_token)).unwrap(),
        );

        let err = interceptor.authenticate(&staff_meta).unwrap_err();
        assert_eq!(err.code(), tonic::Code::PermissionDenied);
        assert_eq!(err.message(), ERR_FORBIDDEN);
    }

    #[test]
    fn test_staff_verification_enforces_role() {
        let decoder = create_test_decoder();
        let interceptor =
            AuthInterceptor::staff(decoder, (ERR_MISSING, ERR_INVALID, ERR_FORBIDDEN));

        let staff_id = Uuid::new_v4();
        let staff_token = generate_token(&staff_id.to_string(), Role::Staff, Scope::AccessToken);

        let mut meta = MetadataMap::new();
        meta.insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", staff_token)).unwrap(),
        );

        // Staff token succeeds
        let (ctx, _) = interceptor.authenticate(&meta).unwrap();
        assert_eq!(ctx.id, staff_id);
        assert_eq!(ctx.role, Role::Staff);
        assert_eq!(ctx.staff_id(), staff_id);

        // Merchant token rejected with PermissionDenied
        let merchant_id = Uuid::new_v4();
        let merchant_token =
            generate_token(&merchant_id.to_string(), Role::Merchant, Scope::AccessToken);

        let mut merchant_meta = MetadataMap::new();
        merchant_meta.insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", merchant_token)).unwrap(),
        );

        let err = interceptor.authenticate(&merchant_meta).unwrap_err();
        assert_eq!(err.code(), tonic::Code::PermissionDenied);
        assert_eq!(err.message(), ERR_FORBIDDEN);
    }

    #[test]
    fn test_scope_mismatch_returns_unauthenticated() {
        let decoder = create_test_decoder();
        let interceptor = AuthInterceptor::builder(decoder, (ERR_MISSING, ERR_INVALID))
            .with_scope(Scope::AccessToken)
            .build();

        let user_id = Uuid::new_v4();
        // Generate token with RefreshToken scope instead of AccessToken
        let token = generate_token(&user_id.to_string(), Role::Merchant, Scope::RefreshToken);

        let mut meta = MetadataMap::new();
        meta.insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", token)).unwrap(),
        );

        let err = interceptor.authenticate(&meta).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
        assert_eq!(err.message(), ERR_INVALID);
    }

    #[test]
    fn test_interceptor_injects_extensions() {
        let decoder = create_test_decoder();
        let mut interceptor =
            AuthInterceptor::merchant(decoder, (ERR_MISSING, ERR_INVALID, ERR_FORBIDDEN));

        let user_id = Uuid::new_v4();
        let token = generate_token(&user_id.to_string(), Role::Merchant, Scope::AccessToken);

        let mut req = Request::new(());
        req.metadata_mut().insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", token)).unwrap(),
        );

        let intercepted = interceptor.call(req).unwrap();
        let ext = intercepted.extensions();

        let ctx = ext
            .get::<AuthedUserContext>()
            .expect("missing AuthedUserContext");
        assert_eq!(ctx.id, user_id);
        assert_eq!(ctx.role, Role::Merchant);

        let claims = ext.get::<Claims>().expect("missing Claims");
        assert_eq!(claims.sub, user_id.to_string());
    }
}
