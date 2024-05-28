pub mod chess_piece;
pub mod validation;

#[cfg(test)]
pub mod tests;

use crate::game::chess_piece::{ChessPiece, Color, Piece};
use crate::utils::get_fields::get_fields;
use uuid::Uuid;

use self::validation::bishop::validate_bishop_move;
use self::validation::rook::validate_rook_move;

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
    pub fn validate_move<'a>(
        &self,
        algebraic_from: &'a str,
        algebraic_to: &'a str,
    ) -> Result<(), &'a str> {
        let (from, to) = get_fields(algebraic_from, algebraic_to)?;

        match self.field[from.0][from.1] {
            None => Err("There is no piece on that square"),
            Some(x) => match x.piece {
                Piece::BISHOP => validate_bishop_move(from, to, &self),
                Piece::ROOK => validate_rook_move(from, to, &self),
                _ => Ok(()),
            },
        }
    }
    pub fn make_move<'a>(
        &'a mut self,
        algebraic_from: &'a str,
        algebraic_to: &'a str,
        promotion_piece: Option<Piece>,
    ) -> Result<(), &'a str> {
        Self::validate_move(self, algebraic_from, algebraic_to)?;

        let (from, to) = get_fields(algebraic_from, algebraic_to)?;

        // Add x in case we capture
        let move_with_capture = self.field[to.0][to.1].is_some();
        if move_with_capture {
            self.previous_move = "x".to_string();
        } else {
            self.previous_move = "".to_string();
        }

        // Target square
        self.previous_move.push_str(algebraic_to);

        // Move to new square
        self.field[to.0][to.1] = self.field[from.0][from.1];

        match self.field[from.0][from.1].unwrap().piece {
            Piece::KING => self.make_king_move(from, to),
            Piece::PAWN => self.make_pawn_move(from, to, promotion_piece)?,
            Piece::QUEEN => self.previous_move.insert(0, 'Q'),
            Piece::ROOK => self.make_rook_move(from),
            Piece::BISHOP => self.previous_move.insert(0, 'B'),
            Piece::KNIGHT => self.previous_move.insert(0, 'N'),
        }

        // Move away from old square
        self.field[from.0][from.1] = None;
        match self.next_to_move {
            Color::BLACK => self.next_to_move = Color::WHITE,
            Color::WHITE => self.next_to_move = Color::BLACK,
        }

        Ok(())
    }
    fn make_rook_move(&mut self, from: (usize, usize)) {
        self.previous_move.insert(0, 'R');

        // Take away castling rights if necessary
        if self.next_to_move == Color::BLACK {
            if from.0 == 0 && from.1 == 0 {
                self.can_castle.black_can_long_castle = false;
            }
            if from.0 == 0 && from.1 == 7 {
                self.can_castle.black_can_short_castle = false;
            }
        }
        if self.next_to_move == Color::WHITE {
            if from.0 == 7 && from.1 == 0 {
                self.can_castle.white_can_long_castle = false;
            }
            if from.0 == 7 && from.1 == 7 {
                self.can_castle.white_can_short_castle = false;
            }
        }
    }
    fn make_king_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        // Check if castling move
        match (from, to) {
            ((0, 4), (0, 6)) => {
                self.field[0][5] = self.field[0][7];
                self.field[0][7] = None;
                self.previous_move = "0-0".to_string();
            }
            ((0, 4), (0, 2)) => {
                self.field[0][3] = self.field[0][0];
                self.field[0][0] = None;
                self.previous_move = "0-0-0".to_string();
            }
            ((7, 4), (7, 6)) => {
                self.field[7][5] = self.field[7][7];
                self.field[7][7] = None;
                self.previous_move = "0-0".to_string();
            }
            ((7, 4), (7, 2)) => {
                self.field[7][3] = self.field[7][0];
                self.field[7][0] = None;
                self.previous_move = "0-0-0".to_string();
            }
            _ => (),
        }

        // Take away castling rights
        if self.next_to_move == Color::BLACK {
            self.can_castle.black_can_long_castle = false;
            self.can_castle.black_can_short_castle = false;
        } else {
            self.can_castle.white_can_long_castle = false;
            self.can_castle.white_can_short_castle = false;
        }
    }
    fn make_pawn_move<'a>(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        promotion_piece: Option<Piece>,
    ) -> Result<(), &'a str> {
        // Check if promotion move
        if to.0 == 7 || to.0 == 0 {
            match promotion_piece {
                None => return Err("No promotion piece specified for promotion"),
                Some(prom_piece) => {
                    self.field[to.0][to.1].unwrap().piece = prom_piece;
                    // TODO
                    self.previous_move.push_str("=");
                }
            }
        }

        // Check if en passant
        if from.1 != to.1 && self.field[to.0][to.1].is_none() {
            self.field[to.0][from.1] = None;
            self.previous_move.insert(0, 'x');
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
