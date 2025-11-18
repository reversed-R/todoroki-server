use todoroki_domain::{
    entities::{
        self,
        client::ContextedClient,
        label::Label,
        todo::{TodoPublishment, TodoSchedule},
    },
    value_objects::{datetime::DateTime, error::ErrorCode, permission::Permission},
};
use uuid::Uuid;

const TODO_PRIVATE_DEFAULT_ALTERNATIVE_NAME: &str = "[見せられないよ]";
const TODO_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION: &str = "[見せられないよ]";

#[derive(Debug, Clone)]
pub struct TodoDto {
    pub id: Uuid,
    pub name: String,
    pub is_public: bool,
    pub description: String,
    pub alternative_name: Option<String>,
    pub labels: Vec<Label>,
    pub schedules: Vec<TodoSchedule>,
    pub deadlined_at: Option<DateTime>,
    pub started_at: Option<DateTime>,
    pub ended_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl TodoDto {
    pub(crate) fn try_from_with_permission<'a>(
        value: entities::todo::Todo,
        client: ContextedClient<'a>,
    ) -> Result<Self, ErrorCode> {
        let use_alt: bool = if matches!(value.is_public(), TodoPublishment::Public) {
            // NOTE: 公開されているTodoすら閲覧する権限がなければ(そのようなロールは存在しないが)直ちに権限エラーで終了
            client.has_permission(Permission::ReadTodo)?;

            false
        } else {
            // NOTE: 非公開のTodoの閲覧権限がなければ、代わりの名前を使用する
            client.has_permission(Permission::ReadPrivateTodo).is_err()
        };

        Ok(Self {
            id: value.id().clone().value(),
            name: if use_alt {
                match value.is_public().clone() {
                    TodoPublishment::Public => value.name().clone().value(),
                    TodoPublishment::Private(alt) => {
                        if let Some(alt) = alt {
                            alt
                        } else {
                            TODO_PRIVATE_DEFAULT_ALTERNATIVE_NAME.to_string()
                        }
                    }
                }
            } else {
                value.name().clone().value()
            },
            description: if use_alt {
                TODO_PRIVATE_DEFAULT_ALTERNATIVE_DESCRIPTION.to_string()
            } else {
                value.description().clone().value()
            },
            is_public: matches!(value.is_public(), TodoPublishment::Public),
            alternative_name: if let TodoPublishment::Private(alt) = value.is_public() {
                alt.clone()
            } else {
                None
            },
            labels: value.labels().clone(),
            schedules: value.schedules().clone(),
            started_at: value.started_at().clone(),
            deadlined_at: value.deadlined_at().clone(),
            ended_at: value.ended_at().clone(),
            created_at: value.created_at().clone(),
            updated_at: value.updated_at().clone(),
        })
    }
}
