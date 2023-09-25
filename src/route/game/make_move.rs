use std::str::FromStr;

use axum::{extract::State, http::StatusCode, Json};
use chess::{Board, BoardStatus, ChessMove};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;

pub async fn handler(State(postgres): State<PgPool>, Json(payload): Json<Move>) -> StatusCode {
    // Start transaction
    let trx = postgres.begin().await;

    let mut trx = match trx {
        Ok(x) => x,
        Err(err) => {
            error!("Error starting transaction {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // Get details for the current game
    let res = sqlx::query_as!(
        CGame,
        "
        SELECT id, fen, start_pos
        FROM games.t_active
        WHERE player_w = $1
          AND player_b = $2
        ",
        payload.player_w,
        payload.player_b,
    )
    .fetch_optional(&mut *trx)
    .await;

    let cgame = match res {
        Ok(x) => match x {
            Some(x) => x,
            None => return StatusCode::NOT_ACCEPTABLE,
        },
        Err(err) => {
            error!("Error getting game {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // Interpret game from string
    let board = match Board::from_str(&cgame.fen) {
        Ok(x) => x,
        Err(err) => {
            error!("Error interpreting fen from database {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // Interpret move in this game
    let cmove: ChessMove = match ChessMove::from_san(&board, &payload.san) {
        Ok(x) => x,
        Err(_) => return StatusCode::NOT_ACCEPTABLE,
    };

    // Insert move in the database
    let res = sqlx::query!(
        "
        INSERT INTO games.t_moves(id_game, san, previous_fen, move_num)
        SELECT 
            $1 as id_game, 
            $2 as san, 
            $3 as previous_fen,
            move_num + 1 as move_num
        FROM games.t_moves
        WHERE id_game = $1
        ORDER BY move_num DESC
        LIMIT 1
        ",
        cgame.id,
        payload.san,
        cgame.fen,
    )
    .execute(&mut *trx)
    .await;

    match res {
        Ok(_) => (),
        Err(err) => {
            error!("Error inserting move {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    // Make the move in this board
    let board = board.make_move_new(cmove);

    // Check if the position has repeated 3 times
    let res = sqlx::query!(
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
    .await;

    let repeated: bool = match res {
        Ok(x) => x.is_some(),
        Err(err) => {
            error!("Error counting repetitions {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if board.status() != BoardStatus::Ongoing || repeated {
        let res = sqlx::query_as!(
            CMove,
            "
            SELECT san
            FROM games.t_moves
            WHERE id_game = $1
            ",
            cgame.id,
        )
        .fetch_all(&mut *trx)
        .await;

        let moves: String = match res {
            Ok(x) => x
                .into_iter()
                .map(|x| x.san)
                .collect::<Vec<String>>()
                .join(" "),
            Err(err) => {
                error!("Error getting moves {err}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        let res = sqlx::query!(
            "
            DELETE FROM games.t_active
            WHERE id = $1
            ",
            cgame.id,
        )
        .execute(&mut *trx)
        .await;

        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error deleting active board {err}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        let res = sqlx::query!(
            "
            INSERT INTO games.t_finished(id, start_pos, moves)
            VALUES ($1, $2, $3)
            ",
            cgame.id,
            cgame.start_pos,
            moves,
        )
        .execute(&mut *trx)
        .await;

        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error inserting finished game {err}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };
    } else {
        let res = sqlx::query!(
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
        .await;

        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error inserting new board {err}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };
    }

    match trx.commit().await {
        Ok(_) => (),
        Err(err) => {
            error!("Error commiting transaction {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    StatusCode::OK
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
}

#[derive(sqlx::FromRow)]
pub struct CMove {
    san: String,
}
