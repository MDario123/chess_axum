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
) -> Result<Json<Vec<ABoard>>, StatusCode> {
    let boards = sqlx::query_as!(
        ABoard,
        "
        SELECT 
            player_b as opponent,
            fen
        FROM games.t_active
        WHERE player_w = $1
        
        UNION 
        
        SELECT 
            player_w as opponent,
            fen
        FROM games.t_active
        WHERE player_b = $1
        ",
        user.username,
    )
    .fetch_all(&postgres)
    .await
    .map_err(|err| {
        error!("Error getting active boards {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(boards))
}

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct ABoard {
    opponent: Option<String>,
    fen: Option<String>,
}
