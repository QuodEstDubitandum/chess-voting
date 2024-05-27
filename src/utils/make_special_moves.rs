use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::game::chess_piece::{ChessPiece, Piece};

static PROMOTION_MAP: Lazy<Mutex<HashMap<char, Piece>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('K', Piece::KING);
    map.insert('Q', Piece::QUEEN);
    map.insert('R', Piece::ROOK);
    map.insert('B', Piece::BISHOP);
    map.insert('N', Piece::KNIGHT);
    Mutex::new(map)
});
