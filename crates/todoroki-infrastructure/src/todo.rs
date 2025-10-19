use crate::shared::postgresql::Postgresql;

use futures_util::{StreamExt, TryFutureExt, TryStreamExt};
use sqlx::{prelude::FromRow, types::chrono, QueryBuilder};
use todoroki_domain::{
    entities::todo::{
        Todo, TodoDescription, TodoId, TodoName, TodoPublishment, TodoUpdateCommand,
        TodoUpdateProgressStatus,
    },
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
        let mut builder = QueryBuilder::new("UPDATE todos SET ");

        let mut sep = builder.separated(", ");

        if let Some(name) = cmd.name() {
            sep.push("name = ");
            sep.push_bind(name.clone().value());
        }

        if let Some(description) = cmd.description() {
            sep.push("description = ");
            sep.push_bind(description.clone().value());
        }

        if let Some(scheduled_at) = cmd.scheduled_at() {
            sep.push("scheduled_at = ");
            sep.push_bind(scheduled_at.clone().map(|t| t.value()));
        }

        // すでに開始済みまたは終了済みの際は列の更新をしない
        if let Some(status) = cmd.status() {
            match status {
                TodoUpdateProgressStatus::OnProgress => {
                    sep.push("started_at = CASE WHEN started_at IS NULL THEN ");
                    sep.push_bind(chrono::Utc::now());
                    sep.push(" ELSE started_at END");
                }
                TodoUpdateProgressStatus::Completed => {
                    sep.push("ended_at = CASE WHEN ended_at IS NULL THEN ");
                    sep.push_bind(chrono::Utc::now());
                    sep.push(" ELSE ended_at END");
                }
            }
        }

        builder.push(" WHERE id = ");
        builder.push_bind(cmd.id().clone().value());

        builder
            .build()
            .execute(&*self.db)
            .await
            .map_err(|e| TodoRepositoryError::InternalError(e.to_string()))?;

        Ok(())
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
