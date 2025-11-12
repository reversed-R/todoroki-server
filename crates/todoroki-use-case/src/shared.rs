use todoroki_domain::entities::client::Client;

pub trait ContextProvider {
    fn client(&self) -> &Client;

    fn config(&self) -> &impl ConfigProvider;
}

pub trait ConfigProvider {
    fn firebase_project_id(&self) -> &str;
}
