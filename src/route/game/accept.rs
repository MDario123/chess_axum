use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{error::ErrorKind, PgPool};
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(State(postgres): State<PgPool>, Json(payload): Json<Accept>) -> StatusCode {
    info!("Accepting invite");

    let mut trx = match postgres.begin().await {
        Ok(x) => x,
        Err(err) => {
            error!("Error starting transaction {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let res = sqlx::query!(
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
    .await;

    let res = match res {
        Ok(x) => x,
        Err(err) => {
            error!("Error checking if this invite exists {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let exists = res.map_or_else(|| false, |_| true);

    if !exists {
        return StatusCode::NOT_ACCEPTABLE;
    }

    let res = sqlx::query!(
        "
        DELETE FROM games.v_pending_invites
        WHERE inviter = $1
          AND invited = $2
        ",
        payload.inviter,
        payload.invited,
    )
    .execute(&mut *trx)
    .await;

    match res {
        Ok(_) => (),
        Err(err) => {
            error!("Error deleting invite {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let res = sqlx::query!(
        "
        INSERT INTO games.t_active(player_w, player_b, fen) 
        VALUES ($1, $2, $3)
        ",
        payload.inviter,
        payload.invited,
        chess::Board::default().to_string(),
    )
    .execute(&mut *trx)
    .await;

    match res {
        Ok(_) => (),
        Err(err) => {
            if err
                .as_database_error()
                .is_some_and(|err| err.kind() == ErrorKind::UniqueViolation)
            {
                return StatusCode::NOT_ACCEPTABLE;
            } else {
                error!("Error inserting new game {err}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    };

    match trx.commit().await {
        Ok(_) => (),
        Err(err) => {
            error!("Error commiting transaction {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    StatusCode::OK
}

#[derive(Debug, Deserialize)]
pub struct Accept {
    invited: String,
    inviter: String,
}
