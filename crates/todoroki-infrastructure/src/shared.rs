pub mod postgresql;

use crate::{shared::postgresql::Postgresql, todo::PgTodoRepository};
use postgresql::PostgresqlError;
use todoroki_domain::repositories::Repositories;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultRepositoriesError {
    #[error(transparent)]
    PostgresqlError(#[from] PostgresqlError),
}

pub struct DefaultRepositories {
    pg_todo_repository: PgTodoRepository,
}

impl DefaultRepositories {
    pub async fn new(postgres_url: &str) -> Result<Self, DefaultRepositoriesError> {
        let postgresql = Postgresql::new(postgres_url).await?;

        Ok(Self {
            pg_todo_repository: PgTodoRepository::new(postgresql),
        })
    }
}

impl Repositories for DefaultRepositories {
    type TodoRepositoryImpl = PgTodoRepository;

    fn todo_repository(&self) -> &Self::TodoRepositoryImpl {
        &self.pg_todo_repository
    }
}
