use thiserror;

use crate::entities::doit::{Doit, DoitId, DoitUpdateCommand};

#[derive(Debug, Clone, thiserror::Error)]
pub enum DoitRepositoryError {
    #[error("Internal Error: {0:?}")]
    InternalError(String),
}

#[allow(async_fn_in_trait)]
pub trait DoitRepository: Send + Sync + 'static {
    async fn create(&self, doit: Doit) -> Result<DoitId, DoitRepositoryError>;

    async fn update(&self, cmd: DoitUpdateCommand) -> Result<(), DoitRepositoryError>;

    // async fn get_by_id(&self, id: DoitId) -> Result<Option<Doit>, DoitRepositoryError>;

    async fn list(&self) -> Result<Vec<Doit>, DoitRepositoryError>;

    async fn delete_by_id(&self, id: DoitId) -> Result<(), DoitRepositoryError>;
}
