use std::collections::HashMap;

use super::{Board, Color, ColoredPiece, Game, Piece};

type CP = ColoredPiece;
type C = Color;
type P = Piece;

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [
                [
                    Some(CP {
                        color: C::White,
                        piece: P::Rook,
                    }),
                    Some(CP {
                        color: C::White,
                        piece: P::Knight,
                    }),
                    Some(CP {
                        color: C::White,
                        piece: P::Bishop,
                    }),
                    Some(CP {
                        color: C::White,
                        piece: P::Queen,
                    }),
                    Some(CP {
                        color: C::White,
                        piece: P::King,
                    }),
                    Some(CP {
                        color: C::White,
                        piece: P::Bishop,
                    }),
                    Some(CP {
                        color: C::White,
                        piece: P::Knight,
                    }),
                    Some(CP {
                        color: C::White,
                        piece: P::Rook,
                    }),
                ],
                [Some(CP {
                    color: C::White,
                    piece: P::Pawn,
                }); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(CP {
                    color: C::Black,
                    piece: P::Pawn,
                }); 8],
                [
                    Some(CP {
                        color: C::Black,
                        piece: P::Rook,
                    }),
                    Some(CP {
                        color: C::Black,
                        piece: P::Knight,
                    }),
                    Some(CP {
                        color: C::Black,
                        piece: P::Bishop,
                    }),
                    Some(CP {
                        color: C::Black,
                        piece: P::Queen,
                    }),
                    Some(CP {
                        color: C::Black,
                        piece: P::King,
                    }),
                    Some(CP {
                        color: C::Black,
                        piece: P::Bishop,
                    }),
                    Some(CP {
                        color: C::Black,
                        piece: P::Knight,
                    }),
                    Some(CP {
                        color: C::Black,
                        piece: P::Rook,
                    }),
                ],
            ],
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            starting_board: Board::new(),
            current_board: Board::new(),
            half_move_amount: 0,
            half_moves: vec![],
            // TODO: include starting board on creation
            board_history: HashMap::new(),
        }
    }
}
