use crate::game::chess_piece::{ChessPiece, Color, Piece};

pub fn serialize_field(field: &Vec<Vec<Option<ChessPiece>>>) -> Vec<Vec<String>> {
    let mut serialized_fields: Vec<Vec<String>> =
        vec![vec!["".to_string(); field.len()]; field[0].len()];

    for i in 0..field.len() {
        for j in 0..field[0].len() {
            match field[i][j] {
                None => (),
                Some(chess_piece) => {
                    match chess_piece.color {
                        Color::WHITE => serialized_fields[i][j].push('W'),
                        Color::BLACK => serialized_fields[i][j].push('B'),
                    }
                    match chess_piece.piece {
                        Piece::KING => serialized_fields[i][j].push('K'),
                        Piece::QUEEN => serialized_fields[i][j].push('Q'),
                        Piece::ROOK => serialized_fields[i][j].push('R'),
                        Piece::BISHOP => serialized_fields[i][j].push('B'),
                        Piece::KNIGHT => serialized_fields[i][j].push('N'),
                        _ => (),
                    }
                }
            }
        }
    }

    serialized_fields
}
