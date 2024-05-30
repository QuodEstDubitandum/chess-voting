pub mod chess_piece;
pub mod validation;

use crate::game::chess_piece::{ChessPiece, Color, Piece};
use crate::utils::convert_notation::get_squares_from_notation;
use crate::utils::error::{CHECK_ERROR, NO_PIECE_SELECTED_ERROR, PROMOTION_ERROR};
use uuid::Uuid;

use self::validation::bishop::validate_bishop_move;
use self::validation::check_mate::can_be_captured_by;
use self::validation::king::validate_king_move;
use self::validation::knight::validate_knight_move;
use self::validation::pawn::validate_pawn_move;
use self::validation::queen::validate_queen_move;
use self::validation::rook::validate_rook_move;

#[derive(Clone)]
pub struct Game {
    pub id: Uuid,
    pub next_to_move: Color,
    pub previous_move: String,
    pub can_castle: CastlingRights,
    pub can_en_passant: bool,
    pub king_position: KingPosition,
    pub field: Vec<Vec<Option<ChessPiece>>>,
}

#[derive(Clone)]
pub struct CastlingRights {
    pub white_can_short_castle: bool,
    pub white_can_long_castle: bool,
    pub black_can_short_castle: bool,
    pub black_can_long_castle: bool,
}

#[derive(Clone)]
pub struct KingPosition {
    pub white_king_position: (usize, usize),
    pub black_king_position: (usize, usize),
}

impl Game {
    pub fn new() -> Game {
        create_new_game()
    }
    pub fn validate_and_make_move(
        &mut self,
        algebraic_from: &str,
        algebraic_to: &str,
        promotion_piece: Option<Piece>,
    ) -> Result<(), &'static str> {
        self.validate_move(algebraic_from, algebraic_to, promotion_piece)?;
        self.make_move(algebraic_from, algebraic_to, promotion_piece);

        Ok(())
    }
    pub fn validate_move(
        &self,
        algebraic_from: &str,
        algebraic_to: &str,
        promotion_piece: Option<Piece>,
    ) -> Result<(), &'static str> {
        let (from, to) = get_squares_from_notation(algebraic_from, algebraic_to)?;

        // check if the move is valid
        match self.field[from.0][from.1] {
            None => return Err(NO_PIECE_SELECTED_ERROR),
            Some(x) => match x.piece {
                Piece::BISHOP => validate_bishop_move(from, to, &self)?,
                Piece::ROOK => validate_rook_move(from, to, &self)?,
                Piece::QUEEN => validate_queen_move(from, to, &self)?,
                Piece::KNIGHT => validate_knight_move(from, to, &self)?,
                Piece::PAWN => validate_pawn_move(from, to, promotion_piece, &self)?,
                Piece::KING => validate_king_move(from, to, &self)?,
            },
        };

        // check if the move would put your king in check
        let mut game_clone = self.clone();
        game_clone.make_move(algebraic_from, algebraic_to, promotion_piece);
        match game_clone.next_to_move {
            Color::WHITE => {
                let checking_pieces = can_be_captured_by(
                    Color::BLACK,
                    game_clone.king_position.white_king_position,
                    game_clone,
                );
                if checking_pieces.len() != 0 {
                    return Err(CHECK_ERROR);
                }
            }
            Color::BLACK => {
                let checking_pieces = can_be_captured_by(
                    Color::WHITE,
                    game_clone.king_position.black_king_position,
                    game_clone,
                );
                if checking_pieces.len() != 0 {
                    return Err(CHECK_ERROR);
                }
            }
        }

        Ok(())
    }
    pub fn make_move(
        &mut self,
        algebraic_from: &str,
        algebraic_to: &str,
        promotion_piece: Option<Piece>,
    ) {
        // we can unwrap here since we perform this function in the validation function as well
        let (from, to) = get_squares_from_notation(algebraic_from, algebraic_to).unwrap();
        self.can_en_passant = false;

        // Add x in case we capture
        let move_with_capture = self.field[to.0][to.1].is_some();
        if move_with_capture {
            self.previous_move = "x".to_string();
        } else {
            self.previous_move = "".to_string();
        }

        // Target square
        self.previous_move.push_str(algebraic_to);

        match self.field[from.0][from.1].unwrap().piece {
            Piece::KING => self.make_king_move(from, to),
            Piece::PAWN => self.make_pawn_move(from, to, promotion_piece),
            Piece::QUEEN => self.previous_move.insert(0, 'Q'),
            Piece::ROOK => self.make_rook_move(from),
            Piece::BISHOP => self.previous_move.insert(0, 'B'),
            Piece::KNIGHT => self.previous_move.insert(0, 'N'),
        }

        // Move to new square
        self.field[to.0][to.1] = self.field[from.0][from.1];
        self.field[from.0][from.1] = None;
        match self.next_to_move {
            Color::BLACK => self.next_to_move = Color::WHITE,
            Color::WHITE => self.next_to_move = Color::BLACK,
        }
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

        // Change king position
        match self.next_to_move {
            Color::BLACK => self.king_position.black_king_position = to,
            Color::WHITE => self.king_position.white_king_position = to,
        }
    }
    fn make_pawn_move(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        promotion_piece: Option<Piece>,
    ) {
        // Check if promotion move
        if to.0 == 7 || to.0 == 0 {
            self.field[to.0][to.1].unwrap().piece = promotion_piece.unwrap();
            // TODO
            self.previous_move.push_str("=");
        }

        // Check if en passant
        if from.1 != to.1 && self.field[to.0][to.1].is_none() {
            self.field[from.0][to.1] = None;
            self.previous_move.insert(0, 'x');
        }

        // Set en passant rights if pawn moved 2 squares
        if (from.0 as i32 - to.0 as i32).abs() == 2 {
            self.can_en_passant = true;
        }
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
        can_en_passant: false,
        king_position: {
            KingPosition {
                white_king_position: (7, 4),
                black_king_position: (0, 4),
            }
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
