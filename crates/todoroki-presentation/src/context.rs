use todoroki_domain::entities::client::Client;
use todoroki_use_case::shared::ContextProvider;

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
    fn client(&self) -> &Client {
        &self.client
    }

    fn config(&self) -> &impl todoroki_use_case::shared::ConfigProvider {
        &self.config
    }
}
