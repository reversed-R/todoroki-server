use serde::Serialize;
use utoipa::ToSchema;

use todoroki_domain::entities::{self, label::LabelColor};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LabelResponse {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&entities::label::Label> for LabelResponse {
    fn from(value: &entities::label::Label) -> Self {
        Self {
            id: value.id().clone().value().as_hyphenated().to_string(),
            name: value.name().clone().value(),
            description: value.description().clone().value(),
            color: value.color().clone().map(|c| color_into_string(c)),
            created_at: value.created_at().clone().value().to_rfc3339(),
            updated_at: value.updated_at().clone().value().to_rfc3339(),
        }
    }
}

fn color_into_string(color: LabelColor) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        color.red(),
        color.green(),
        color.blue()
    )
}
