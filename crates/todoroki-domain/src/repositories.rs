pub mod doit;
pub mod label;
pub mod todo;
pub mod user;
pub mod user_auth;

pub trait Repositories: Send + Sync + 'static {
    type TodoRepositoryImpl: todo::TodoRepository;
    type DoitRepositoryImpl: doit::DoitRepository;
    type LabelRepositoryImpl: label::LabelRepository;
    type UserRepositoryImpl: user::UserRepository;
    type UserAuthRepositoryImpl: user_auth::UserAuthRepository;

    fn todo_repository(&self) -> &Self::TodoRepositoryImpl;
    fn doit_repository(&self) -> &Self::DoitRepositoryImpl;
    fn label_repository(&self) -> &Self::LabelRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepositoryImpl;
    fn user_auth_repository(&self) -> &Self::UserAuthRepositoryImpl;
}
