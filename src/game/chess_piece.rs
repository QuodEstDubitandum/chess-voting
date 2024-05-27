#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChessPiece {
    pub piece: Piece,
    pub color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum Piece {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    BLACK,
    WHITE,
}
