use crate::authentication::is_pass_equivalent;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Result,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info};

#[tracing::instrument]
pub(crate) async fn handler(
    // database connection pool
    State(pool): State<PgPool>,
    Query(user): Query<LoginAttempt>,
) -> Result<StatusCode> {
    info!("Starting!");
    // Get user and password
    let pot_user = sqlx::query_as!(
        User,
        "
        SELECT
            password
        FROM users.basic_info
        WHERE username = $1
        ",
        &user.username,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        error!("Error getting user's password {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    if !is_pass_equivalent(&user.password, &pot_user.password) {
        return Err(StatusCode::NOT_ACCEPTABLE.into());
    }

    Ok(StatusCode::OK)
}

// the input to our handler
#[derive(sqlx::FromRow)]
pub(crate) struct User {
    password: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginAttempt {
    username: String,
    password: String,
}
