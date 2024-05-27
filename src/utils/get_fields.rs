use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

pub fn get_fields<'a>(
    from: &'a str,
    to: &'a str,
) -> Result<((usize, usize), (usize, usize)), &'a str> {
    let field_to_col_mapping = FIELD_TO_COL_MAP.lock().unwrap();
    let from_col = field_to_col_mapping
        .get(
            &from
                .chars()
                .nth(0)
                .ok_or("From field needs to be at least 2 characters long")?,
        )
        .ok_or("Invalid field col in from field")?;
    let to_col = field_to_col_mapping
        .get(
            &to.chars()
                .nth(0)
                .ok_or("To field needs to be at least 2 characters long")?,
        )
        .ok_or("Invalid field col in to field")?;

    let from_row = 8 - from
        .chars()
        .nth(1)
        .ok_or("From field needs to be at least 2 characters long")?
        .to_digit(10)
        .ok_or("Invalid field row in from field")?;
    let to_row = 8 - to
        .chars()
        .nth(1)
        .ok_or("To field needs to be at least 2 characters long")?
        .to_digit(10)
        .ok_or("Invalid field row in to field")?;

    Ok(((from_row as usize, *from_col), (to_row as usize, *to_col)))
}

static FIELD_TO_COL_MAP: Lazy<Mutex<HashMap<char, usize>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('a', 0);
    map.insert('b', 1);
    map.insert('c', 2);
    map.insert('d', 3);
    map.insert('e', 4);
    map.insert('f', 5);
    map.insert('g', 6);
    map.insert('h', 7);
    Mutex::new(map)
});
