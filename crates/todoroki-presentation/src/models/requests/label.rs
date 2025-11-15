use serde::Deserialize;
use todoroki_domain::{
    entities::{
        self,
        label::{LabelColor, LabelDescription, LabelName},
    },
    value_objects::error::ErrorCode,
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LabelRequest {
    pub name: String,
    pub description: String,
    pub color: Option<String>,
}

impl TryInto<entities::label::Label> for LabelRequest {
    type Error = ErrorCode;

    fn try_into(self) -> Result<entities::label::Label, Self::Error> {
        Ok(entities::label::Label::generate(
            LabelName::new(self.name),
            LabelDescription::new(self.description),
            self.color.map(|s| color_try_from_str(&s)).transpose()?,
        ))
    }
}

fn color_try_from_str(s: &str) -> Result<LabelColor, ErrorCode> {
    if !s.starts_with('#') {
        return Err(ErrorCode::InvalidColorFormat(s.to_string()));
    }

    let hex = &s[1..];

    if hex.len() != 6 {
        return Err(ErrorCode::InvalidColorFormat(s.to_string()));
    }

    let red = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| ErrorCode::InvalidColorFormat(s.to_string()))?;
    let green = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| ErrorCode::InvalidColorFormat(s.to_string()))?;
    let blue = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| ErrorCode::InvalidColorFormat(s.to_string()))?;

    Ok(LabelColor::new(red, green, blue))
}
