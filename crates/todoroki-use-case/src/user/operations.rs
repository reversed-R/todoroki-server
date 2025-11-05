use crate::user::{UserUseCase, UserUseCaseError};

use todoroki_domain::{
    entities::user::{User, UserId},
    repositories::{user::UserRepository, Repositories},
    value_objects::error::ErrorCode,
};

impl<R: Repositories> UserUseCase<R> {
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
