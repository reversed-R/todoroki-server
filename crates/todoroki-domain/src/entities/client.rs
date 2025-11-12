use getset::Getters;

use crate::entities::user::{User, UserEmail};

#[derive(Debug, Clone)]
pub enum Client {
    User(User),
    Unregistered(UserEmail),
    Unverified,
}

#[derive(Debug, Clone, Getters)]
pub struct ContextedClient<'a> {
    #[getset(get = "pub")]
    client: &'a Client,

    #[getset(get = "pub")]
    default_owner_email: UserEmail,
}

impl<'a> ContextedClient<'a> {
    pub fn new(client: &'a Client, default_owner_email: UserEmail) -> Self {
        Self {
            client,
            default_owner_email,
        }
    }
}
