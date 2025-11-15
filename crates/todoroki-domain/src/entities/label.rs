use crate::{
    value_object,
    value_objects::{datetime::DateTime, error::ErrorCode},
};
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, Getters)]
pub struct Label {
    #[getset(get = "pub")]
    id: LabelId,
    #[getset(get = "pub")]
    name: LabelName,
    #[getset(get = "pub")]
    description: LabelDescription,
    #[getset(get = "pub")]
    color: Option<LabelColor>,
    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
}

value_object!(LabelId(Uuid));
value_object!(LabelName(String));
value_object!(LabelDescription(String));

impl LabelId {
    pub(crate) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<String> for LabelId {
    type Error = ErrorCode;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(
            Uuid::parse_str(&value).map_err(|_| ErrorCode::InvalidUuidFormat(value))?,
        ))
    }
}

impl Label {
    pub fn new(
        id: LabelId,
        name: LabelName,
        description: LabelDescription,
        color: Option<LabelColor>,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Self {
        Self {
            id,
            name,
            description,
            color,
            created_at,
            updated_at,
        }
    }

    pub fn generate(
        name: LabelName,
        description: LabelDescription,
        color: Option<LabelColor>,
    ) -> Self {
        Self {
            id: LabelId::generate(),
            name,
            description,
            color,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl LabelColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }
}
