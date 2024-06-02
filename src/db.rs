use libsql::{de, params, Builder, Connection, Rows};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::chess_piece::Color;

pub async fn connect_db() -> Connection {
    let db = Builder::new_local("local.db")
        .build()
        .await
        .expect("Could not connect local database");
    let conn = db.connect().unwrap();

    conn
}

pub async fn seed_db(db: &Connection) {
    db.execute_batch(
        r#"
    DROP TABLE IF EXISTS Move;
    DROP TABLE IF EXISTS Game;

    CREATE TABLE IF NOT EXISTS Game(
    game_id VARCHAR(255) PRIMARY KEY,
    result VARCHAR(10)
    );

    CREATE TABLE IF NOT EXISTS Move(
    move_id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id VARCHAR(255),
    turn INTEGER,
    player VARCHAR(10),
    move_notation VARCHAR(10),
    FOREIGN KEY(game_id) REFERENCES Game(game_id)
    );
    "#,
    )
    .await
    .expect("Cant seed DB");
}

pub struct DB {
    pub conn: Connection,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Move {
    pub move_notation: String,
    pub turn: u32,
    pub player: String,
}

impl DB {
    pub async fn new() -> DB {
        let conn = connect_db().await;
        seed_db(&conn).await;
        DB { conn }
    }
    pub async fn create_game(&self, id: &str) {
        self.conn
            .execute(
                "INSERT INTO Game(game_id, result) VALUES(?1, null)",
                params![id],
            )
            .await
            .expect("Could not create a new game");
    }
    pub async fn finish_game(&self, result: &str, id: &str) {
        self.conn
            .execute(
                "UPDATE TABLE Game SET result = ?1 WHERE game_id = ?2",
                params![result, id],
            )
            .await
            .expect("Could not create a new game");
    }
    pub async fn insert_move(&self, turn: u32, id: &str, new_move: &str, player: &str) {
        self.conn
            .execute(
                "INSERT INTO Move(turn, move_notation, player, game_id) VALUES(?1, ?2, ?3, ?4)",
                params![turn, new_move, player, id],
            )
            .await
            .expect("Could not insert move");
    }
    pub async fn get_moves(&self, id: &str) -> Vec<Move> {
        let mut rows: Rows = self
            .conn
            .query(
                "SELECT * FROM Move WHERE game_id = ?1 ORDER BY turn",
                params![id],
            )
            .await
            .expect("Could not get moves");

        let mut moves: Vec<Move> = vec![];
        while let Some(row) = rows.next().await.unwrap() {
            moves.push(de::from_row::<Move>(&row).unwrap());
        }

        println!("{:?}", moves);

        moves
    }
}
