use crate::{
    label::{LabelUseCase, LabelUseCaseError},
    shared::ContextProvider,
};

use todoroki_domain::{
    entities::label::{Label, LabelId},
    repositories::{label::LabelRepository, Repositories},
    value_objects::{error::ErrorCode, permission::Permission},
};

impl<R: Repositories> LabelUseCase<R> {
    pub async fn create(
        &self,
        label: Label,
        ctx: &impl ContextProvider,
    ) -> Result<LabelId, ErrorCode> {
        ctx.client().has_permission(Permission::CreateLabel)?;

        let res = self.repositories.label_repository().create(label).await;

        res.map_err(LabelUseCaseError::LabelRepositoryError)
            .map_err(|e| e.into())
    }

    pub async fn list(&self, ctx: &impl ContextProvider) -> Result<Vec<Label>, ErrorCode> {
        ctx.client().has_permission(Permission::ReadLabel)?;

        let res = self.repositories.label_repository().list().await;

        res.map_err(LabelUseCaseError::LabelRepositoryError)
            .map_err(|e| e.into())
    }
}
