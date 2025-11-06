use crate::user::{UserUseCase, UserUseCaseError};

use todoroki_domain::{
    entities::{
        user::{User, UserId},
        user_auth::UserAuthToken,
    },
    repositories::{user::UserRepository, user_auth::UserAuthRepository, Repositories},
    value_objects::error::ErrorCode,
};

use jsonwebtoken::{
    decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, TokenData, Validation,
};

impl<R: Repositories> UserUseCase<R> {
    pub async fn verify(&self, token: UserAuthToken) -> Result<(), ErrorCode> {
        let header = decode_header(token)?;
        let kid = header.kid.context("No key ID found in JWT header")?;

        let key = self.repositories.user_auth_repository().get_key_by_id(kid);

        let mut validation = Validation::new(Algorithm::RS256);

        validation.validate_exp = true;
        validation.validate_nbf = false;
        validation.set_audience(&[&firebase_project_id]);
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            &firebase_project_id
        )]);
        validation.sub = None;

        let data = decode(token, &key, &validation).context("Failed to validate JWT")?;

        Ok(data)
    }

    pub async fn create(&self, user: User) -> Result<UserId, ErrorCode> {
        let res = self.repositories.user_repository().create(User).await;

        res.map_err(UserUseCaseError::UserRepositoryError)
            .map_err(|e| e.into())
    }

    pub async fn get_by_id(&self, id: UserId) -> Result<(), ErrorCode> {
        let res = self.repositories.user_repository().get_by_id(id).await;

        res.map_err(UserUseCaseError::UserRepositoryError)
            .map_err(|e| e.into())
    }
}
