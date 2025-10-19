use crate::shared::postgresql::Postgresql;

use futures_util::{StreamExt, TryStreamExt};
use sqlx::{prelude::FromRow, types::chrono};
use todoroki_domain::{
    entities::todo::{Todo, TodoDescription, TodoId, TodoName, TodoPublishment, TodoUpdateCommand},
    repositories::todo::{TodoRepository, TodoRepositoryError},
    value_objects::datetime::DateTime,
};
use uuid::Uuid;

#[derive(FromRow)]
struct TodoRow {
    id: Uuid,
    name: String,
    description: String,
    is_public: bool,
    alternative_name: Option<String>,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    ended_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

struct TodoIdColumn {
    id: Uuid,
}

impl From<TodoRow> for Todo {
    fn from(value: TodoRow) -> Self {
        Self::new(
            TodoId::new(value.id),
            TodoName::new(value.name),
            TodoDescription::new(value.description),
            if value.is_public {
                TodoPublishment::Public
            } else {
                TodoPublishment::Private(value.alternative_name)
            },
            value.started_at.map(DateTime::new),
            value.scheduled_at.map(DateTime::new),
            value.ended_at.map(DateTime::new),
            DateTime::new(value.created_at),
            DateTime::new(value.updated_at),
            value.deleted_at.map(DateTime::new),
        )
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
           INSERT INTO todos (id, name, description, is_public, alternative_name, started_at, scheduled_at, ended_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING id
            "#,
            todo.id().clone().value(),
            todo.name().clone().value(),
            todo.description().clone().value(),
            matches!(todo.is_public(), TodoPublishment::Public),
            match todo.is_public() {
                TodoPublishment::Public => None,
                TodoPublishment::Private(alt) => alt.clone()
            },
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
        let res = sqlx::query_as!(
            TodoRow,
            r#"SELECT
            todos.id AS "id",
            todos.name AS "name",
            todos.description AS "description",
            todos.is_public AS "is_public",
            todos.alternative_name AS "alternative_name",
            todos.started_at AS "started_at?",
            todos.scheduled_at AS "scheduled_at?",
            todos.ended_at AS "ended_at?",
            todos.created_at AS "created_at",
            todos.updated_at AS "updated_at",
            todos.deleted_at AS "deleted_at?"
            FROM todos
            ORDER BY todos.updated_at DESC"#
        )
        .fetch(&*self.db)
        .map(|row| Ok(Todo::from(row?)))
        .try_collect()
        .await;

        res.map_err(|e: sqlx::Error| TodoRepositoryError::InternalError(e.to_string()))
    }

    async fn delete_by_id(&self, id: TodoId) -> Result<(), TodoRepositoryError> {
        todo!()
    }
}
