use crate::todo::{TodoUseCase, TodoUseCaseError};

use todoroki_domain::{
    entities::todo::{Todo, TodoId},
    repositories::{todo::TodoRepository, Repositories},
    value_objects::error::ErrorCode,
};

impl<R: Repositories> TodoUseCase<R> {
    pub async fn create(&self, todo: Todo) -> Result<TodoId, ErrorCode> {
        let res = self.repositories.todo_repository().create(todo).await;

        res.map_err(TodoUseCaseError::TodoRepositoryError)
            .map_err(|e| e.into())
    }

    pub async fn list(&self) -> Result<Vec<Todo>, ErrorCode> {
        let res = self.repositories.todo_repository().list().await;

        res.map_err(TodoUseCaseError::TodoRepositoryError)
            .map_err(|e| e.into())
    }
}
