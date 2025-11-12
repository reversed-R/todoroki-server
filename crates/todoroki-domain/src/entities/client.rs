use crate::entities::user::{User, UserEmail};

#[derive(Debug, Clone)]
pub enum Client {
    User(User),
    Unregistered(UserEmail),
    Unverified,
}
