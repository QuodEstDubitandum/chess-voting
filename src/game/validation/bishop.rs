use crate::game::Game;

pub fn validate_bishop_move<'a>(
    from: (usize, usize),
    to: (usize, usize),
    game: &Game,
) -> Result<(), &'a str> {
    let row_diff = from.0 as i32 - to.0 as i32;
    let col_diff = from.1 as i32 - to.1 as i32;

    // not even diagonal move
    if row_diff != col_diff || row_diff == 0 {
        return Err("Invalid bishop move");
    }

    // is there a piece in the way?
    let row_direction_sign = row_diff / -row_diff.abs();
    let col_direction_sign = col_diff / -col_diff.abs();
    for i in 1..row_diff.abs() {
        if game.field[(from.0 as i32 + i * row_direction_sign) as usize]
            [(from.1 as i32 + i * col_direction_sign) as usize]
            .is_some()
        {
            return Err("There is a piece in the way of your bishop move");
        };
    }

    // if you capture a piece, is it of the opposite color?
    if let Some(piece) = game.field[to.0][to.1] {
        if piece.color == game.next_to_move {
            return Err("You cannot capture your own piece");
        }
    }

    Ok(())
}
