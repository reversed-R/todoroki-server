use serde::Serialize;
use todoroki_use_case::todo::dto::TodoDto;
use utoipa::ToSchema;

use todoroki_domain::{entities, value_objects::datetime::DateTime};

use crate::models::responses::label::LabelResponse;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodoResponse {
    pub id: String,
    pub name: String,
    pub is_public: bool,
    pub description: String,
    pub alternative_name: Option<String>,
    pub labels: Vec<LabelResponse>,
    pub schedules: Vec<TodoScheduleResponse>,
    pub deadlined_at: Option<String>,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodoScheduleResponse {
    pub interval: TodoScheduleIntervalResponse,
    pub starts_at: String,
    pub ends_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum TodoScheduleIntervalResponse {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

impl From<entities::todo::TodoSchedule> for TodoScheduleResponse {
    fn from(value: entities::todo::TodoSchedule) -> Self {
        match value {
            entities::todo::TodoSchedule::Once(s, e) => Self {
                interval: TodoScheduleIntervalResponse::Once,
                starts_at: s.value().to_rfc3339(),
                ends_at: e.value().to_rfc3339(),
            },
            entities::todo::TodoSchedule::Daily(s, e) => Self {
                interval: TodoScheduleIntervalResponse::Daily,
                starts_at: DateTime::from(s).value().to_rfc3339(),
                ends_at: DateTime::from(e).value().to_rfc3339(),
            },
            entities::todo::TodoSchedule::Weekly(s, e) => Self {
                interval: TodoScheduleIntervalResponse::Weekly,
                starts_at: DateTime::from(s).value().to_rfc3339(),
                ends_at: DateTime::from(e).value().to_rfc3339(),
            },
            entities::todo::TodoSchedule::Monthly(s, e) => Self {
                interval: TodoScheduleIntervalResponse::Monthly,
                starts_at: DateTime::from(s).value().to_rfc3339(),
                ends_at: DateTime::from(e).value().to_rfc3339(),
            },
        }
    }
}

impl From<TodoDto> for TodoResponse {
    fn from(value: TodoDto) -> Self {
        Self {
            id: value.id.as_hyphenated().to_string(),
            name: value.name,
            is_public: value.is_public,
            description: value.description,
            alternative_name: value.alternative_name,
            labels: value.labels.into_iter().map(LabelResponse::from).collect(),
            schedules: value
                .schedules
                .clone()
                .into_iter()
                .map(TodoScheduleResponse::from)
                .collect(),
            started_at: value.started_at.clone().map(|t| t.value().to_rfc3339()),
            ended_at: value.ended_at.clone().map(|t| t.value().to_rfc3339()),
            deadlined_at: value.deadlined_at.clone().map(|t| t.value().to_rfc3339()),
            created_at: value.created_at.clone().value().to_rfc3339(),
            updated_at: value.updated_at.clone().value().to_rfc3339(),
        }
    }
}
