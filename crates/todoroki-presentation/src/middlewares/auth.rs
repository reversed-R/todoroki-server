use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};

use serde::{Deserialize, Serialize};
use todoroki_domain::{
    entities::user_auth::UserAuthToken, repositories::Repositories, value_objects::error::ErrorCode,
};

use crate::{models::responses::error::ErrorResponse, modules::Modules};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub email_verified: bool,
}

pub(crate) async fn jwt_auth(
    State(modules): State<Arc<Modules<impl Repositories>>>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, ErrorResponse> {
    let authorization_header =
        request
            .headers()
            .get("Authorization")
            .ok_or(ErrorResponse::from(
                ErrorCode::UserAuthTokenVerificationError(
                    "missing-authorization-header".to_string(),
                ),
            ))?;

    let authorization = authorization_header.to_str().map_err(|e| {
        ErrorResponse::from(ErrorCode::UserAuthTokenVerificationError(
            "invalid-authorization-header".to_string(),
        ))
    })?;

    if !authorization.starts_with("Bearer ") {
        return Err(ErrorResponse::from(
            ErrorCode::UserAuthTokenVerificationError(
                "invalid-authorization-header, should be starts with `Bearer`".to_string(),
            ),
        ));
    }

    let jwt_token = authorization.trim_start_matches("Bearer ");

    let verified_user = modules
        .user_use_case()
        .verify(UserAuthToken::new(jwt_token.to_string()))
        .await
        .map_err(ErrorResponse::from)?;

    // もし user_id 以上のものを Extension に入れるなら、ここで渡す
    let ctx = Context::new(
        verified_user.email().clone(),
        modules.config().clone().into(),
    );
    request.extensions_mut().insert(ctx);

    Ok(next.run(request).await)
}
