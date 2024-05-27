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

pub fn make_pawn_move<'a>(
    from: (usize, usize),
    to: (usize, usize),
    promotion_piece: char,
    field: &'a mut Vec<Vec<Option<ChessPiece>>>,
) -> Result<(), &'a str> {
    if to.0 == 7 || to.0 == 0 {
        let promotion_mapping = PROMOTION_MAP.lock().unwrap();

        field[to.0][to.1].unwrap().piece = *promotion_mapping
            .get(&promotion_piece)
            .ok_or("Invalid promotion piece")?;
    }
    if from.1 != to.1 {
        field[to.0][from.1] = None;
    }

    Ok(())
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
