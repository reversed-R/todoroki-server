use std::fmt::Display;

use crate::{
    entities::{
        client::{Client, ContextedClient},
        doit::Doit,
        user::{User, UserRole},
    },
    value_objects::error::ErrorCode,
};

#[derive(Debug, Clone)]
pub enum Permission {
    CreateUser(User),
    ReadUser,
    // UpdateUser(User),
    // DeleteUser(User),
    CreateTodo,
    ReadTodo,
    ReadPrivateTodo, // name や description に private ガードがかけられているものを読めるか
    UpdateTodo,
    DeleteTodo,
    CreateDoit,
    ReadDoit,
    ReadPrivateDoit(Doit), // name や description に private ガードがかけられているものを読めるか。 Doit の作成者自身である場合はContributorも読める
    UpdateDoit(Doit),      //  Doit の作成者自身である場合はContributorも更新できる
    DeleteDoit,
    CreateLabel,
    ReadLabel,
    UpdateLabel,
    DeleteLabel,
}

impl<'a> ContextedClient<'a> {
    pub fn has_permission(&self, permission: Permission) -> Result<(), ErrorCode> {
        let has = match self.client() {
            Client::User(u) => match u.role() {
                UserRole::Owner => true,
                UserRole::Contributor => matches!(
                    permission,
                    Permission::ReadTodo
                        | Permission::CreateDoit
                        | Permission::ReadDoit
                        | Permission::ReadLabel
                ),
            },
            Client::Unregistered(email) => {
                matches!(
                    permission,
                    Permission::ReadTodo | Permission::ReadDoit | Permission::ReadLabel
                ) || if let Permission::CreateUser(u) = permission.clone() {
                    (u.role() == &UserRole::Contributor
                        || (u.email().clone().value()
                            == self.default_owner_email().to_owned().to_owned().value())
                            && u.role() == &UserRole::Owner)
                        && u.email().clone().value() == email.clone().value()
                } else {
                    false
                }
            }
            Client::Unverified => matches!(
                permission,
                Permission::ReadTodo | Permission::ReadDoit | Permission::ReadLabel
            ),
        };

        if has {
            Ok(())
        } else {
            Err(ErrorCode::PermissionDenied(Box::new(permission)))
        }
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateTodo => write!(f, "create-todo"),
            Self::ReadTodo => write!(f, "read-todo"),
            Self::ReadPrivateTodo => write!(f, "read-private-todo"),
            Self::UpdateTodo => write!(f, "update-todo"),
            Self::DeleteTodo => write!(f, "delete-todo"),
            Self::CreateDoit => write!(f, "create-doit"),
            Self::ReadDoit => write!(f, "read-doit"),
            Self::ReadPrivateDoit(_) => write!(f, "read-private-doit"),
            Self::UpdateDoit(_) => write!(f, "update-doit"),
            Self::DeleteDoit => write!(f, "delete-doit"),
            Self::CreateUser(_) => write!(f, "create-user"),
            Self::ReadUser => write!(f, "read-user"),
            Self::CreateLabel => write!(f, "create-label"),
            Self::ReadLabel => write!(f, "read-label"),
            Self::UpdateLabel => write!(f, "update-label"),
            Self::DeleteLabel => write!(f, "delete-label"),
        }
    }
}
