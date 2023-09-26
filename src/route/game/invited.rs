use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Json, Result},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Query(payload): Query<User>,
) -> Result<Json<Vec<Inviter>>, StatusCode> {
    info!("Checking invites");

    let res = sqlx::query_as!(
        Inviter,
        "
        SELECT inviter
        FROM games.v_pending_invites
        WHERE invited = $1
        ORDER BY created_at DESC
        ",
        payload.username,
    )
    .fetch_all(&postgres)
    .await
    .map_err(|err| {
        error!("Error checking invites {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(res))
}

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Inviter {
    inviter: Option<String>,
}
