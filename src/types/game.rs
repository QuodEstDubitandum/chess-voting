pub struct Game {
    pub field: Vec<Vec<Option<ChessPiece>>>,
}

#[derive(Clone)]
pub enum Piece {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
}

#[derive(Clone)]
pub enum Color {
    BLACK,
    WHITE,
}

#[derive(Clone)]
pub struct ChessPiece {
    pub piece: Piece,
    pub color: Color,
}

impl Game {
    pub fn new() -> Game {
        Game {
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
}

