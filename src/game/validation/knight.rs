use crate::game::Game;

pub fn validate_knight_move<'a>(
    from: (usize, usize),
    to: (usize, usize),
    game: &Game,
) -> Result<(), &'a str> {
    let row_diff = (from.0 as i32 - to.0 as i32).abs();
    let col_diff = (from.1 as i32 - to.1 as i32).abs();

    // not even move
    match (row_diff, col_diff) {
        (1, 2) | (2, 1) => (),
        _ => return Err("Invalid knight move"),
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
mod test_knight {
    use crate::{
        game::chess_piece::{Color, Piece},
        game::ChessPiece,
        game::Game,
    };

    #[test]
    fn test_knight_move() {
        let mut game = Game::new();
        let val = game.make_move("b1", "c3", None);
        if let Err(e) = val {
            panic!("Expected knight move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[7][1], None);
        assert_eq!(
            game.field[5][2],
            Some({
                ChessPiece {
                    piece: Piece::KNIGHT,
                    color: Color::WHITE,
                }
            })
        );
        assert_eq!(game.previous_move, "Nc3");
    }

    #[test]
    fn test_knight_move_with_capture() {
        let mut game = Game::new();
        game.field[6][3] = Some(ChessPiece {
            piece: Piece::ROOK,
            color: Color::BLACK,
        });
        let val = game.make_move("b1", "d2", None);
        if let Err(e) = val {
            panic!("Expected knight move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[7][1], None);
        assert_eq!(
            game.field[6][3],
            Some({
                ChessPiece {
                    piece: Piece::KNIGHT,
                    color: Color::WHITE,
                }
            })
        );
        assert_eq!(game.previous_move, "Nxd2");
    }

    #[test]
    fn test_knight_move_with_wrong_capture() {
        let mut game = Game::new();
        let val = game.make_move("b1", "d2", None);
        if val.is_ok() {
            panic!("Expected knight move to fail due to your own piece being captured");
        }
    }
}