use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::entities::{self, todo::TodoPublishment};

use crate::models::responses::label::LabelResponse;

const TODO_PRIVATE_DEFAULT_ALTERNATIVE_NAME: &str = "見せられないよ";
const TODO_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION: &str = "見せられないよ";

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodoResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub labels: Vec<LabelResponse>,
    pub started_at: Option<String>,
    pub deadlined_at: Option<String>,
    pub ended_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&entities::todo::Todo> for TodoResponse {
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
            labels: value
                .labels()
                .into_iter()
                .map(LabelResponse::from)
                .collect(),
            started_at: value.started_at().clone().map(|t| t.value().to_rfc3339()),
            deadlined_at: value.deadlined_at().clone().map(|t| t.value().to_rfc3339()),
            ended_at: value.ended_at().clone().map(|t| t.value().to_rfc3339()),
            created_at: value.created_at().clone().value().to_rfc3339(),
            updated_at: value.updated_at().clone().value().to_rfc3339(),
        }
    }
}
