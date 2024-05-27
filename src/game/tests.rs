#[cfg(test)]
mod test_special_moves {
    use crate::{
        chess_piece::{Color, Piece},
        game::ChessPiece,
    };

    use crate::game::Game;

    #[test]
    fn test_correct_castle_move() {
        let mut game = Game::new();
        game.field[7][5] = None;
        game.field[7][6] = None;
        let val = game.make_move("e1", "g1", 'Q');
        if let Err(e) = val {
            panic!("Expected castle move to be performed, got {:?}", e);
        }

        assert_eq!(
            game.field[7][6],
            Some(ChessPiece {
                color: Color::WHITE,
                piece: Piece::KING
            })
        );
        assert_eq!(
            game.field[7][5],
            Some(ChessPiece {
                color: Color::WHITE,
                piece: Piece::ROOK
            })
        );
    }
    fn test_castle_move_with_piece_in_the_way() {}
    fn test_castle_move_with_pieces_on_wrong_positions() {}
    fn test_castle_move_after_piece_movement() {}
}
