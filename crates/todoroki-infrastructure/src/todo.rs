use crate::shared::postgresql::Postgresql;

use todoroki_domain::{
    entities::todo::{Todo, TodoId, TodoUpdateCommand},
    repositories::todo::{TodoRepository, TodoRepositoryError},
};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct TodoRow {
    id: Uuid,
    name: String,
}

struct TodoIdColumn {
    id: Uuid,
}

impl From<TodoRow> for Todo {
    fn from(value: TodoRow) -> Self {
        todo!()
    }
}

pub struct PgTodoRepository {
    db: Postgresql,
}

impl PgTodoRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl TodoRepository for PgTodoRepository {
    async fn create(&self, todo: Todo) -> Result<TodoId, TodoRepositoryError> {
        let res = sqlx::query_as!(
            TodoIdColumn,
            r#"
           INSERT INTO todos (id, name, description, started_at, scheduled_at, ended_at)
           VALUES ($1, $2, $3, $4, $5, $6)
           RETURNING id
            "#,
            todo.id().clone().value(),
            todo.name().clone().value(),
            todo.description().clone().value(),
            todo.started_at().clone().map(|t| t.value()),
            todo.scheduled_at().clone().map(|t| t.value()),
            todo.ended_at().clone().map(|t| t.value()),
        )
        .fetch_one(&*self.db)
        .await;

        match res {
            Ok(id_column) => Ok(TodoId::new(id_column.id)),
            Err(e) => match e.as_database_error() {
                Some(e) => Err(TodoRepositoryError::InternalError(e.message().to_string())),
                _ => Err(TodoRepositoryError::InternalError(e.to_string())),
            },
        }
    }

    async fn update(&self, cmd: TodoUpdateCommand) -> Result<(), TodoRepositoryError> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<Todo>, TodoRepositoryError> {
        todo!()
    }

    async fn delete_by_id(&self, id: TodoId) -> Result<(), TodoRepositoryError> {
        todo!()
    }
}
