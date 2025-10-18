use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::entities;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub started_at: Option<String>,
    pub scheduled_at: Option<String>,
    pub ended_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<&entities::todo::Todo> for Todo {
    fn from(value: &entities::todo::Todo) -> Self {
        Self {
            id: value.id().clone().value().as_hyphenated().to_string(),
            name: value.name().clone().value(),
            description: value.description().clone().value(),
            started_at: value.started_at().clone().map(|t| t.value().to_rfc3339()),
            scheduled_at: value.scheduled_at().clone().map(|t| t.value().to_rfc3339()),
            ended_at: value.ended_at().clone().map(|t| t.value().to_rfc3339()),
            created_at: value.created_at().clone().value().to_rfc3339(),
            updated_at: value.updated_at().clone().value().to_rfc3339(),
            deleted_at: value.deleted_at().clone().map(|t| t.value().to_rfc3339()),
        }
    }
}
