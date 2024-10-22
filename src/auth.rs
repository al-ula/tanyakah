use eyre::Result;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tracing::error;

static KEY: OnceLock<String> = OnceLock::new();

pub fn init() {
    let key = std::env::var("TANYAKAH_JWT").unwrap_or_else(|_| {
        error!("TANYAKAH_JWT env variable must be set");
        std::process::exit(1)
    });
    KEY.set(key).ok();
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SimpleAuth {
    pub user: String,
    pub board: String,
    pub iat: usize,
    pub exp: usize,
}

impl SimpleAuth {
    pub fn new(user: String, board: String, iat: usize) -> Self {
        Self {
            user,
            board,
            iat,
            exp: usize::MAX,
        }
    }

    pub fn token(&self) -> Result<String> {
        let secret = KEY.get().unwrap();
        let token = encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        Ok(token)
    }

    pub fn from_token(token: &str) -> Result<Self> {
        let secret = KEY.get().unwrap();
        let decoded = jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Default::default(),
        )?;
        Ok(decoded.claims)
    }
}
