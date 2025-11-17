use crate::{label::LabelRow, shared::postgresql::Postgresql};

use futures_util::TryStreamExt;
use sqlx::{prelude::FromRow, types::chrono};
use todoroki_domain::{
    entities::{
        doit::{Doit, DoitDescription, DoitId, DoitName, DoitPublishment, DoitUpdateCommand},
        label::Label,
        todo::TodoId,
        user::UserId,
    },
    repositories::doit::{DoitRepository, DoitRepositoryError},
    value_objects::datetime::DateTime,
};
use uuid::Uuid;

#[derive(FromRow)]
struct DoitRow {
    id: Uuid,
    name: String,
    description: String,
    is_public: bool,
    alternative_name: Option<String>,
    affects_to: Option<Uuid>,
    deadlined_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    labels: serde_json::Value,
    created_by: Uuid,
}

struct DoitIdColumn {
    id: Uuid,
}

impl TryFrom<DoitRow> for Doit {
    type Error = DoitRepositoryError;

    fn try_from(value: DoitRow) -> Result<Self, Self::Error> {
        let labels: Vec<Label> = serde_json::from_value::<Vec<LabelRow>>(value.labels)
            .map_err(|e| DoitRepositoryError::InternalError(e.to_string()))?
            .into_iter()
            .map(Label::from)
            .collect();

        Ok(Self::new(
            DoitId::new(value.id),
            DoitName::new(value.name),
            DoitDescription::new(value.description),
            if value.is_public {
                DoitPublishment::Public
            } else {
                DoitPublishment::Private(value.alternative_name)
            },
            labels,
            value.affects_to.map(TodoId::new),
            value.deadlined_at.map(DateTime::new),
            DateTime::new(value.created_at),
            DateTime::new(value.updated_at),
            UserId::new(value.created_by),
        ))
    }
}

pub struct PgDoitRepository {
    db: Postgresql,
}

impl PgDoitRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl DoitRepository for PgDoitRepository {
    async fn create(&self, doit: Doit) -> Result<DoitId, DoitRepositoryError> {
        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| DoitRepositoryError::InternalError(e.to_string()))?;

        let res = sqlx::query_as!(
            DoitIdColumn,
            r#"
           INSERT INTO doits (id, name, description, is_public, alternative_name, affects_to, deadlined_at, created_by)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING id
            "#,
            doit.id().clone().value(),
            doit.name().clone().value(),
            doit.description().clone().value(),
            matches!(doit.is_public(), DoitPublishment::Public),
            match doit.is_public() {
                DoitPublishment::Public => None,
                DoitPublishment::Private(alt) => alt.clone()
            },
            doit.affects_to().clone().map(|id| id.value()),
            doit.deadlined_at().clone().map(|t| t.value()),
            doit.created_by().clone().value()
        )
        .fetch_one(&mut *tx)
        .await.map_err(|e: sqlx::Error| {
            DoitRepositoryError::InternalError(e.to_string())
        })?;

        for label in doit.labels() {
            sqlx::query!(
                r#"INSERT INTO doit_labels (doit_id, label_id) VALUES ($1, $2)"#,
                res.id,
                label.id().clone().value(),
            )
            .execute(&mut *tx)
            .await
            .map_err(|e: sqlx::Error| DoitRepositoryError::InternalError(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| DoitRepositoryError::InternalError(e.to_string()))?;

        Ok(DoitId::new(res.id))
    }

    async fn update(&self, cmd: DoitUpdateCommand) -> Result<(), DoitRepositoryError> {
        if cmd.is_nothing_todo() {
            return Ok(());
        }

        sqlx::query!(
            r#"
            UPDATE doits
            SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_public = COALESCE($4, is_public),
                alternative_name = COALESCE($5, alternative_name),
                affects_to = COALESCE($6, affects_to),
                deadlined_at = COALESCE($7, deadlined_at)
            WHERE id = $1
            "#,
            cmd.id().clone().value(),
            cmd.name().clone().map(|n| n.value()),
            cmd.description().clone().map(|d| d.value()),
            cmd.is_public()
                .clone()
                .map(|is_public| matches!(is_public, DoitPublishment::Public)),
            cmd.is_public()
                .clone()
                .map(|is_public| match is_public {
                    DoitPublishment::Public => None,
                    DoitPublishment::Private(alt) => alt.clone(),
                })
                .flatten(),
            cmd.affects_to().clone().map(|id| id.value()),
            cmd.deadlined_at()
                .clone()
                .map(|opt_t| opt_t.map(|t| t.value()))
                .flatten(),
        )
        .execute(&*self.db)
        .await
        .map_err(|e: sqlx::Error| DoitRepositoryError::InternalError(e.to_string()))?;

        Ok(())
    }

    async fn get_by_id(&self, id: DoitId) -> Result<Option<Doit>, DoitRepositoryError> {
        let res: Result<Option<DoitRow>, sqlx::Error> = sqlx::query_as!(
            DoitRow,
            r#"SELECT
            doits.id AS "id",
            doits.name AS "name",
            doits.description AS "description",
            doits.is_public AS "is_public",
            doits.alternative_name AS "alternative_name",
            doits.affects_to AS "affects_to?",
            doits.deadlined_at AS "deadlined_at?",
            doits.created_at AS "created_at",
            doits.updated_at AS "updated_at",
            doits.deleted_at AS "deleted_at?",
            doits.created_by AS "created_by",
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
            FROM doits
            LEFT JOIN doit_labels tl ON doits.id = tl.doit_id
            LEFT JOIN labels l ON tl.label_id = l.id
            GROUP BY doits.id
            ORDER BY doits.updated_at DESC"#
        )
        .fetch_optional(&*self.db)
        .await;

        res.map_err(|e: sqlx::Error| DoitRepositoryError::InternalError(e.to_string()))?
            .map(Doit::try_from)
            .transpose()
    }

    async fn list(&self) -> Result<Vec<Doit>, DoitRepositoryError> {
        let res = sqlx::query_as!(
            DoitRow,
            r#"SELECT
            doits.id AS "id",
            doits.name AS "name",
            doits.description AS "description",
            doits.is_public AS "is_public",
            doits.alternative_name AS "alternative_name",
            doits.affects_to AS "affects_to?",
            doits.deadlined_at AS "deadlined_at?",
            doits.created_at AS "created_at",
            doits.updated_at AS "updated_at",
            doits.deleted_at AS "deleted_at?",
            doits.created_by AS "created_by",
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
            FROM doits
            LEFT JOIN doit_labels tl ON doits.id = tl.doit_id
            LEFT JOIN labels l ON tl.label_id = l.id
            GROUP BY doits.id
            ORDER BY doits.updated_at DESC"#
        )
        .fetch(&*self.db)
        .and_then(|row| async move {
            Doit::try_from(row).map_err(|e| sqlx::Error::ColumnDecode {
                index: "labels".into(),
                source: Box::new(e),
            })
        })
        .try_collect::<Vec<Doit>>()
        .await
        .map_err(|e: sqlx::Error| DoitRepositoryError::InternalError(e.to_string()))?;

        Ok(res)
    }

    async fn delete_by_id(&self, _id: DoitId) -> Result<(), DoitRepositoryError> {
        todo!()
    }
}
