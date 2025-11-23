use todoroki_domain::{
    entities::user_auth::VerificationKey,
    repositories::user_auth::{UserAuthRepository, UserAuthRepositoryError},
};

use jsonwebtoken::{jwk::JwkSet, DecodingKey};
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
            .map_err(|_| {
                UserAuthRepositoryError::InternalError("Failed to create http client".to_string())
            })?;

        tracing::info!("fetching jwks...; jwk_url={}", self.jwk_url);

        let jwks: JwkSet = client
            .get(&self.jwk_url)
            .send()
            .await
            .map_err(|e| {
                UserAuthRepositoryError::InternalError(format!(
                    "Failed to get JWKS; error={}",
                    e.to_string()
                ))
            })?
            .json()
            .await
            .map_err(|e| {
                UserAuthRepositoryError::InternalError(format!(
                    "Failed to deserialize JWKS; error={}",
                    e.to_string()
                ))
            })?;

        let jwk = jwks
            .find(&id)
            .ok_or(UserAuthRepositoryError::KeyNotFound(id))?;

        let key = DecodingKey::from_jwk(jwk).map_err(|_| {
            UserAuthRepositoryError::InternalError("Failed to get key from jwk".to_string())
        })?;

        Ok(VerificationKey::new(key))
    }
}
