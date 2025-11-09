pub mod postgresql;

use crate::{
    shared::postgresql::Postgresql, todo::PgTodoRepository, user::PgUserRepository,
    user_auth::FirebaseUserAuthRepository,
};
use postgresql::PostgresqlError;
use todoroki_domain::repositories::Repositories;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultRepositoriesError {
    #[error(transparent)]
    PostgresqlError(#[from] PostgresqlError),
}

pub struct DefaultRepositories {
    todo_repository: PgTodoRepository,
    user_repository: PgUserRepository,
    user_auth_repository: FirebaseUserAuthRepository,
}

impl DefaultRepositories {
    pub async fn new(postgres_url: &str, jwk_url: &str) -> Result<Self, DefaultRepositoriesError> {
        let postgresql: Postgresql = Postgresql::new(postgres_url).await?;

        Ok(Self {
            todo_repository: PgTodoRepository::new(postgresql.clone()),
            user_repository: PgUserRepository::new(postgresql),
            user_auth_repository: FirebaseUserAuthRepository::new(jwk_url.to_string()),
        })
    }
}

impl Repositories for DefaultRepositories {
    type TodoRepositoryImpl = PgTodoRepository;
    type UserRepositoryImpl = PgUserRepository;
    type UserAuthRepositoryImpl = FirebaseUserAuthRepository;

    fn todo_repository(&self) -> &Self::TodoRepositoryImpl {
        &self.todo_repository
    }

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.user_repository
    }

    fn user_auth_repository(&self) -> &Self::UserAuthRepositoryImpl {
        &self.user_auth_repository
    }
}
