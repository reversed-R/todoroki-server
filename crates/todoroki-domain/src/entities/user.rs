use crate::{
    value_object,
    value_objects::{datetime::DateTime, permission::Permission},
};
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, Getters)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub")]
    role: UserRole,
    #[getset(get = "pub")]
    name: UserName,
    #[getset(get = "pub")]
    email: UserEmail,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
}

value_object!(UserId(Uuid));
value_object!(UserName(String));
value_object!(UserEmail(String));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Owner,
    Contributor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientRole {
    Owner,
    Contributor,
    NotVerified,
}

impl From<UserRole> for ClientRole {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Owner => Self::Owner,
            UserRole::Contributor => Self::Contributor,
        }
    }
}

impl UserId {
    pub(crate) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl User {
    pub fn new(
        id: UserId,
        role: UserRole,
        name: UserName,
        email: UserEmail,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> User {
        User {
            id,
            role,
            name,
            email,
            created_at,
            updated_at,
        }
    }

    pub fn generate(name: UserName, email: UserEmail) -> Self {
        Self {
            id: UserId::generate(),
            role: UserRole::Contributor,
            name,
            email,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        let client_role = ClientRole::from(self.role);

        client_role.has_permission(permission)
    }
}

#[derive(Debug, Clone, Getters)]
pub struct AuthVerifiedUser {
    #[getset(get = "pub")]
    email: UserEmail,
}

impl AuthVerifiedUser {
    pub fn new(email: UserEmail) -> Self {
        Self { email }
    }
}
