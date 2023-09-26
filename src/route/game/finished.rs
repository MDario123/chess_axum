use crate::authentication::LoggedUser;
use axum::{extract::State, http::StatusCode, response::Result, Extension, Json};
use sqlx::PgPool;
use tracing::error;

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Extension(user): Extension<LoggedUser>,
) -> Result<Json<Vec<FGames>>> {
    let finished_games = sqlx::query_as!(
        FGames,
        "
        SELECT
            CASE
                WHEN player_w = $1 THEN player_b
                WHEN player_b = $1 THEN player_w
            END as opponent,
            moves as pgn
        FROM games.t_finished
        WHERE player_w = $1
           OR player_b = $1
        ORDER BY end_date DESC
        ",
        user.username(),
    )
    .fetch_all(&postgres)
    .await
    .map_err(|err| {
        error!("Error getting finished games {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(finished_games))
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct FGames {
    opponent: Option<String>,
    pgn: Option<String>,
}
