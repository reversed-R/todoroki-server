use crate::{
    entities::{label::Label, todo::TodoId, user::UserId},
    value_object,
    value_objects::{datetime::DateTime, error::ErrorCode},
};
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, Getters)]
pub struct Doit {
    #[getset(get = "pub")]
    id: DoitId,
    #[getset(get = "pub")]
    name: DoitName,
    #[getset(get = "pub")]
    description: DoitDescription,
    #[getset(get = "pub")]
    is_public: DoitPublishment,
    #[getset(get = "pub")]
    labels: Vec<Label>,
    #[getset(get = "pub")]
    affects_to: Option<TodoId>,
    #[getset(get = "pub")]
    deadlined_at: Option<DateTime>,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
    #[getset(get = "pub")]
    created_by: UserId,
}

value_object!(DoitId(Uuid));
value_object!(DoitName(String));
value_object!(DoitDescription(String));

impl DoitId {
    pub(crate) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<String> for DoitId {
    type Error = ErrorCode;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(
            Uuid::parse_str(&value).map_err(|_| ErrorCode::InvalidUuidFormat(value))?,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum DoitPublishment {
    Public,
    Private(Option<String>), // alternative name
}

impl Doit {
    pub fn new(
        id: DoitId,
        name: DoitName,
        description: DoitDescription,
        is_public: DoitPublishment,
        labels: Vec<Label>,
        affects_to: Option<TodoId>,
        deadlined_at: Option<DateTime>,
        created_at: DateTime,
        updated_at: DateTime,
        created_by: UserId,
    ) -> Self {
        Self {
            id,
            name,
            description,
            is_public,
            labels,
            affects_to,
            deadlined_at,
            created_at,
            updated_at,
            created_by,
        }
    }

    pub fn generate(
        name: DoitName,
        description: DoitDescription,
        is_public: DoitPublishment,
        labels: Vec<Label>,
        deadlined_at: Option<DateTime>,
        created_by: UserId,
    ) -> Self {
        Self {
            id: DoitId::generate(),
            name,
            description,
            is_public,
            labels,
            affects_to: None,
            deadlined_at,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            created_by,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.affects_to.is_none()
    }
}

// None fieald will not be updated
#[derive(Debug, Clone, Getters)]
pub struct DoitUpdateCommand {
    #[getset(get = "pub")]
    id: DoitId,
    #[getset(get = "pub")]
    name: Option<DoitName>,
    #[getset(get = "pub")]
    description: Option<DoitDescription>,
    #[getset(get = "pub")]
    is_public: Option<DoitPublishment>,
    #[getset(get = "pub")]
    affects_to: Option<TodoId>,
    #[getset(get = "pub")]
    deadlined_at: Option<Option<DateTime>>,
}

impl DoitUpdateCommand {
    pub fn new(
        id: DoitId,
        name: Option<DoitName>,
        description: Option<DoitDescription>,
        is_public: Option<DoitPublishment>,
        affects_to: Option<TodoId>,
        deadlined_at: Option<Option<DateTime>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            is_public,
            affects_to,
            deadlined_at,
        }
    }

    pub fn is_nothing_todo(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.is_public.is_none()
            && self.deadlined_at.is_none()
    }
}
