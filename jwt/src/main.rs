use jamsrpay_jwt::{JwtDecoder, JwtEncoder, TokenParams, scope};

fn main() {
    let private_key = std::fs::read_to_string("jwt_private.pem").expect("missing jwt_private.pem");
    let public_key = std::fs::read_to_string("jwt_public.pem").expect("missing jwt_public.pem");

    // ── Encoder (auth-service) ───────────────────────────────────────
    let encoder = JwtEncoder::new(
        &private_key,
        "auth-service".into(),
        "api".into(),
        chrono::Duration::minutes(15),
    )
    .expect("failed to create encoder");

    let token = encoder
        .encode(TokenParams {
            sub: "user-uuid-001".into(),
            scope: scope::ACCESS_TOKEN.into(),
            role: "merchant_admin".into(),
            tenant_id: Some("merchant-uuid-001".into()),
            session_id: "session-uuid-001".into(),
            expires_in: None,
        })
        .expect("failed to encode token");

    println!("Token:\n{token}\n");

    // ── Decoder (any microservice) ───────────────────────────────────
    let decoder =
        JwtDecoder::new(&public_key, "auth-service", "api").expect("failed to create decoder");

    let claims = decoder
        .decode_with_scope(&token, scope::ACCESS_TOKEN)
        .expect("failed to decode token");

    println!("Claims:\n{claims:#?}");
}
