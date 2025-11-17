use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::entities::{self, client::Client, todo::TodoPublishment, user::UserRole};

use crate::models::responses::label::LabelResponse;

const TODO_PRIVATE_DEFAULT_ALTERNATIVE_NAME: &str = "見せられないよ";
const TODO_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION: &str = "見せられないよ";

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodoResponse {
    pub id: String,
    pub name: String,
    pub is_public: bool,
    pub description: String,
    pub alternative_name: Option<String>,
    pub labels: Vec<LabelResponse>,
    pub started_at: Option<String>,
    pub deadlined_at: Option<String>,
    pub ended_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl TodoResponse {
    pub fn from_with_ownership(value: entities::todo::Todo, client: &Client) -> Self {
        let is_owner = if let Client::User(u) = client {
            matches!(u.role(), UserRole::Owner)
        } else {
            false
        };

        Self {
            id: value.id().clone().value().as_hyphenated().to_string(),
            name: if is_owner {
                value.name().clone().value()
            } else {
                match value.is_public() {
                    TodoPublishment::Public => value.name().clone().value(),
                    TodoPublishment::Private(alt) => match alt {
                        Some(name) => name.to_string(),
                        None => TODO_PRIVATE_DEFAULT_ALTERNATIVE_NAME.to_string(),
                    },
                }
            },
            is_public: matches!(value.is_public(), TodoPublishment::Public),
            description: if is_owner {
                value.description().clone().value()
            } else if let TodoPublishment::Public = value.is_public() {
                value.description().clone().value()
            } else {
                TODO_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION.to_string()
            },
            alternative_name: if let TodoPublishment::Private(alt) = value.is_public() {
                alt.clone()
            } else {
                None
            },
            labels: value
                .labels()
                .clone()
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
