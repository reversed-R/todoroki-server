pub mod todo;

pub trait Repositories: Send + Sync + 'static {
    type TodoRepositoryImpl: todo::TodoRepository;

    fn todo_repository(&self) -> &Self::TodoRepositoryImpl;
}
