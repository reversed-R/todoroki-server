use serde::Deserialize;
use todoroki_domain::{entities, value_objects::error::ErrorCode};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct Todo {
    pub name: String,
    pub description: String,
    pub started_at: Option<String>,
    pub scheduled_at: Option<String>,
    pub ended_at: Option<String>,
}

impl TryInto<entities::todo::Todo> for Todo {
    type Error = ErrorCode;

    fn try_into(self) -> Result<entities::todo::Todo, Self::Error> {
        todo!()
    }
}
