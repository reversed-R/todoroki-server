use thiserror;

use crate::entities::label::{Label, LabelId};

#[derive(Debug, Clone, thiserror::Error)]
pub enum LabelRepositoryError {
    #[error("Internal Error: {0:?}")]
    InternalError(String),
}

#[allow(async_fn_in_trait)]
pub trait LabelRepository: Send + Sync + 'static {
    async fn create(&self, label: Label) -> Result<LabelId, LabelRepositoryError>;

    // async fn update(&self, cmd: LabelUpdateCommand) -> Result<(), LabelRepositoryError>;

    // async fn get_by_id(&self, id: LabelId) -> Result<Option<Label>, LabelRepositoryError>;

    async fn list(&self) -> Result<Vec<Label>, LabelRepositoryError>;

    async fn delete_by_id(&self, id: LabelId) -> Result<(), LabelRepositoryError>;
}
