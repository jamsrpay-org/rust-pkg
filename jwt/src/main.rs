use jamsrpay_jwt::{JwtDecoder, JwtEncoder};

fn main() {
    let private_key = std::fs::read_to_string("jwt_private.pem")
        .expect("missing private key file: jwt_private.pem");
    let public_key =
        std::fs::read_to_string("jwt_public.pem").expect("missing public key file: jwt_public.pem");

    let encoder = JwtEncoder::new(private_key);

    let token = encoder
        .create_token("user-id".to_string(), "Authorization".to_string(), None)
        .unwrap();

    let decoder = JwtDecoder::new(public_key);
    let claims = decoder
        .decode_token(&token, "Authorization", None, None)
        .unwrap();

    println!("{token}");
    println!("{claims:?}");
}
