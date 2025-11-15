use crate::shared::postgresql::Postgresql;

use futures_util::{StreamExt, TryStreamExt};
use sqlx::{prelude::FromRow, types::chrono};
use todoroki_domain::{
    entities::label::{Label, LabelColor, LabelDescription, LabelId, LabelName},
    repositories::label::{LabelRepository, LabelRepositoryError},
    value_objects::datetime::DateTime,
};
use uuid::Uuid;

#[derive(FromRow)]
struct LabelRow {
    id: Uuid,
    name: String,
    description: String,
    color: Option<i32>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

struct LabelIdColumn {
    id: Uuid,
}

impl From<LabelRow> for Label {
    fn from(value: LabelRow) -> Self {
        Self::new(
            LabelId::new(value.id),
            LabelName::new(value.name),
            LabelDescription::new(value.description),
            value.color.map(|i| color_from_i32(i)),
            DateTime::new(value.created_at),
            DateTime::new(value.updated_at),
        )
    }
}

fn color_into_i32(value: LabelColor) -> i32 {
    ((value.red() as i32) << 16) + ((value.green() as i32) << 8) + (value.blue() as i32)
}

fn color_from_i32(value: i32) -> LabelColor {
    LabelColor::new(
        ((value & 0x00ff0000) >> 16) as u8,
        ((value & 0x0000ff00) >> 8) as u8,
        (value & 0x000000ff) as u8,
    )
}

pub struct PgLabelRepository {
    db: Postgresql,
}

impl PgLabelRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl LabelRepository for PgLabelRepository {
    async fn create(&self, label: Label) -> Result<LabelId, LabelRepositoryError> {
        let res = sqlx::query_as!(
            LabelIdColumn,
            r#"
           INSERT INTO labels (id, name, description, color)
           VALUES ($1, $2, $3, $4)
           RETURNING id
            "#,
            label.id().clone().value(),
            label.name().clone().value(),
            label.description().clone().value(),
            label.color().clone().map(|c: LabelColor| color_into_i32(c))
        )
        .fetch_one(&*self.db)
        .await;

        match res {
            Ok(id_column) => Ok(LabelId::new(id_column.id)),
            Err(e) => match e.as_database_error() {
                Some(e) => Err(LabelRepositoryError::InternalError(e.message().to_string())),
                _ => Err(LabelRepositoryError::InternalError(e.to_string())),
            },
        }
    }

    async fn list(&self) -> Result<Vec<Label>, LabelRepositoryError> {
        let res = sqlx::query_as!(
            LabelRow,
            r#"SELECT
            labels.id AS "id",
            labels.name AS "name",
            labels.description AS "description",
            labels.color AS "color",
            labels.created_at AS "created_at",
            labels.updated_at AS "updated_at",
            labels.deleted_at AS "deleted_at?"
            FROM labels
            ORDER BY labels.updated_at DESC"#
        )
        .fetch(&*self.db)
        .map(|row| Ok(Label::from(row?)))
        .try_collect()
        .await;

        res.map_err(|e: sqlx::Error| LabelRepositoryError::InternalError(e.to_string()))
    }

    async fn delete_by_id(&self, id: LabelId) -> Result<(), LabelRepositoryError> {
        todo!()
    }
}
