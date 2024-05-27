#[cfg(test)]
pub mod tests;

use crate::chess_piece::{ChessPiece, Color, Piece};
use crate::utils::get_fields::get_fields;
use crate::utils::make_special_moves::make_king_move;
use crate::validate_move::bishop::validate_bishop_move;
use uuid::Uuid;

pub struct Game {
    pub id: Uuid,
    pub next_to_move: Color,
    pub previous_move: String,
    pub can_castle: CastlingRights,
    pub field: Vec<Vec<Option<ChessPiece>>>,
}

pub struct CastlingRights {
    pub white_can_short_castle: bool,
    pub white_can_long_castle: bool,
    pub black_can_short_castle: bool,
    pub black_can_long_castle: bool,
}

impl Game {
    pub fn new() -> Game {
        create_new_game()
    }
    pub fn validate_move<'a>(&self, from: &'a str, to: &'a str) -> Result<bool, &'a str> {
        let (from, to) = get_fields(from, to)?;

        match self.field[from.0][from.1] {
            None => Ok(false),
            _ => Ok(true),
        }
    }
    pub fn make_move<'a>(
        &'a mut self,
        from: &'a str,
        to: &'a str,
        promotion_piece: Option<Piece>,
    ) -> Result<(), &'a str> {
        if !Self::validate_move(self, from, to)? {
            return Err("Invalid move");
        }

        let (from, to) = get_fields(from, to)?;
        self.field[to.0][to.1] = self.field[from.0][from.1];

        // In case of castles
        if self.field[from.0][from.1].unwrap().piece == Piece::KING {
            let _ = make_king_move(from, to, &mut self.field);
        }

        // In case of pawn promotions or "en passant"
        if self.field[from.0][from.1].unwrap().piece == Piece::PAWN {
            if to.0 == 7 || to.0 == 0 {
                match promotion_piece {
                    None => return Err("No promotion piece specified for promotion"),
                    Some(prom_piece) => self.field[to.0][to.1].unwrap().piece = prom_piece,
                }
            }
            if from.1 != to.1 {
                self.field[to.0][from.1] = None;
            }
        }

        self.field[from.0][from.1] = None;
        match self.next_to_move {
            Color::BLACK => self.next_to_move = Color::WHITE,
            Color::WHITE => self.next_to_move = Color::BLACK,
        }

        Ok(())
    }
}

fn create_new_game() -> Game {
    Game {
        id: Uuid::new_v4(),
        previous_move: "".to_string(),
        next_to_move: Color::WHITE,
        can_castle: CastlingRights {
            white_can_short_castle: true,
            white_can_long_castle: true,
            black_can_short_castle: true,
            black_can_long_castle: true,
        },
        field: vec![
            vec![
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::ROOK,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::KNIGHT,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::BISHOP,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::QUEEN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::KING,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::BISHOP,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::KNIGHT,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::ROOK,
                }),
            ],
            vec![
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::BLACK,
                    piece: Piece::PAWN,
                }),
            ],
            vec![None; 8],
            vec![None; 8],
            vec![None; 8],
            vec![None; 8],
            vec![
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::PAWN,
                }),
            ],
            vec![
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::ROOK,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::KNIGHT,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::BISHOP,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::QUEEN,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::KING,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::BISHOP,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::KNIGHT,
                }),
                Some(ChessPiece {
                    color: Color::WHITE,
                    piece: Piece::ROOK,
                }),
            ],
        ],
    }
}
