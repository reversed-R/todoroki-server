use std::collections::HashMap;

use serde::Deserialize;
use todoroki_domain::{
    entities::{
        self,
        doit::{DoitDescription, DoitId, DoitName, DoitPublishment},
        user::UserId,
    },
    value_objects::{datetime::DateTime, error::ErrorCode},
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DoitRequest {
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub alternative_name: Option<String>,
    pub deadlined_at: Option<String>,
    pub labels: Vec<DoitLabel>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DoitLabel {
    pub id: String,
}

impl DoitRequest {
    pub fn try_into_with_labels_and_created_by(
        self,
        labels: Vec<entities::label::Label>,
        created_by: UserId,
    ) -> Result<entities::doit::Doit, ErrorCode> {
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

        Ok(entities::doit::Doit::generate(
            DoitName::new(self.name),
            DoitDescription::new(self.description),
            if self.is_public {
                DoitPublishment::Public
            } else {
                DoitPublishment::Private(self.alternative_name)
            },
            requested_labels,
            self.deadlined_at
                .map(|t| DateTime::try_from(t))
                .transpose()?,
            created_by,
        ))
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DoitUpdateCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub alternative_name: Option<String>,
    pub deadlined_at: Option<Option<String>>,
}

impl DoitUpdateCommand {
    pub fn try_into_with_id(
        self,
        id: DoitId,
    ) -> Result<entities::doit::DoitUpdateCommand, ErrorCode> {
        Ok(entities::doit::DoitUpdateCommand::new(
            id,
            self.name.map(DoitName::new),
            self.description.map(DoitDescription::new),
            self.is_public.map(|b| {
                if b {
                    DoitPublishment::Public
                } else {
                    DoitPublishment::Private(self.alternative_name)
                }
            }),
            None, // NOTE: affects_to is not updated by PATCH /doits/{doit_id}
            self.deadlined_at
                .map(|opt_t| opt_t.map(DateTime::try_from).transpose())
                .transpose()?,
        ))
    }
}
