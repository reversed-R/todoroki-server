use crate::{value_object, value_objects::error::ErrorCode};

value_object!(DateTime(chrono::DateTime<chrono::Utc>));

impl DateTime {
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }
}

impl TryFrom<String> for DateTime {
    type Error = ErrorCode;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(
            chrono::DateTime::parse_from_rfc3339(&value)
                .map_err(|_| ErrorCode::InvalidDateTimeFormat(value))?
                .with_timezone(&chrono::Utc),
        ))
    }
}
