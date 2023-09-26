use crate::authentication::LoggedUser;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Result},
    Extension,
};
use serde::Serialize;
use sqlx::PgPool;
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Extension(user): Extension<LoggedUser>,
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
        user.username(),
    )
    .fetch_all(&postgres)
    .await
    .map_err(|err| {
        error!("Error checking invites {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(res))
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Inviter {
    inviter: Option<String>,
}
