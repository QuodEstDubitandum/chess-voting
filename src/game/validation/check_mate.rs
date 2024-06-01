use crate::{
    game::{
        chess_piece::{ChessPiece, Color, Piece},
        Game,
    },
    utils::is_in_bounds,
};

pub struct CapturePiece {
    pub row: usize,
    pub col: usize,
    pub piece: Piece,
}

pub fn can_king_be_captured_after_move(
    game: &Game,
    algebraic_from: &str,
    algebraic_to: &str,
    promotion_ch: char,
) -> Vec<CapturePiece> {
    let mut game_clone = game.clone();
    game_clone.make_move(algebraic_from, algebraic_to, promotion_ch);
    match game_clone.next_to_move {
        Color::BLACK => can_be_captured_by(
            Color::BLACK,
            game_clone.king_position.white_king_position,
            &game_clone,
        ),
        Color::WHITE => can_be_captured_by(
            Color::WHITE,
            game_clone.king_position.black_king_position,
            &game_clone,
        ),
    }
}

pub fn can_be_captured_by(
    enemy_color: Color,
    square: (usize, usize),
    game: &Game,
) -> Vec<CapturePiece> {
    let mut capturable_by = vec![];

    capturable_by_knight(enemy_color, square, game, &mut capturable_by);
    capturable_by_diagonal_move(enemy_color, square, game, &mut capturable_by);
    capturable_by_linear_move(enemy_color, square, game, &mut capturable_by);

    capturable_by
}

fn capturable_by_knight(
    enemy_color: Color,
    square: (usize, usize),
    game: &Game,
    capturable_by: &mut Vec<CapturePiece>,
) {
    let row = square.0 as i32;
    let col = square.1 as i32;

    let knight_squares = vec![
        (row + 2, col + 1),
        (row + 2, col - 1),
        (row + 1, col + 2),
        (row + 1, col - 2),
        (row - 2, col + 1),
        (row - 2, col - 1),
        (row - 1, col + 2),
        (row - 1, col - 2),
    ];

    for (row, col) in knight_squares {
        if !is_in_bounds(row, col) {
            continue;
        }

        if game.field[row as usize][col as usize]
            == Some(ChessPiece {
                piece: Piece::KNIGHT,
                color: enemy_color,
            })
        {
            capturable_by.push(CapturePiece {
                row: row as usize,
                col: col as usize,
                piece: Piece::KNIGHT,
            });
        }
    }
}

fn capturable_by_diagonal_move(
    enemy_color: Color,
    square: (usize, usize),
    game: &Game,
    capturable_by: &mut Vec<CapturePiece>,
) {
    let row = square.0 as i32;
    let col = square.1 as i32;

    let directions = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
    'outer: for dir in directions {
        for i in 1..8 {
            if !is_in_bounds(row + i * dir.0, col + i * dir.1) {
                continue 'outer;
            }

            if let Some(piece) = game.field[(row + i * dir.0) as usize][(col + i * dir.1) as usize]
            {
                if piece.color != enemy_color {
                    continue 'outer;
                }

                match piece.piece {
                    Piece::QUEEN => {
                        capturable_by.push(CapturePiece {
                            row: row as usize,
                            col: col as usize,
                            piece: Piece::QUEEN,
                        });
                    }
                    Piece::BISHOP => {
                        capturable_by.push(CapturePiece {
                            row: row as usize,
                            col: col as usize,
                            piece: Piece::BISHOP,
                        });
                    }
                    Piece::PAWN => {
                        if i == 1 {
                            match (dir.0, dir.1, enemy_color) {
                                (-1, 1, Color::BLACK)
                                | (-1, -1, Color::BLACK)
                                | (1, 1, Color::WHITE)
                                | (1, -1, Color::WHITE) => {
                                    capturable_by.push(CapturePiece {
                                        row: row as usize,
                                        col: col as usize,
                                        piece: Piece::PAWN,
                                    });
                                }
                                _ => (),
                            }
                        }
                    }
                    Piece::KING => {
                        if i == 1 {
                            capturable_by.push(CapturePiece {
                                row: row as usize,
                                col: col as usize,
                                piece: Piece::KING,
                            });
                        }
                    }
                    _ => (),
                }
                continue 'outer;
            }
        }
    }
}

fn capturable_by_linear_move(
    enemy_color: Color,
    square: (usize, usize),
    game: &Game,
    capturable_by: &mut Vec<CapturePiece>,
) {
    let row = square.0 as i32;
    let col = square.1 as i32;

    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    'outer: for dir in directions {
        for i in 1..8 {
            if !is_in_bounds(row + i * dir.0, col + i * dir.1) {
                continue 'outer;
            }

            if let Some(piece) = game.field[(row + i * dir.0) as usize][(col + i * dir.1) as usize]
            {
                if piece.color != enemy_color {
                    continue 'outer;
                }

                match piece.piece {
                    Piece::QUEEN => {
                        capturable_by.push(CapturePiece {
                            row: row as usize,
                            col: col as usize,
                            piece: Piece::QUEEN,
                        });
                    }
                    Piece::ROOK => {
                        capturable_by.push(CapturePiece {
                            row: row as usize,
                            col: col as usize,
                            piece: Piece::ROOK,
                        });
                    }
                    Piece::KING => {
                        if i == 1 {
                            capturable_by.push(CapturePiece {
                                row: row as usize,
                                col: col as usize,
                                piece: Piece::KING,
                            });
                        }
                    }
                    _ => (),
                }
                continue 'outer;
            }
        }
    }
}
