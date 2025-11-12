use crate::{
    shared::{ConfigProvider, ContextProvider},
    user::{UserUseCase, UserUseCaseError},
};

use todoroki_domain::{
    entities::{
        client::Client,
        user::{User, UserEmail, UserId},
        user_auth::UserAuthToken,
    },
    repositories::{
        user::UserRepository,
        user_auth::{UserAuthRepository, UserAuthRepositoryError},
        Repositories,
    },
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
    pub async fn verify(
        &self,
        token: UserAuthToken,
        config: &impl ConfigProvider,
    ) -> Result<Client, ErrorCode> {
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
            .map_err(|e| match e {
                UserAuthRepositoryError::KeyNotFound(k) => {
                    ErrorCode::from(UserUseCaseError::UserAuthTokenKeyNotFound(k))
                }
                UserAuthRepositoryError::InternalError(e) => {
                    ErrorCode::from(UserUseCaseError::UserAuthTokenVerificationError(e))
                }
            })?;

        let mut validation = Validation::new(Algorithm::RS256);

        validation.validate_exp = true;
        validation.validate_nbf = false;
        validation.set_audience(&[config.firebase_project_id()]);
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            config.firebase_project_id()
        )]);
        validation.sub = None;

        let data = decode::<UserAuthClaims>(token.value(), key.value(), &validation)
            .map_err(|_| {
                UserUseCaseError::UserAuthTokenVerificationError(
                    "Failed to validate JWT".to_string(),
                )
            })
            .map_err(ErrorCode::from)?;

        if !data.claims.email_verified {
            return Err(ErrorCode::from(
                UserUseCaseError::UserAuthTokenVerificationError(
                    "Email not verified yet".to_string(),
                ),
            ));
        }

        let email = UserEmail::new(data.claims.email);

        let opt_user = self
            .repositories
            .user_repository()
            .get_by_email(email.clone())
            .await
            .map_err(UserUseCaseError::UserRepositoryError)
            .map_err(ErrorCode::from)?;

        match opt_user {
            Some(u) => Ok(Client::User(u)),
            None => Ok(Client::Unregistered(email)),
        }
    }

    pub async fn create(
        &self,
        user: User,
        ctx: &impl ContextProvider,
    ) -> Result<UserId, ErrorCode> {
        let res = self.repositories.user_repository().create(user).await;

        res.map_err(UserUseCaseError::UserRepositoryError)
            .map_err(|e| e.into())
    }

    pub async fn get_by_id(
        &self,
        id: UserId,
        ctx: &impl ContextProvider,
    ) -> Result<User, ErrorCode> {
        let res = self
            .repositories
            .user_repository()
            .get_by_id(id.clone())
            .await;

        res.map_err(UserUseCaseError::UserRepositoryError)
            .map_err(ErrorCode::from)?
            .ok_or(UserUseCaseError::UserNotFound(id))
            .map_err(|e| e.into())
    }
}
