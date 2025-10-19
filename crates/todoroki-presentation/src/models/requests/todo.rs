use serde::Deserialize;
use todoroki_domain::{
    entities::{
        self,
        todo::{TodoDescription, TodoName, TodoPublishment},
    },
    value_objects::{datetime::DateTime, error::ErrorCode},
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct Todo {
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub alternative_name: Option<String>,
    pub started_at: Option<String>,
    pub scheduled_at: Option<String>,
    pub ended_at: Option<String>,
}

impl TryInto<entities::todo::Todo> for Todo {
    type Error = ErrorCode;

    fn try_into(self) -> Result<entities::todo::Todo, Self::Error> {
        Ok(entities::todo::Todo::generate(
            TodoName::new(self.name),
            TodoDescription::new(self.description),
            if self.is_public {
                TodoPublishment::Public
            } else {
                TodoPublishment::Private(self.alternative_name)
            },
            self.started_at.map(|t| DateTime::try_from(t)).transpose()?,
            self.scheduled_at
                .map(|t| DateTime::try_from(t))
                .transpose()?,
            self.ended_at.map(|t| DateTime::try_from(t)).transpose()?,
        ))
    }
}
