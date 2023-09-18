use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Result,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::error;

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Query(user): Query<User>,
) -> Result<(StatusCode, Json<Vec<Record>>), StatusCode> {
    let res = sqlx::query_as!(
        Record,
        "
        SELECT game_id as id
        FROM games.invited
        WHERE inv_player = $1
        ",
        user.user
    )
    .fetch_all(&postgres)
    .await;

    let res = match res {
        Ok(ok) => ok,
        Err(err) => {
            error!("Error ocurred getting invited games: {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok((
        if res.is_empty() {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::OK
        },
        Json(res),
    ))
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Record {
    id: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    user: String,
}
