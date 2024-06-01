use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use chess_voting::game::{chess_piece::Color, validation::check_mate::is_mate, Game, GameResult};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RequestBody {
    pub from: String,
    pub to: String,
    pub promotion: char,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game = web::Data::new(Mutex::new(Game::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(game.clone())
            .service(health)
            .service(make_move)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> Result<String> {
    Ok("OK".to_string())
}

#[post("/move")]
async fn make_move(req: web::Json<RequestBody>, data: web::Data<Mutex<Game>>) -> HttpResponse {
    let game = &mut data.lock().unwrap();
    if let Err(e) = game.validate_and_make_move(&req.from, &req.to, req.promotion) {
        return HttpResponse::BadRequest().body(e);
    }
    println!("{:?} {:?}", game.next_to_move, game.previous_move);

    if is_mate(game) {
        match game.next_to_move {
            Color::WHITE => {
                game.game_result = Some(GameResult::BlackWon);
            }
            Color::BLACK => {
                game.game_result = Some(GameResult::WhiteWon);
            }
        }
    }

    HttpResponse::Ok().body("OK".to_string())
}
