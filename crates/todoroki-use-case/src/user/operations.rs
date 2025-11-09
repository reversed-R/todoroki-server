use crate::user::{UserUseCase, UserUseCaseError};

use todoroki_domain::{
    entities::{
        user::{AuthVerifiedUser, User, UserEmail, UserId},
        user_auth::UserAuthToken,
    },
    repositories::{user::UserRepository, user_auth::UserAuthRepository, Repositories},
    value_objects::error::ErrorCode,
};

use serde::{Deserialize, Serialize};

use jsonwebtoken::{decode, decode_header, Algorithm, Validation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserAuthClaims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
}

impl<R: Repositories> UserUseCase<R> {
    pub async fn verify(&self, token: UserAuthToken) -> Result<AuthVerifiedUser, ErrorCode> {
        let header = decode_header(token.clone().value()).map_err(|_| {
            ErrorCode::from(UserUseCaseError::UserAuthTokenVerificationError(
                "Failed to decode jwt header".to_string(),
            ))
        })?;

        let kid = header
            .kid
            .ok_or(UserUseCaseError::UserAuthTokenVerificationError(
                "Failed to decode jwt header".to_string(),
            ))?;

        let key = self
            .repositories
            .user_auth_repository()
            .get_key_by_id(kid.clone())
            .await
            .map_err(|_| ErrorCode::from(UserUseCaseError::UserAuthTokenKeyNotFound(kid)))?;

        let mut validation = Validation::new(Algorithm::RS256);

        validation.validate_exp = true;
        validation.validate_nbf = false;
        validation.set_audience(&[&self.firebase_project_id]);
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            &self.firebase_project_id
        )]);
        validation.sub = None;

        let data = decode::<UserAuthClaims>(token.value(), key.value(), &validation)
            .map_err(|_| {
                UserUseCaseError::UserAuthTokenVerificationError(
                    "Failed to validate JWT".to_string(),
                )
            })
            .map_err(ErrorCode::from)?;

        if data.claims.email_verified {
            return Err(ErrorCode::from(
                UserUseCaseError::UserAuthTokenVerificationError(
                    "Email not verified yet".to_string(),
                ),
            ));
        }

        Ok(AuthVerifiedUser::new(UserEmail::new(data.claims.email)))
    }

    pub async fn create(&self, user: User) -> Result<UserId, ErrorCode> {
        let res = self.repositories.user_repository().create(user).await;

        res.map_err(UserUseCaseError::UserRepositoryError)
            .map_err(|e| e.into())
    }

    pub async fn get_by_id(&self, id: UserId) -> Result<User, ErrorCode> {
        let res = self.repositories.user_repository().get_by_id(id).await;

        res.map_err(UserUseCaseError::UserRepositoryError)
            .map_err(|e| e.into())
    }
}
