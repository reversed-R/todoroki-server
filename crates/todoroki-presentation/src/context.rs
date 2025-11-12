use todoroki_domain::entities::{
    client::{Client, ContextedClient},
    user::UserEmail,
};
use todoroki_use_case::shared::{ConfigProvider, ContextProvider};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Context {
    client: Client,
    config: Config,
}

impl Context {
    pub fn new(client: Client, config: Config) -> Self {
        Self { client, config }
    }
}

impl ContextProvider for Context {
    fn client<'a>(&'a self) -> ContextedClient<'a> {
        ContextedClient::new(
            &self.client,
            UserEmail::new(self.config.default_owner_email().to_string()),
        )
    }

    fn config(&self) -> &impl ConfigProvider {
        &self.config
    }
}
