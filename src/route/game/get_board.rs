use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::PgPool;
use tracing::error;

use crate::authentication::LoggedUser;

#[tracing::instrument]
pub async fn handler(
    State(postgres): State<PgPool>,
    Extension(user): Extension<LoggedUser>,
    Query(payload): Query<GetBoard>,
) -> Result<Json<Answer>, StatusCode> {
    let res = sqlx::query_as!(
        CGame,
        "
        SELECT 
            player_w, 
            player_b,
            fen
        FROM games.t_active
        WHERE id = $1
        ",
        payload.id,
    )
    .fetch_optional(&postgres)
    .await
    .map_err(|err| {
        error!("Error getting fen of a board {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    let (is_w, is_b) = (
        &res.player_w == user.username(),
        &res.player_b == user.username(),
    );

    let opponent: String;
    if is_w {
        opponent = res.player_b;
    } else if is_b {
        opponent = res.player_w;
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(Answer {
        opponent,
        fen: res.fen,
    }))
}

#[derive(sqlx::FromRow, Debug)]
pub struct CGame {
    player_w: String,
    player_b: String,
    fen: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GetBoard {
    id: i64,
}

#[derive(serde::Serialize, Debug)]
pub struct Answer {
    opponent: String,
    fen: String,
}
