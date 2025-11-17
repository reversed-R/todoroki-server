use serde::Serialize;
use todoroki_use_case::doit::dto::DoitDto;
use utoipa::ToSchema;

use crate::models::responses::label::LabelResponse;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DoitResponse {
    pub id: String,
    pub name: String,
    pub is_public: bool,
    pub description: String,
    pub alternative_name: Option<String>,
    pub labels: Vec<LabelResponse>,
    pub deadlined_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
}

impl From<DoitDto> for DoitResponse {
    fn from(value: DoitDto) -> Self {
        Self {
            id: value.id.as_hyphenated().to_string(),
            name: value.name,
            is_public: value.is_public,
            description: value.description,
            alternative_name: value.alternative_name,
            labels: value.labels.into_iter().map(LabelResponse::from).collect(),
            deadlined_at: value.deadlined_at.clone().map(|t| t.value().to_rfc3339()),
            created_at: value.created_at.clone().value().to_rfc3339(),
            updated_at: value.updated_at.clone().value().to_rfc3339(),
            created_by: value.created_by.as_hyphenated().to_string(),
        }
    }
}
