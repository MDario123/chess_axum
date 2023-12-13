use crate::authentication::{generate_token, is_pass_equivalent};
use axum::{
    extract::{Json, State},
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
    Json(user): Json<LoginAttempt>,
) -> Result<String, StatusCode> {
    info!("Starting!");
    // Get user and password
    let pot_user = sqlx::query_as!(
        User,
        "
        SELECT
            id,
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
        return Err(StatusCode::NOT_ACCEPTABLE);
    }

    let token = generate_token();

    sqlx::query!(
        "
        INSERT INTO users.token(token, user_id)
        VALUES($1, $2)
        ",
        &token,
        pot_user.id,
    )
    .execute(&pool)
    .await
    .map_err(|err| {
        error!("Error storing new token {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(token)
}

// the input to our handler
#[derive(sqlx::FromRow)]
pub(crate) struct User {
    id: i64,
    password: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginAttempt {
    username: String,
    password: String,
}
