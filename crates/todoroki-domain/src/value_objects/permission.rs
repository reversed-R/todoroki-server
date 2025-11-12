use crate::entities::user::ClientRole;

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

impl ClientRole {
    pub(crate) fn has_permission(self, permission: Permission) -> bool {
        match self {
            Self::Owner => true,
            Self::Contributor => matches!(
                permission,
                Permission::ReadTodo | Permission::CreateDoit | Permission::ReadDoit
            ),
            Self::NotVerified => matches!(permission, Permission::ReadTodo | Permission::ReadDoit),
        }
    }
}
