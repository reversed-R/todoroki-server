pub mod todo;
pub mod user;
pub mod user_auth;

pub trait Repositories: Send + Sync + 'static {
    type TodoRepositoryImpl: todo::TodoRepository;
    type UserRepositoryImpl: user::UserRepository;

    fn todo_repository(&self) -> &Self::TodoRepositoryImpl;
    fn user_repository(&self) -> &Self::UserRepositoryImpl;
}
