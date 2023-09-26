use crate::authentication::LoggedUser;
use axum::{extract::State, http::StatusCode, response::Result, Extension, Json};
use serde::Serialize;
use sqlx::PgPool;
use tracing::error;

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Extension(user): Extension<LoggedUser>,
) -> Result<Json<Vec<ABoard>>, StatusCode> {
    let boards = sqlx::query_as!(
        ABoard,
        "
        SELECT 
            id,
            player_b as opponent,
            fen
        FROM games.t_active
        WHERE player_w = $1
        
        UNION 
        
        SELECT 
            id, 
            player_w as opponent,
            fen
        FROM games.t_active
        WHERE player_b = $1
        ",
        user.username(),
    )
    .fetch_all(&postgres)
    .await
    .map_err(|err| {
        error!("Error getting active boards {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(boards))
}

#[derive(sqlx::FromRow, Serialize)]
pub struct ABoard {
    id: Option<i64>,
    opponent: Option<String>,
    fen: Option<String>,
}
