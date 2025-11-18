use std::collections::HashMap;

use serde::Deserialize;
use todoroki_domain::{
    entities::{
        self,
        todo::{TodoDescription, TodoId, TodoName, TodoPublishment},
    },
    value_objects::{
        datetime::{DateTime, MonthlyTime, Time, WeeklyTime},
        error::ErrorCode,
    },
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoRequest {
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub alternative_name: Option<String>,
    pub scheduled_at: Option<String>,
    pub labels: Vec<TodoLabel>,
    pub schedules: Vec<TodoSchedule>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoLabel {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoSchedule {
    pub interval: TodoScheduleInterval,
    pub starts_at: String,
    pub ends_at: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub enum TodoScheduleInterval {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

impl TryFrom<TodoSchedule> for entities::todo::TodoSchedule {
    type Error = ErrorCode;

    fn try_from(value: TodoSchedule) -> Result<Self, Self::Error> {
        match value.interval {
            TodoScheduleInterval::Once => Ok(entities::todo::TodoSchedule::Once(
                DateTime::try_from(value.starts_at)?,
                DateTime::try_from(value.ends_at)?,
            )),
            TodoScheduleInterval::Daily => Ok(entities::todo::TodoSchedule::Daily(
                Time::try_from(DateTime::try_from(value.starts_at)?)?,
                Time::try_from(DateTime::try_from(value.ends_at)?)?,
            )),
            TodoScheduleInterval::Weekly => Ok(entities::todo::TodoSchedule::Weekly(
                WeeklyTime::try_from(DateTime::try_from(value.starts_at)?)?,
                WeeklyTime::try_from(DateTime::try_from(value.ends_at)?)?,
            )),
            TodoScheduleInterval::Monthly => Ok(entities::todo::TodoSchedule::Monthly(
                MonthlyTime::try_from(DateTime::try_from(value.starts_at)?)?,
                MonthlyTime::try_from(DateTime::try_from(value.ends_at)?)?,
            )),
        }
    }
}

impl TodoRequest {
    pub fn try_into_with_labels(
        self,
        labels: Vec<entities::label::Label>,
    ) -> Result<entities::todo::Todo, ErrorCode> {
        let exists_labels: HashMap<uuid::Uuid, entities::label::Label> = labels
            .into_iter()
            .map(|l| (l.id().clone().value(), l))
            .collect();

        let requested_labels: Vec<entities::label::Label> = self
            .labels
            .into_iter()
            .map(|l| uuid::Uuid::parse_str(&l.id).map_err(|_| ErrorCode::InvalidUuidFormat(l.id)))
            .collect::<Result<Vec<uuid::Uuid>, ErrorCode>>()?
            .into_iter()
            .map(|id| {
                exists_labels
                    .get(&id)
                    .cloned()
                    .ok_or(ErrorCode::LabelNotFound(entities::label::LabelId::new(id)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entities::todo::Todo::generate(
            TodoName::new(self.name),
            TodoDescription::new(self.description),
            if self.is_public {
                TodoPublishment::Public
            } else {
                TodoPublishment::Private(self.alternative_name)
            },
            requested_labels,
            self.schedules
                .into_iter()
                .map(|s| entities::todo::TodoSchedule::try_from(s))
                .collect::<Result<Vec<_>, _>>()?,
            self.scheduled_at
                .map(|t| DateTime::try_from(t))
                .transpose()?,
        ))
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoUpdateCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub alternative_name: Option<String>,
    pub scheduled_at: Option<Option<String>>,
    pub status: Option<TodoUpdateProgressStatus>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub enum TodoUpdateProgressStatus {
    #[serde(rename = "on-progress")]
    OnProgress,
    #[serde(rename = "completed")]
    Completed,
}

impl TodoUpdateCommand {
    pub fn try_into_with_id(
        self,
        id: TodoId,
    ) -> Result<entities::todo::TodoUpdateCommand, ErrorCode> {
        Ok(entities::todo::TodoUpdateCommand::new(
            id,
            self.name.map(TodoName::new),
            self.description.map(TodoDescription::new),
            self.is_public.map(|b| {
                if b {
                    TodoPublishment::Public
                } else {
                    TodoPublishment::Private(self.alternative_name)
                }
            }),
            self.scheduled_at
                .map(|opt_t| opt_t.map(DateTime::try_from).transpose())
                .transpose()?,
            self.status
                .map(entities::todo::TodoUpdateProgressStatus::from),
        ))
    }
}

impl From<TodoUpdateProgressStatus> for entities::todo::TodoUpdateProgressStatus {
    fn from(value: TodoUpdateProgressStatus) -> Self {
        match value {
            TodoUpdateProgressStatus::OnProgress => Self::OnProgress,
            TodoUpdateProgressStatus::Completed => Self::Completed,
        }
    }
}
