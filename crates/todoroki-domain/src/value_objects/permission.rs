use crate::entities::{client::Client, user::UserRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    CreateTodo,
    ReadTodo,
    ReadPrivateTodo, // name や description に private ガードがかけられているものを読めるか
    UpdateTodo,
    DeleteTodo,
    CreateDoit,
    ReadDoit,
    // ReadPrivateDoit(Doit), // name や description に private ガードがかけられているものを読めるか。 Doit の作成者自身である場合はContributorも読める
    // UpdateDoit(Doit), //  Doit の作成者自身である場合はContributorも更新できる
    DeleteDoit,
}

impl Client {
    pub(crate) fn has_permission(&self, permission: Permission) -> bool {
        match self {
            Self::User(u) => match u.role() {
                UserRole::Owner => true,
                UserRole::Contributor => matches!(
                    permission,
                    Permission::ReadTodo | Permission::CreateDoit | Permission::ReadDoit
                ),
            },
            Self::Unregistered(_) => {
                matches!(permission, Permission::ReadTodo | Permission::ReadDoit)
            }
            Self::Unverified => matches!(permission, Permission::ReadTodo | Permission::ReadDoit),
        }
    }
}
