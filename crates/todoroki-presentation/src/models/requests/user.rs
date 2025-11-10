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
pub struct User {
    pub name: String,
}

impl User {
    pub fn try_into_with_email(
        self,
        email: entities::user::UserEmail,
    ) -> Result<entities::user::User, ErrorCode> {
        Ok(entities::user::User::generate(
            entities::user::UserName::new(self.name),
            email,
        ))
    }
}
