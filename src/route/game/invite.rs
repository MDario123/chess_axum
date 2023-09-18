use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{error::ErrorKind, PgPool};
use tracing::{error, info};

#[tracing::instrument]
pub async fn handler(
    // database
    State(postgres): State<PgPool>,
    Json(payload): Json<Invite>,
) -> StatusCode {
    let res = sqlx::query!(
        "
        INSERT INTO games.active_matches (player1, player2, game)
        SELECT u1.id, u2.id, $3
        FROM users.basic_info u1
            CROSS JOIN users.basic_info u2
        WHERE u1.username = $1
          AND u2.username = $2
        ",
        payload.player1,
        payload.player2,
        chess::Board::default().to_string(),
    )
    .execute(&postgres)
    .await;

    match res {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            if err
                .as_database_error()
                .is_some_and(|err| err.kind() == ErrorKind::UniqueViolation)
            {
                info!(
                    "There is already a game between {} and {}",
                    payload.player1, payload.player2
                );
                StatusCode::NOT_ACCEPTABLE
            } else {
                error!("Error inserting to database: {err}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Invite {
    player1: String,
    player2: String,
}
