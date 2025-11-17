use crate::{
    doit::{dto::DoitDto, DoitUseCase, DoitUseCaseError},
    shared::ContextProvider,
};

use todoroki_domain::{
    entities::doit::{Doit, DoitId, DoitUpdateCommand},
    repositories::{doit::DoitRepository, Repositories},
    value_objects::{error::ErrorCode, permission::Permission},
};

impl<R: Repositories> DoitUseCase<R> {
    pub async fn create(
        &self,
        doit: Doit,
        ctx: &impl ContextProvider,
    ) -> Result<DoitId, ErrorCode> {
        ctx.client().has_permission(Permission::CreateDoit)?;

        let res = self.repositories.doit_repository().create(doit).await;

        res.map_err(DoitUseCaseError::DoitRepositoryError)
            .map_err(|e| e.into())
    }

    pub async fn list(&self, ctx: &impl ContextProvider) -> Result<Vec<DoitDto>, ErrorCode> {
        ctx.client().has_permission(Permission::ReadDoit)?;

        let res = self.repositories.doit_repository().list().await;

        res.map_err(DoitUseCaseError::DoitRepositoryError)
            .map_err(ErrorCode::from)?
            .into_iter()
            .map(|d| DoitDto::try_from_with_permission(d, ctx.client()))
            .collect()
    }

    pub async fn update(
        &self,
        cmd: DoitUpdateCommand,
        ctx: &impl ContextProvider,
    ) -> Result<(), ErrorCode> {
        todo!()

        // let doit = self
        //     .repositories
        //     .doit_repository()
        //     .get_by_id(cmd.id().clone())
        //     .await
        //     .map_err(DoitUseCaseError::DoitRepositoryError)
        //     .map_err(|e| e.into())?;
        //
        // ctx.client().has_permission(Permission::UpdateDoit(doit))?;
        //
        // let res = self.repositories.doit_repository().update(cmd).await;
        //
        // res.map_err(DoitUseCaseError::DoitRepositoryError)
        //     .map_err(|e| e.into())
    }
}
