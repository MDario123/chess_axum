use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::Result, Json};
use chess::{Board, BoardStatus, ChessMove};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;

pub async fn handler(
    State(postgres): State<PgPool>,
    Json(payload): Json<Move>,
) -> Result<StatusCode> {
    // Start transaction
    let mut trx = postgres.begin().await.map_err(|err| {
        error!("Error starting transaction {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get details for the current game
    let cgame = sqlx::query_as!(
        CGame,
        "
        SELECT id, fen, start_pos, COALESCE(mo.move_num, 0::int) as last_move
        FROM games.t_active ac
            LEFT JOIN games.t_moves mo ON mo.id_game = ac.id
        WHERE player_w = $1
          AND player_b = $2
        ORDER BY mo.move_num DESC
        LIMIT 1
        ",
        payload.player_w,
        payload.player_b,
    )
    .fetch_optional(&mut *trx)
    .await
    .map_err(|err| {
        error!("Error getting game {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_ACCEPTABLE)?;

    // Interpret game from string
    let board = Board::from_str(&cgame.fen).map_err(|err| {
        error!("Error interpreting fen from database {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Interpret move in this game
    let cmove: ChessMove =
        ChessMove::from_san(&board, &payload.san).map_err(|_| StatusCode::NOT_ACCEPTABLE)?;

    // Insert move in the database
    sqlx::query!(
        "
        INSERT INTO games.t_moves(id_game, san, previous_fen, move_num)
        VALUES($1, $2, $3, $4)
        ",
        cgame.id,
        payload.san,
        cgame.fen,
        cgame.last_move.unwrap_or(0) + 1,
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| {
        error!("Error inserting move {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Make the move in this board
    let board = board.make_move_new(cmove);

    // Check if the position has repeated 3 times
    let repeated = sqlx::query!(
        "
        SELECT 
        FROM games.t_moves
        WHERE id_game = $1
          AND previous_fen = $2
        GROUP BY previous_fen
        HAVING COUNT(1) >= 2
        ",
        cgame.id,
        board.to_string(),
    )
    .fetch_optional(&mut *trx)
    .await
    .map_err(|err| {
        error!("Error counting repetitions {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .is_some();

    if board.status() != BoardStatus::Ongoing || repeated {
        let moves = sqlx::query_as!(
            CMove,
            "
            SELECT san
            FROM games.t_moves
            WHERE id_game = $1
            ORDER BY move_num
            ",
            cgame.id,
        )
        .fetch_all(&mut *trx)
        .await
        .map_err(|err| {
            error!("Error getting moves {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(|x| x.san)
        .collect::<Vec<String>>()
        .join(" ");

        sqlx::query!(
            "
            DELETE FROM games.t_active
            WHERE id = $1
            ",
            cgame.id,
        )
        .execute(&mut *trx)
        .await
        .map_err(|err| {
            error!("Error deleting active board {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        sqlx::query!(
            "
            INSERT INTO games.t_finished(
                id,
                start_pos,
                moves,
                player_w,
                player_b
            )
            VALUES ($1, $2, $3, $4, $5)
            ",
            cgame.id,
            cgame.start_pos,
            moves,
            payload.player_w,
            payload.player_b,
        )
        .execute(&mut *trx)
        .await
        .map_err(|err| {
            error!("Error inserting finished game {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    } else {
        sqlx::query!(
            "
        UPDATE games.t_active
        SET fen = $1
        WHERE player_w = $2
          AND player_b = $3
        ",
            board.to_string(),
            payload.player_w,
            payload.player_b,
        )
        .execute(&mut *trx)
        .await
        .map_err(|err| {
            error!("Error inserting new board {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    trx.commit().await.map_err(|err| {
        error!("Error commiting transaction {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Debug)]
pub struct Move {
    player_w: String,
    player_b: String,
    san: String,
}

#[derive(sqlx::FromRow)]
pub struct CGame {
    id: i64,
    fen: String,
    start_pos: String,
    last_move: Option<i32>,
}

#[derive(sqlx::FromRow)]
pub struct CMove {
    san: String,
}
