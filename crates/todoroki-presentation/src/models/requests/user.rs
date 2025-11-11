use serde::Deserialize;
use todoroki_domain::{entities, value_objects::error::ErrorCode};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UserRequest {
    pub name: String,
}

impl UserRequest {
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
