use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::entities;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub role: UserRoleResponse,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
pub enum UserRoleResponse {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "contributor")]
    Contributor,
}

impl From<entities::user::UserRole> for UserRoleResponse {
    fn from(value: entities::user::UserRole) -> Self {
        match value {
            entities::user::UserRole::Owner => Self::Owner,
            entities::user::UserRole::Contributor => Self::Contributor,
        }
    }
}

impl From<entities::user::User> for UserResponse {
    fn from(value: entities::user::User) -> Self {
        Self {
            id: value.id().clone().value().as_hyphenated().to_string(),
            name: value.name().clone().value(),
            role: UserRoleResponse::from(value.role().clone()),
            created_at: value.created_at().clone().value().to_rfc3339(),
            updated_at: value.updated_at().clone().value().to_rfc3339(),
        }
    }
}
