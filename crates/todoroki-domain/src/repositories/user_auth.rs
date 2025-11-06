use thiserror;

use crate::entities::user_auth::VerificationKey;

#[derive(Debug, Clone, thiserror::Error)]
pub enum UserAuthRepositoryError {
    #[error("Internal Error: {0:?}")]
    InternalError(String),
}

#[allow(async_fn_in_trait)]
pub trait UserAuthRepository: Send + Sync + 'static {
    async fn get_key_by_id(&self, id: String) -> Result<VerificationKey, UserAuthRepositoryError>;
}
