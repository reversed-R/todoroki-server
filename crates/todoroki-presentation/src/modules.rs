use std::sync::Arc;

use crate::config::Config;
use todoroki_domain::repositories::Repositories;
use todoroki_infrastructure::shared::{DefaultRepositories, DefaultRepositoriesError};

use thiserror::Error;
use todoroki_use_case::{todo::TodoUseCase, user::UserUseCase};

pub struct Modules<R: Repositories> {
    config: Config,
    repositories: Arc<R>,

    todo_use_case: TodoUseCase<R>,
    user_use_case: UserUseCase<R>,
}

impl<R: Repositories> Modules<R> {
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn repositories(&self) -> &Arc<R> {
        &self.repositories
    }

    pub fn todo_use_case(&self) -> &TodoUseCase<R> {
        &self.todo_use_case
    }

    pub fn user_use_case(&self) -> &UserUseCase<R> {
        &self.user_use_case
    }
}

#[derive(Debug, Error)]
pub enum DefaultModulesError {
    #[error(transparent)]
    DefaultRepositoriesError(#[from] DefaultRepositoriesError),
}

const JWK_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

pub async fn default(config: Config) -> Result<Modules<DefaultRepositories>, DefaultModulesError> {
    let default_repositories = DefaultRepositories::new(config.postgres_url(), JWK_URL).await?;
    let repositories = Arc::new(default_repositories);

    Ok(Modules {
        config,
        repositories: Arc::clone(&repositories),
        todo_use_case: TodoUseCase::new(Arc::clone(&repositories)),
        user_use_case: UserUseCase::new(Arc::clone(&repositories)),
    })
}
