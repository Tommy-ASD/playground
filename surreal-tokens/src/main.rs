use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, Surreal};

const EXPIRATION_TIME: i64 = 60 * 60; // 1 hour

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Claims {
    ns: String,
    db: String,
    tk: String,
    exp: i64,
}

pub fn sign_token(claims: Claims, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret)).map_err(|e| e.into())
}

pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret);
    let validation = Validation::new(Algorithm::HS512);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims.into())
}

#[tokio::main]
async fn main() {
    let db = Surreal::new::<Ws>("localhost:1000").await.unwrap();

    let private_key = "1234567890";

    db.use_db("test").await.unwrap();
    db.use_ns("test").await.unwrap();

    let subject = Claims {
        ns: "test".to_string(),
        db: "test".to_string(),
        tk: "my_token".to_string(),
        exp: chrono::Utc::now().timestamp() + EXPIRATION_TIME,
    };

    // Create a JWT
    let jwt = sign_token(subject.clone(), private_key.as_bytes()).expect("Failed to sign JWT");

    // Verify and decode the JWT
    let decoded_claims = verify_token(&jwt, private_key.as_bytes()).expect("Failed to verify JWT");

    // Ensure the subject in the claims matches the original subject
    assert_eq!(decoded_claims, subject);

    println!("JWT: {jwt}");

    db.authenticate(jwt).await.unwrap();
}
