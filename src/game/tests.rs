#[cfg(test)]
mod test_game {
    use crate::{
        game::chess_piece::{Color, Piece},
        game::{CastlingRights, ChessPiece},
    };

    use crate::game::Game;

    #[test]
    fn test_castle_move() {
        let mut game = Game::new();
        game.field[7][3] = None;
        game.field[7][2] = None;
        game.field[7][1] = None;
        let val = game.make_move("e1", "c1", None);
        if let Err(e) = val {
            panic!("Expected castle move to be performed, got {:?}", e);
        }

        assert_eq!(
            game.field[7][2],
            Some(ChessPiece {
                color: Color::WHITE,
                piece: Piece::KING
            })
        );
        assert_eq!(
            game.field[7][3],
            Some(ChessPiece {
                color: Color::WHITE,
                piece: Piece::ROOK
            })
        );

        assert_eq!(game.previous_move, "0-0-0");
    }

    #[test]
    fn test_castle_move_with_piece_in_the_way() {
        let mut game = Game::new();
        game.field[7][5] = None;
        let val = game.make_move("e1", "g1", None);
        if val.is_ok() {
            panic!("Expected castle move to fail due to a piece being in the way");
        }
    }

    #[test]
    fn test_castle_move_with_pieces_on_wrong_positions() {
        let mut game = Game::new();
        game.field[0][5] = None;
        game.field[0][6] = None;
        game.field[0][7] = None;
        let val = game.make_move("e8", "g8", None);
        if val.is_ok() {
            panic!("Expected castle move to fail due to pieces being on the wrong position");
        }
    }

    #[test]
    fn test_castle_move_after_piece_movement() {
        let mut game = Game::new();
        game.can_castle = CastlingRights {
            white_can_long_castle: true,
            white_can_short_castle: false,
            black_can_short_castle: true,
            black_can_long_castle: true,
        };
        game.field[0][5] = None;
        game.field[0][6] = None;
        let val = game.make_move("e8", "g8", None);
        if val.is_ok() {
            panic!("Expected castle move to fail due to pieces having already moved before");
        }
    }

    #[test]
    fn test_bishop_move() {
        let mut game = Game::new();
        game.field[6][4] = None;
        let val = game.make_move("f1", "b5", None);
        if let Err(e) = val {
            panic!("Expected bishop move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[7][5], None);
        assert_eq!(
            game.field[3][1],
            Some({
                ChessPiece {
                    piece: Piece::BISHOP,
                    color: Color::WHITE,
                }
            })
        );
        assert_eq!(game.previous_move, "Bb5");
    }

    #[test]
    fn test_bishop_move_with_capture() {
        let mut game = Game::new();
        game.field[6][4] = None;
        game.field[3][1] = Some(ChessPiece {
            piece: Piece::PAWN,
            color: Color::BLACK,
        });
        let val = game.make_move("f1", "b5", None);
        if let Err(e) = val {
            panic!("Expected bishop move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[7][5], None);
        assert_eq!(
            game.field[3][1],
            Some({
                ChessPiece {
                    piece: Piece::BISHOP,
                    color: Color::WHITE,
                }
            })
        );
        assert_eq!(game.previous_move, "Bxb5");
    }

    #[test]
    fn test_bishop_move_with_wrong_capture() {
        let mut game = Game::new();
        game.field[6][4] = None;
        game.field[3][1] = Some(ChessPiece {
            piece: Piece::PAWN,
            color: Color::WHITE,
        });
        let val = game.make_move("f1", "b5", None);
        if val.is_ok() {
            panic!("Expected bishop move to fail due to your own piece being captured");
        }
    }

    #[test]
    fn test_bishop_move_with_piece_in_the_way() {
        let mut game = Game::new();
        let val = game.make_move("f1", "b5", None);
        if val.is_ok() {
            panic!("Expected bishop move to fail due to a piece being in the way");
        }
    }

    #[test]
    fn test_rook_move() {
        let mut game = Game::new();
        game.field[6][0] = None;
        let val = game.make_move("a1", "a5", None);
        if let Err(e) = val {
            panic!("Expected rook move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[7][0], None);
        assert_eq!(
            game.field[3][0],
            Some({
                ChessPiece {
                    piece: Piece::ROOK,
                    color: Color::WHITE,
                }
            })
        );
        assert_eq!(game.previous_move, "Ra5");
        assert_eq!(game.can_castle.white_can_long_castle, false);
    }

    #[test]
    fn test_rook_move_with_capture() {
        let mut game = Game::new();
        game.field[1][0] = None;
        game.field[5][0] = Some(ChessPiece {
            piece: Piece::ROOK,
            color: Color::WHITE,
        });
        game.next_to_move = Color::BLACK;
        let val = game.make_move("a8", "a3", None);
        if let Err(e) = val {
            panic!("Expected rook move to be performed, got {:?}", e);
        }

        assert_eq!(game.field[0][0], None);
        assert_eq!(
            game.field[5][0],
            Some({
                ChessPiece {
                    piece: Piece::ROOK,
                    color: Color::BLACK,
                }
            })
        );
        assert_eq!(game.previous_move, "Rxa3");
        assert_eq!(game.can_castle.black_can_long_castle, false);
        assert_eq!(game.can_castle.white_can_long_castle, true);
    }

    #[test]
    fn test_rook_move_with_wrong_capture() {
        let mut game = Game::new();
        game.field[7][0] = None;
        game.field[3][0] = Some(ChessPiece {
            piece: Piece::PAWN,
            color: Color::WHITE,
        });
        let val = game.make_move("a1", "a5", None);
        if val.is_ok() {
            panic!("Expected rook move to fail due to your own piece being captured");
        }
    }

    #[test]
    fn test_rook_move_with_piece_in_the_way() {
        let mut game = Game::new();
        let val = game.make_move("a1", "a5", None);
        if val.is_ok() {
            panic!("Expected rook move to fail due to a piece being in the way");
        }
    }
}
