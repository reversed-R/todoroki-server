use crate::{value_object, value_objects::datetime::DateTime};
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, Getters)]
pub struct Todo {
    #[getset(get = "pub")]
    id: TodoId,
    #[getset(get = "pub")]
    name: TodoName,
    #[getset(get = "pub")]
    description: TodoDescription,
    #[getset(get = "pub")]
    started_at: Option<DateTime>,
    #[getset(get = "pub")]
    scheduled_at: Option<DateTime>,
    #[getset(get = "pub")]
    ended_at: Option<DateTime>,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
    #[getset(get = "pub")]
    deleted_at: Option<DateTime>,
}

value_object!(TodoId(Uuid));
value_object!(TodoName(String));
value_object!(TodoDescription(String));

impl TodoId {
    pub(crate) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Todo {
    pub fn new(
        id: TodoId,
        name: TodoName,
        description: TodoDescription,
        started_at: Option<DateTime>,
        scheduled_at: Option<DateTime>,
        ended_at: Option<DateTime>,
        created_at: DateTime,
        updated_at: DateTime,
        deleted_at: Option<DateTime>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            started_at,
            scheduled_at,
            ended_at,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn generate(
        name: TodoName,
        description: TodoDescription,
        started_at: Option<DateTime>,
        scheduled_at: Option<DateTime>,
        ended_at: Option<DateTime>,
    ) -> Self {
        Self {
            id: TodoId::generate(),
            name,
            description,
            started_at,
            scheduled_at,
            ended_at,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            deleted_at: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.deleted_at.is_none() && self.ended_at.is_none()
    }
}

// None fieald will not be updated
#[derive(Debug, Clone, Getters)]
pub struct TodoUpdateCommand {
    #[getset(get = "pub")]
    id: TodoId,
    #[getset(get = "pub")]
    name: Option<TodoName>,
    #[getset(get = "pub")]
    description: Option<TodoDescription>,
    #[getset(get = "pub")]
    starts_at: Option<Option<DateTime>>,
    #[getset(get = "pub")]
    scheduled_at: Option<Option<DateTime>>,
    #[getset(get = "pub")]
    ends_at: Option<Option<DateTime>>,
}
