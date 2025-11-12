use todoroki_domain::entities::client::ContextedClient;

pub trait ContextProvider {
    fn client<'a>(&'a self) -> ContextedClient<'a>;

    fn config(&self) -> &impl ConfigProvider;
}

pub trait ConfigProvider {
    fn firebase_project_id(&self) -> &str;

    fn default_owner_email(&self) -> &str;
}
