use todoroki_domain::entities::user::UserEmail;
use todoroki_use_case::shared::ContextProvider;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Context {
    user_email: UserEmail,
    config: Config,
}

impl Context {
    pub fn new(user_email: UserEmail, config: Config) -> Self {
        Self { user_email, config }
    }
}

impl ContextProvider for Context {
    fn user_email(&self) -> &UserEmail {
        &self.user_email
    }

    fn config(&self) -> &impl todoroki_use_case::shared::ConfigProvider {
        &self.config
    }
}
