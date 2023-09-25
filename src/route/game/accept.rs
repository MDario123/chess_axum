use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{error::ErrorKind, PgPool};
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Json(payload): Json<Accept>,
) -> Result<StatusCode, StatusCode> {
    info!("Accepting invite");

    let mut trx = postgres.begin().await.map_err(|err| {
        error!("Error starting transaction {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query!(
        "
        SELECT true as exists
        FROM games.v_pending_invites
        WHERE inviter = $1
          AND invited = $2
        ",
        payload.inviter,
        payload.invited,
    )
    .fetch_optional(&mut *trx)
    .await
    .map_err(|err| {
        error!("Error checking if this invite exists {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .map_or_else(|| Err(StatusCode::NOT_FOUND), |_| Ok(true))?;

    sqlx::query!(
        "
        DELETE FROM games.v_pending_invites
        WHERE inviter = $1
          AND invited = $2
        ",
        payload.inviter,
        payload.invited,
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| {
        error!("Error deleting invite {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query!(
        "
        INSERT INTO games.t_active(player_w, player_b, fen, start_pos) 
        VALUES ($1, $2, $3, $3)
        ",
        payload.inviter,
        payload.invited,
        chess::Board::default().to_string(),
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| {
        if err
            .as_database_error()
            .is_some_and(|err| err.kind() == ErrorKind::UniqueViolation)
        {
            StatusCode::NOT_ACCEPTABLE
        } else {
            error!("Error inserting new game {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    trx.commit().await.map_err(|err| {
        error!("Error commiting transaction {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct Accept {
    invited: String,
    inviter: String,
}
