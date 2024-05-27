use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::chess_piece::{ChessPiece, Piece};

pub fn make_king_move(
    from: (usize, usize),
    to: (usize, usize),
    field: &mut Vec<Vec<Option<ChessPiece>>>,
) {
    match (from, to) {
        ((0, 4), (0, 6)) => {
            field[0][5] = field[0][7];
            field[0][7] = None;
        }
        ((0, 4), (0, 2)) => {
            field[0][3] = field[0][0];
            field[0][0] = None;
        }
        ((7, 4), (7, 6)) => {
            field[7][5] = field[7][7];
            field[7][7] = None;
        }
        ((7, 4), (7, 2)) => {
            field[7][3] = field[7][0];
            field[7][0] = None;
        }
        _ => (),
    }
}

static PROMOTION_MAP: Lazy<Mutex<HashMap<char, Piece>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('K', Piece::KING);
    map.insert('Q', Piece::QUEEN);
    map.insert('R', Piece::ROOK);
    map.insert('B', Piece::BISHOP);
    map.insert('N', Piece::KNIGHT);
    Mutex::new(map)
});
