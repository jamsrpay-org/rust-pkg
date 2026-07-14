use jamsrpay_jwt::{Audience, Issuer, JwtDecoder, JwtEncoder, Role, Scope, TokenParams};

fn main() {
    let private_key = std::fs::read_to_string("jwt_private.pem").expect("missing jwt_private.pem");
    let public_key = std::fs::read_to_string("jwt_public.pem").expect("missing jwt_public.pem");

    // ── Encoder (auth-service) ───────────────────────────────────────
    let encoder = JwtEncoder::new(
        &private_key,
        Issuer::AuthService,
        Audience::ApiGateway,
        chrono::Duration::minutes(15),
    )
    .expect("failed to create encoder");

    let token = encoder
        .encode(TokenParams {
            sub: "user-uuid-001".into(),
            scope: Scope::AccessToken,
            role: Role::Merchant,
            session_id: "session-uuid-001".into(),
            expires_in: None,
        })
        .expect("failed to encode token");

    println!("Token:\n{token}\n");

    // ── Decoder (any microservice) ───────────────────────────────────
    let decoder = JwtDecoder::new(&public_key, Issuer::AuthService, Audience::ApiGateway)
        .expect("failed to create decoder");

    let claims = decoder
        .decode_with_scope(&token, Scope::AccessToken)
        .expect("failed to decode token");

    println!("Claims:\n{claims:#?}");
}
