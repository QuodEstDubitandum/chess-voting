use crate::game::{
    chess_piece::{ChessPiece, Color, Piece},
    Game,
};

pub struct CapturePiece {
    pub row: usize,
    pub col: usize,
    pub piece: Piece,
}

pub fn can_be_captured_by(
    enemy_color: Color,
    square: (usize, usize),
    game: Game,
) -> Vec<CapturePiece> {
    let mut capturable_by = vec![];

    capturable_by_knight(enemy_color, square, &game, &mut capturable_by);
    capturable_by_diagonal_move(enemy_color, square, &game, &mut capturable_by);
    capturable_by_linear_move(enemy_color, square, &game, &mut capturable_by);

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
        if row >= 0
            && row <= 7
            && col >= 0
            && col <= 7
            && game.field[row as usize][col as usize]
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
    for dir in directions {
        'inner: for i in 1..8 {
            if row + i * dir.0 >= 0
                && row + i * dir.0 <= 7
                && col + i * dir.1 >= 0
                && col + i * dir.1 <= 7
                && game.field[square.0 + (i * dir.0) as usize][square.1 + (i * dir.1) as usize]
                    .is_some()
            {
                let piece = game.field[square.0 + (i * dir.0) as usize]
                    [square.1 + (i * dir.1) as usize]
                    .unwrap();
                if piece.color != enemy_color {
                    break 'inner;
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
                break 'inner;
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
    for dir in directions {
        'inner: for i in 1..8 {
            if row + i * dir.0 >= 0
                && row + i * dir.0 <= 7
                && col + i * dir.1 >= 0
                && col + i * dir.1 <= 7
                && game.field[square.0 + (i * dir.0) as usize][square.1 + (i * dir.1) as usize]
                    .is_some()
            {
                let piece = game.field[square.0 + (i * dir.0) as usize]
                    [square.1 + (i * dir.1) as usize]
                    .unwrap();
                if piece.color != enemy_color {
                    break 'inner;
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
                break 'inner;
            }
        }
    }
}

pub fn is_mate(color: Color, game: Game) -> bool {
    false
}
