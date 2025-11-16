use crate::{label::LabelRow, shared::postgresql::Postgresql};

use futures_util::TryStreamExt;
use sqlx::{prelude::FromRow, types::chrono, QueryBuilder};
use todoroki_domain::{
    entities::{
        label::Label,
        todo::{
            Todo, TodoDescription, TodoId, TodoName, TodoPublishment, TodoUpdateCommand,
            TodoUpdateProgressStatus,
        },
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
    labels: serde_json::Value,
}

struct TodoIdColumn {
    id: Uuid,
}

impl TryFrom<TodoRow> for Todo {
    type Error = TodoRepositoryError;

    fn try_from(value: TodoRow) -> Result<Self, Self::Error> {
        let labels: Vec<Label> = serde_json::from_value::<Vec<LabelRow>>(value.labels)
            .map_err(|e| TodoRepositoryError::InternalError(e.to_string()))?
            .into_iter()
            .map(Label::from)
            .collect();

        Ok(Self::new(
            TodoId::new(value.id),
            TodoName::new(value.name),
            TodoDescription::new(value.description),
            if value.is_public {
                TodoPublishment::Public
            } else {
                TodoPublishment::Private(value.alternative_name)
            },
            labels,
            value.started_at.map(DateTime::new),
            value.scheduled_at.map(DateTime::new),
            value.ended_at.map(DateTime::new),
            DateTime::new(value.created_at),
            DateTime::new(value.updated_at),
            value.deleted_at.map(DateTime::new),
        ))
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
        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| TodoRepositoryError::InternalError(e.to_string()))?;

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
            todo.deadlined_at().clone().map(|t| t.value()),
            todo.ended_at().clone().map(|t| t.value()),
        )
        .fetch_one(&mut *tx)
        .await.map_err(|e| {
            TodoRepositoryError::InternalError(e.to_string())
        })?;

        for label in todo.labels() {
            sqlx::query!(
                r#"INSERT INTO todo_labels (todo_id, label_id) VALUES ($1, $2)"#,
                res.id,
                label.id().clone().value(),
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| TodoRepositoryError::InternalError(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| TodoRepositoryError::InternalError(e.to_string()))?;

        Ok(TodoId::new(res.id))
    }

    async fn update(&self, cmd: TodoUpdateCommand) -> Result<(), TodoRepositoryError> {
        if cmd.is_nothing_todo() {
            return Ok(());
        }

        let res = sqlx::query!(
            r#"
            UPDATE todos
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_public = COALESCE($4, is_public),
                alternative_name = COALESCE($5, alternative_name),
                started_at = COALESCE(started_at, $6),
                ended_at = COALESCE(ended_at, $7),
                scheduled_at = COALESCE($8, scheduled_at)
            WHERE id = $1
            "#,
            cmd.id().clone().value(),
            cmd.name().clone().map(|n| n.value()),
            cmd.description().clone().map(|d| d.value()),
            cmd.is_public()
                .clone()
                .map(|is_public| matches!(is_public, TodoPublishment::Public)),
            cmd.is_public()
                .clone()
                .map(|is_public| match is_public {
                    TodoPublishment::Public => None,
                    TodoPublishment::Private(alt) => alt.clone(),
                })
                .flatten(),
            if matches!(cmd.status(), Some(TodoUpdateProgressStatus::OnProgress)) {
                Some(chrono::Utc::now())
            } else {
                None
            },
            if matches!(cmd.status(), Some(TodoUpdateProgressStatus::Completed)) {
                Some(chrono::Utc::now())
            } else {
                None
            },
            cmd.deadlined_at()
                .clone()
                .map(|opt_t| opt_t.map(|t| t.value()))
                .flatten(),
        )
        .execute(&*self.db)
        .await
        .map_err(|e: sqlx::Error| TodoRepositoryError::InternalError(e.to_string()))?;

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
            todos.deleted_at AS "deleted_at?",
            COALESCE(
                json_agg(
                    json_build_object(
                        'id', l.id,
                        'name', l.name,
                        'description', l.description,
                        'color', l.color,
                        'created_at', l.created_at,
                        'updated_at', l.updated_at,
                        'deleted_at', l.deleted_at
                    )
                ) FILTER (WHERE l.id IS NOT NULL),
                '[]'
            ) AS "labels"
            FROM todos
            LEFT JOIN todo_labels tl ON todos.id = tl.todo_id
            LEFT JOIN labels l ON tl.label_id = l.id
            GROUP BY todos.id
            ORDER BY todos.updated_at DESC"#
        )
        .fetch(&*self.db)
        .and_then(|row| async move {
            Todo::try_from(row).map_err(|e| sqlx::Error::ColumnDecode {
                index: "labels".into(),
                source: Box::new(e),
            })
        })
        .try_collect::<Vec<Todo>>()
        .await
        .map_err(|e: sqlx::Error| TodoRepositoryError::InternalError(e.to_string()))?;

        Ok(res)
    }

    async fn delete_by_id(&self, _id: TodoId) -> Result<(), TodoRepositoryError> {
        todo!()
    }
}
