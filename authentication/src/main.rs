use jamsrpay_authentication::{
    Audience, Issuer, JwtDecoder, JwtEncoder, Role, Scope, StoreId, StoreMemberRole, TokenParams,
};
use std::collections::HashMap;

fn main() {
    let private_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/jwt_private.pem"
    ))
    .or_else(|_| std::fs::read_to_string("jwt_private.pem"))
    .expect("missing jwt_private.pem");

    let public_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/jwt_public.pem"
    ))
    .or_else(|_| std::fs::read_to_string("jwt_public.pem"))
    .expect("missing jwt_public.pem");

    // ── Encoder (auth-service) ───────────────────────────────────────
    let encoder = JwtEncoder::new(
        &private_key,
        Issuer::AuthService,
        Audience::ApiGateway,
        chrono::Duration::minutes(15),
    )
    .expect("failed to create encoder");

    let mut stores = HashMap::new();
    let store_id = StoreId::generate();
    stores.insert(store_id, StoreMemberRole::Owner);

    let token = encoder
        .encode(
            TokenParams::new(
                "user-uuid-001",
                Scope::AccessToken,
                Role::Merchant,
                "session-uuid-001",
            )
            .with_stores(stores),
        )
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

