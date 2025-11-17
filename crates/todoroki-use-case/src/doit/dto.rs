use todoroki_domain::{
    entities::{self, client::ContextedClient, doit::DoitPublishment, label::Label},
    value_objects::{datetime::DateTime, error::ErrorCode, permission::Permission},
};
use uuid::Uuid;

pub struct DoitDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub alternative_name: Option<String>,
    pub labels: Vec<Label>,
    pub affects_to: Option<Uuid>,
    pub deadlined_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: Uuid,
}

const DOIT_PRIVATE_DEFAULT_ALTERNATIVE_NAME: &str = "[見せられないよ]";
const DOIT_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION: &str = "[見せられないよ]";

impl DoitDto {
    pub(crate) fn try_from_with_permission<'a>(
        value: entities::doit::Doit,
        client: ContextedClient<'a>,
    ) -> Result<Self, ErrorCode> {
        let use_alt: bool = if matches!(value.is_public(), DoitPublishment::Public) {
            // NOTE: 公開されているDoitすら閲覧する権限がなければ(そのようなロールは存在しないが)直ちに権限エラーで終了
            client.has_permission(Permission::ReadDoit)?;

            false
        } else {
            // NOTE: 非公開のDoitの閲覧権限がなければ、代わりの名前を使用する
            client
                .has_permission(Permission::ReadPrivateDoit(value.clone()))
                .is_err()
        };

        Ok(Self {
            id: value.id().clone().value(),
            name: if use_alt {
                match value.is_public().clone() {
                    DoitPublishment::Public => value.name().clone().value(),
                    DoitPublishment::Private(alt) => {
                        if let Some(alt) = alt {
                            alt
                        } else {
                            DOIT_PRIVATE_DEFAULT_ALTERNATIVE_NAME.to_string()
                        }
                    }
                }
            } else {
                value.name().clone().value()
            },
            description: if use_alt {
                DOIT_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION.to_string()
            } else {
                value.description().clone().value()
            },
            is_public: matches!(value.is_public(), DoitPublishment::Public),
            alternative_name: if let DoitPublishment::Private(alt) = value.is_public().clone() {
                alt
            } else {
                None
            },
            labels: value.labels().clone(),
            affects_to: value.affects_to().clone().map(|id| id.value()),
            deadlined_at: value.deadlined_at().clone(),
            created_at: value.created_at().clone(),
            updated_at: value.updated_at().clone(),
            created_by: value.created_by().clone().value(),
        })
    }
}
