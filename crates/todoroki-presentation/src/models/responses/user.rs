use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::entities;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&entities::user::User> for User {
    fn from(value: &entities::user::User) -> Self {
        Self {
            id: value.id().clone().value().as_hyphenated().to_string(),
            name: value.name().clone().value(),
            created_at: value.created_at().clone().value().to_rfc3339(),
            updated_at: value.updated_at().clone().value().to_rfc3339(),
        }
    }
}
