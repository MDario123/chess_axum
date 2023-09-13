use crate::password_checker::is_pass_equivalent;
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;

#[tracing::instrument]
pub(crate) async fn handler(
    // database connection pool
    State(pool): State<PgPool>,
    Query(user): Query<User>,
) -> StatusCode {
    info!("Starting!");
    // Get user and password
    let query_res = sqlx::query_as!(
        User,
        "
        SELECT
            username,
            password
        FROM users.basic_info
        WHERE username = $1
        ",
        &user.username,
    )
    .fetch_optional(&pool)
    .await;

    match query_res {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(row) => match row {
            None => StatusCode::NOT_FOUND,
            Some(row) => {
                let pot_user: User = row;
                if is_pass_equivalent(&user.password, &pot_user.password) {
                    StatusCode::OK
                } else {
                    StatusCode::NOT_ACCEPTABLE
                }
            }
        },
    }
}

// the input to our handler
#[derive(Deserialize, Debug, sqlx::FromRow)]
pub(crate) struct User {
    username: String,
    password: String,
}
