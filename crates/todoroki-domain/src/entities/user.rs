use crate::{value_object, value_objects::datetime::DateTime};
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, Getters)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
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

impl UserId {
    pub(crate) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl User {
    pub fn new(
        id: UserId,
        name: UserName,
        email: UserEmail,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> User {
        User {
            id,
            name,
            email,
            created_at,
            updated_at,
        }
    }

    pub fn generate(name: UserName, email: UserEmail) -> Self {
        Self {
            id: UserId::generate(),
            name,
            email,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
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
