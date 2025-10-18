use thiserror;

use crate::entities::todo::{Todo, TodoId, TodoUpdateCommand};

#[derive(Debug, Clone, thiserror::Error)]
pub enum TodoRepositoryError {
    #[error("Internal Error: {0:?}")]
    InternalError(String),
}

#[allow(async_fn_in_trait)]
pub trait TodoRepository: Send + Sync + 'static {
    async fn create(&self, todo: Todo) -> Result<TodoId, TodoRepositoryError>;

    async fn update(&self, cmd: TodoUpdateCommand) -> Result<(), TodoRepositoryError>;

    // async fn get_by_id(&self, id: TodoId) -> Result<Option<Todo>, TodoRepositoryError>;

    async fn list(&self) -> Result<Vec<Todo>, TodoRepositoryError>;

    async fn delete_by_id(&self, id: TodoId) -> Result<(), TodoRepositoryError>;
}
