use crate::{
    value_object,
    value_objects::{
        datetime::{DateTime, MonthlyTime, Time, WeeklyTime},
        error::ErrorCode,
    },
};
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
    is_public: TodoPublishment,
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

impl TryFrom<String> for TodoId {
    type Error = ErrorCode;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(
            Uuid::parse_str(&value).map_err(|_| ErrorCode::InvalidUuidFormat(value))?,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum TodoPublishment {
    Public,
    Private(Option<String>), // alternative name
}

#[derive(Debug, Clone)]
pub enum TodoSchedule {
    // (starts_at, ends_at)
    Once(DateTime, DateTime),
    Daily(Time, Time),
    Weekly(WeeklyTime, WeeklyTime),
    Monthly(MonthlyTime, MonthlyTime),
}

impl Todo {
    pub fn new(
        id: TodoId,
        name: TodoName,
        description: TodoDescription,
        is_public: TodoPublishment,
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
            is_public,
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
        is_public: TodoPublishment,
        scheduled_at: Option<DateTime>,
    ) -> Self {
        Self {
            id: TodoId::generate(),
            name,
            description,
            is_public,
            started_at: None,
            scheduled_at,
            ended_at: None,
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
    is_public: Option<TodoPublishment>,
    #[getset(get = "pub")]
    scheduled_at: Option<Option<DateTime>>,
    #[getset(get = "pub")]
    status: Option<TodoUpdateProgressStatus>,
}

#[derive(Debug, Clone)]
pub enum TodoUpdateProgressStatus {
    OnProgress,
    Completed,
}

impl TodoUpdateCommand {
    pub fn new(
        id: TodoId,
        name: Option<TodoName>,
        description: Option<TodoDescription>,
        is_public: Option<TodoPublishment>,
        scheduled_at: Option<Option<DateTime>>,
        status: Option<TodoUpdateProgressStatus>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            is_public,
            scheduled_at,
            status,
        }
    }
}
