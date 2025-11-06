use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{
    decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use todoroki_domain::{entities::user_auth::UserAuthToken, repositories::Repositories};

use crate::modules::Modules;

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
    State(modules): State<Arc<Modules<Repositories>>>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let authorization_header = request.headers().get("Authorization").ok_or(AppError::new(
        StatusCode::UNAUTHORIZED,
        "auth/missing-authorization-header".to_string(),
        "Authorization header is missing.".to_string(),
    ))?;
    let authorization = authorization_header.to_str().map_err(|e| {
        AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/invalid-authorization-header".to_string(),
            e.to_string(),
        )
    })?;

    if !authorization.starts_with("Bearer ") {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/invalid-authorization-header".to_string(),
            "Authorization header is invalid. It should start with 'Bearer'.".to_string(),
        ));
    }

    let jwt_token = authorization.trim_start_matches("Bearer ");

    let token = match modules
        .user_use_case()
        .verify(UserAuthToken::new(jwt_token.clone()))
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "auth/invalid-token".to_string(),
                e.to_string(),
            ));
        }
    };

    // メールが認証されているか確認
    if modules.config().require_email_verification && !token.claims.email_verified {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/email-not-verified".to_string(),
            "Email is not verified.".to_string(),
        ));
    }

    // もし user_id 以上のものを Extension に入れるなら、ここで渡す
    let ctx = Context::new(token.claims.sub.clone(), modules.config().clone().into());
    request.extensions_mut().insert(ctx);

    Ok(next.run(request).await)
}
