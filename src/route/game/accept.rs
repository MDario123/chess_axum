use crate::authentication::LoggedUser;
use axum::{extract::State, http::StatusCode, response::Result, Extension, Json};
use serde::Deserialize;
use sqlx::{error::ErrorKind, PgPool};
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Extension(user): Extension<LoggedUser>,
    Json(payload): Json<Accept>,
) -> Result<StatusCode> {
    info!("Accepting invite");

    // Begin transaction
    let mut trx = postgres.begin().await.map_err(|err| {
        error!("Error starting transaction {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Attempt to delete invite
    // If it doesn't delete anything send StatusCode::NOT_FOUND
    let affected = sqlx::query!(
        "
        DELETE FROM games.v_pending_invites
        WHERE inviter = $1
          AND invited = $2
        ",
        payload.inviter,
        user.username(),
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| {
        error!("Error deleting invite {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .rows_affected();

    if affected == 0 {
        return Err(StatusCode::NOT_FOUND.into());
    }

    // Insert a new game as active between this 2 players
    // If there is already one return StatusCode::NOT_ACCEPTABLE
    sqlx::query!(
        "
        INSERT INTO games.t_active(player_w, player_b, fen, start_pos) 
        VALUES ($1, $2, $3, $3)
        ",
        payload.inviter,
        user.username(),
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

    // Everything went well, commit the transaction
    trx.commit().await.map_err(|err| {
        error!("Error commiting transaction {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct Accept {
    inviter: String,
}
