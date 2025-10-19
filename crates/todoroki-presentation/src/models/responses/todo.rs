use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::entities::{self, todo::TodoPublishment};

const TODO_PRIVATE_DEFAULT_ALTERNATIVE_NAME: &str = "見せられないよ";
const TODO_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION: &str = "見せられないよ";

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
            name: match value.is_public() {
                TodoPublishment::Public => value.name().clone().value(),
                TodoPublishment::Private(alt) => match alt {
                    Some(name) => name.to_string(),
                    None => TODO_PRIVATE_DEFAULT_ALTERNATIVE_NAME.to_string(),
                },
            },
            description: if let TodoPublishment::Public = value.is_public() {
                value.description().clone().value()
            } else {
                TODO_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION.to_string()
            },
            started_at: value.started_at().clone().map(|t| t.value().to_rfc3339()),
            scheduled_at: value.scheduled_at().clone().map(|t| t.value().to_rfc3339()),
            ended_at: value.ended_at().clone().map(|t| t.value().to_rfc3339()),
            created_at: value.created_at().clone().value().to_rfc3339(),
            updated_at: value.updated_at().clone().value().to_rfc3339(),
            deleted_at: value.deleted_at().clone().map(|t| t.value().to_rfc3339()),
        }
    }
}
