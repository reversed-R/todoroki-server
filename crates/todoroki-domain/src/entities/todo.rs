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

impl Todo {
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
