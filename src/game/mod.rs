use std::collections::HashMap;

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Serialize)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Serialize)]
struct ColoredPiece {
    color: Color,
    piece: Piece,
}

#[derive(Clone, Copy, Serialize)]
struct Pos(u8);

#[derive(Clone, Copy, Serialize)]
enum ChessMove {
    // ShortCastle,
    // LongCastle,
    // PawnPromotion(Pos, Pos, Piece),
    Regular(Pos, Pos),
}

#[derive(Clone, Copy, Serialize)]
struct Board {
    squares: [[Option<ColoredPiece>; 8]; 8],
}

#[derive(Clone, Serialize)]
pub struct Game {
    starting_board: Board,
    current_board: Board,
    half_move_amount: usize,
    half_moves: Vec<Option<ChessMove>>,
    board_history: HashMap<Board, u32>,
}

mod new_game;
mod notation;
