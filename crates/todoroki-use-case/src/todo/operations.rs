use crate::{
    shared::ContextProvider,
    todo::{dto::TodoDto, TodoUseCase, TodoUseCaseError},
};

use todoroki_domain::{
    entities::todo::{Todo, TodoId, TodoUpdateCommand},
    repositories::{todo::TodoRepository, Repositories},
    value_objects::{error::ErrorCode, permission::Permission},
};

impl<R: Repositories> TodoUseCase<R> {
    pub async fn create(
        &self,
        todo: Todo,
        ctx: &impl ContextProvider,
    ) -> Result<TodoId, ErrorCode> {
        ctx.client().has_permission(Permission::CreateTodo)?;

        let res = self.repositories.todo_repository().create(todo).await;

        res.map_err(TodoUseCaseError::TodoRepositoryError)
            .map_err(|e| e.into())
    }

    pub async fn list(&self, ctx: &impl ContextProvider) -> Result<Vec<TodoDto>, ErrorCode> {
        ctx.client().has_permission(Permission::ReadTodo)?;

        let res = self.repositories.todo_repository().list().await;

        res.map_err(TodoUseCaseError::TodoRepositoryError)
            .map_err(ErrorCode::from)?
            .into_iter()
            .map(|d| TodoDto::try_from_with_permission(d, ctx.client()))
            .collect()
    }

    pub async fn update(
        &self,
        cmd: TodoUpdateCommand,
        ctx: &impl ContextProvider,
    ) -> Result<(), ErrorCode> {
        ctx.client().has_permission(Permission::UpdateTodo)?;

        let res = self.repositories.todo_repository().update(cmd).await;

        res.map_err(TodoUseCaseError::TodoRepositoryError)
            .map_err(|e| e.into())
    }
}
