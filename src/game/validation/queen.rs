use crate::game::Game;

pub fn validate_queen_move<'a>(
    from: (usize, usize),
    to: (usize, usize),
    game: &Game,
) -> Result<(), &'a str> {
    let row_diff = from.0 as i32 - to.0 as i32;
    let col_diff = from.1 as i32 - to.1 as i32;
    let row_direction_sign: i32;
    let col_direction_sign: i32;

    match (row_diff, col_diff) {
        (0, 0) => return Err("You cant move to the same square"),
        // check horizontal move
        (0, _) => {
            col_direction_sign = col_diff / -col_diff.abs();
            for i in 1..col_diff.abs() {
                if game.field[from.0][(from.1 as i32 + i * col_direction_sign) as usize].is_some() {
                    return Err("There is a piece in the way of your queen move");
                };
            }
        }
        // check vertical move
        (_, 0) => {
            row_direction_sign = row_diff / -row_diff.abs();
            for i in 1..row_diff.abs() {
                if game.field[(from.0 as i32 + i * row_direction_sign) as usize][from.1].is_some() {
                    return Err("There is a piece in the way of your queen move");
                };
            }
        }
        _ => {
            // check if move is not diagonal
            if row_diff.abs() != col_diff.abs() {
                return Err("Not a valid queen move");
            }

            row_direction_sign = row_diff / -row_diff.abs();
            col_direction_sign = col_diff / -col_diff.abs();
            for i in 1..row_diff.abs() {
                if game.field[(from.0 as i32 + i * row_direction_sign) as usize]
                    [(from.1 as i32 + i * col_direction_sign) as usize]
                    .is_some()
                {
                    return Err("There is a piece in the way of your queen move");
                };
            }
        }
    }

    // if you capture a piece, is it of the opposite color?
    if let Some(piece) = game.field[to.0][to.1] {
        if piece.color == game.next_to_move {
            return Err("You cannot capture your own piece");
        }
    }

    Ok(())
}

#[cfg(test)]
mod test_queen {
    use crate::{
        game::chess_piece::{Color, Piece},
        game::ChessPiece,
        game::Game,
    };

    #[test]
    fn test_queen_move() {
        let mut game = Game::new();
        game.field[6][4] = None;
        let val = game.make_move("d1", "g4", None);
        if let Err(e) = val {
            panic!("Expected queen move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[7][3], None);
        assert_eq!(
            game.field[4][6],
            Some({
                ChessPiece {
                    piece: Piece::QUEEN,
                    color: Color::WHITE,
                }
            })
        );
        assert_eq!(game.previous_move, "Qg4");
    }

    #[test]
    fn test_queen_move_with_capture() {
        let mut game = Game::new();
        game.field[6][3] = None;
        let val = game.make_move("d1", "d7", None);
        if let Err(e) = val {
            panic!("Expected queen move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[7][3], None);
        assert_eq!(
            game.field[1][3],
            Some({
                ChessPiece {
                    piece: Piece::QUEEN,
                    color: Color::WHITE,
                }
            })
        );
        assert_eq!(game.previous_move, "Qxd7");
    }

    #[test]
    fn test_queen_move_with_wrong_capture() {
        let mut game = Game::new();
        let val = game.make_move("d1", "d2", None);
        if val.is_ok() {
            panic!("Expected queen move to fail due to your own piece being captured");
        }
    }

    #[test]
    fn test_queen_move_with_piece_in_the_way() {
        let mut game = Game::new();
        game.field[6][3] = None;
        let val = game.make_move("d1", "d8", None);
        if val.is_ok() {
            panic!("Expected queen move to fail due to a piece being in the way");
        }
    }
}