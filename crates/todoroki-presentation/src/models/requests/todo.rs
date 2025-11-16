use std::collections::HashMap;

use serde::Deserialize;
use todoroki_domain::{
    entities::{
        self,
        todo::{TodoDescription, TodoId, TodoName, TodoPublishment},
    },
    value_objects::{datetime::DateTime, error::ErrorCode},
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoRequest {
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub alternative_name: Option<String>,
    pub scheduled_at: Option<String>,
    pub labels: Vec<TodoLabel>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoLabel {
    pub id: String,
}

impl TodoRequest {
    pub fn try_into_with_labels(
        self,
        labels: Vec<entities::label::Label>,
    ) -> Result<entities::todo::Todo, ErrorCode> {
        let exists_labels: HashMap<uuid::Uuid, entities::label::Label> = labels
            .into_iter()
            .map(|l| (l.id().clone().value(), l))
            .collect();

        let requested_labels: Vec<entities::label::Label> = self
            .labels
            .into_iter()
            .map(|l| uuid::Uuid::parse_str(&l.id).map_err(|_| ErrorCode::InvalidUuidFormat(l.id)))
            .collect::<Result<Vec<uuid::Uuid>, ErrorCode>>()?
            .into_iter()
            .map(|id| {
                exists_labels
                    .get(&id)
                    .cloned()
                    .ok_or(ErrorCode::LabelNotFound(entities::label::LabelId::new(id)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entities::todo::Todo::generate(
            TodoName::new(self.name),
            TodoDescription::new(self.description),
            if self.is_public {
                TodoPublishment::Public
            } else {
                TodoPublishment::Private(self.alternative_name)
            },
            requested_labels,
            self.scheduled_at
                .map(|t| DateTime::try_from(t))
                .transpose()?,
        ))
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoUpdateCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub alternative_name: Option<String>,
    pub scheduled_at: Option<Option<String>>,
    pub status: Option<TodoUpdateProgressStatus>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub enum TodoUpdateProgressStatus {
    #[serde(rename = "on-progress")]
    OnProgress,
    #[serde(rename = "completed")]
    Completed,
}

impl TodoUpdateCommand {
    pub fn try_into_with_id(
        self,
        id: TodoId,
    ) -> Result<entities::todo::TodoUpdateCommand, ErrorCode> {
        Ok(entities::todo::TodoUpdateCommand::new(
            id,
            self.name.map(TodoName::new),
            self.description.map(TodoDescription::new),
            self.is_public.map(|b| {
                if b {
                    TodoPublishment::Public
                } else {
                    TodoPublishment::Private(self.alternative_name)
                }
            }),
            self.scheduled_at
                .map(|opt_t| opt_t.map(DateTime::try_from).transpose())
                .transpose()?,
            self.status
                .map(entities::todo::TodoUpdateProgressStatus::from),
        ))
    }
}

impl From<TodoUpdateProgressStatus> for entities::todo::TodoUpdateProgressStatus {
    fn from(value: TodoUpdateProgressStatus) -> Self {
        match value {
            TodoUpdateProgressStatus::OnProgress => Self::OnProgress,
            TodoUpdateProgressStatus::Completed => Self::Completed,
        }
    }
}
