use todoroki_domain::entities::user::UserEmail;

pub trait ContextProvider {
    fn user_email(&self) -> &Option<UserEmail>;

    fn config(&self) -> &impl ConfigProvider;
}

pub trait ConfigProvider {
    fn firebase_project_id(&self) -> &str;
}
