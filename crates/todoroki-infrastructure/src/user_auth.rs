use crate::shared::postgresql::Postgresql;

use todoroki_domain::{
    entities::user_auth::VerificationKey,
    repositories::user_auth::{UserAuthRepository, UserAuthRepositoryError},
};

use jsonwebtoken::{
    decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, TokenData, Validation,
};
use reqwest::ClientBuilder;
use std::time::Duration;

pub struct FirebaseUserAuthRepository {
    jwk_url: String,
}

impl FirebaseUserAuthRepository {
    pub fn new(jwk_url: String) -> Self {
        Self { jwk_url }
    }
}

impl UserAuthRepository for FirebaseUserAuthRepository {
    async fn get_key_by_id(&self, id: String) -> Result<VerificationKey, UserAuthRepositoryError> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(UserAuthRepositoryError::InternalError(
                "Failed to create http client".to_string(),
            ))?;

        let jwks: JwkSet = client.get(self.jwk_url).send().await?.json().await?;

        let jwk = jwks
            .find(&id)
            .ok_or(UserAuthRepositoryError::InternalError(
                "Jwk not found".to_string(),
            ))?;

        let key = DecodingKey::from_jwk(jwk).map_err(UserAuthRepositoryError::InternalError(
            "Failed to get key from jwk".to_string(),
        ))?;

        Ok(VerificationKey::new(key))
    }
}
