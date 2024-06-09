use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use chess_voting::{
    db::{Move, DB},
    game::{chess_piece::Color, Game, GameResult},
    utils::{
        request::{FinishRequest, MoveRequest},
        response::serialize_field,
    },
};

struct Server {
    pub game: Mutex<Game>,
    pub db: DB,
}
impl Server {
    pub async fn new() -> Server {
        Server {
            game: Mutex::new(Game::new()),
            db: DB::new().await,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = web::Data::new(Server::new().await);
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .service(health)
            .service(make_move)
            .service(get_game_history)
            .service(finish_game)
            .service(get_game_state)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}

#[get("/game/history")]
async fn get_game_history(server: web::Data<Server>) -> Result<impl Responder> {
    let game = server.game.lock().unwrap();
    let moves: Vec<Move> = server.db.get_moves(&game.id.to_string()).await;
    Ok(web::Json(moves))
}

#[get("/game/current_state")]
async fn get_game_state(server: web::Data<Server>) -> Result<impl Responder> {
    let game = server.game.lock().unwrap();
    let state: Vec<Vec<String>> = serialize_field(&game.field);
    Ok(web::Json(state))
}

#[post("/game/finish")]
async fn finish_game(req: web::Json<FinishRequest>, server: web::Data<Server>) -> HttpResponse {
    let mut game = server.game.lock().unwrap();
    *game = Game::new();
    server.db.finish_game(&req.game_result, &req.game_id).await;
    server.db.create_game(&game.id.to_string()).await;
    HttpResponse::Ok().body("OK".to_string())
}

#[post("/game/vote")]
async fn make_vote(req: web::Json<MoveRequest>, server: web::Data<Server>) -> HttpResponse {
    let game = server.game.lock().unwrap();
    if let Err(e) = game.validate_move(&req.from, &req.to, req.promotion) {
        return HttpResponse::BadRequest().body(e);
    }

    // kinda hacky to create new game just to get the move notation, wouldve been better to just
    // create a get_notation method but it would have been quite annoying to factor in all the
    // possible cases
    let mut game_clone = game.clone();
    game_clone.make_move(&req.from, &req.to, req.promotion);
    server.db.vote(&game_clone.previous_move).await;
    HttpResponse::Ok().body("OK".to_string())
}

#[post("/game/move")]
async fn make_move(req: web::Json<MoveRequest>, server: web::Data<Server>) -> HttpResponse {
    let mut game = server.game.lock().unwrap();
    if let Err(e) = game.validate_and_make_move(&req.from, &req.to, req.promotion) {
        return HttpResponse::BadRequest().body(e);
    }

    let player_str: &str;
    match game.next_to_move {
        Color::WHITE => player_str = "BLACK",
        Color::BLACK => player_str = "WHITE",
    }

    server
        .db
        .insert_move(
            game.turn_number,
            &game.id.to_string(),
            &game.previous_move,
            player_str,
        )
        .await;
    println!("{:?} {:?}", game.next_to_move, game.previous_move);

    if let Some(result) = &game.game_result {
        match result {
            GameResult::BlackWon => {
                server.db.finish_game("0-1", &game.id.to_string()).await;
            }
            GameResult::WhiteWon => {
                server.db.finish_game("1-0", &game.id.to_string()).await;
            }
        }
        server.db.create_game(&game.id.to_string()).await;
        *game = Game::new();
    }

    HttpResponse::Ok().body("OK".to_string())
}
