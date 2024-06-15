use std::{env, sync::Mutex};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use chess_voting::{
    db::{Move, Vote, DB},
    game::{chess_piece::Color, Game, GameResult},
    utils::{
        request::{FinishRequest, MoveRequest},
        response::serialize_field,
    },
};
use dotenv::dotenv;
use log::{error, info};

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
    dotenv().ok();
    let url = env::var("URL").expect("Missing URL env var");
    let port = env::var("PORT").expect("Missing PORT env var");
    env_logger::Builder::from_default_env().init();
    info!("Server listening on port {}", port);

    let server = web::Data::new(Server::new().await);
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .service(health)
            .service(get_game_history)
            .service(get_game_state)
            .service(get_votes)
            .service(finish_game)
            .service(validate_move)
            .service(make_vote)
            .service(make_move)
    })
    .bind((url, port.parse::<u16>().unwrap()))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK".to_string())
}

#[get("/game/history")]
async fn get_game_history(server: web::Data<Server>) -> Result<impl Responder> {
    info!("Checking game history...");
    let game = server.game.lock().unwrap();
    let moves: Vec<Move> = server.db.get_moves(&game.id.to_string()).await;
    info!("Fetched moves from history, got {} moves", moves.len());
    Ok(web::Json(moves))
}

#[get("/game/current_state")]
async fn get_game_state(server: web::Data<Server>) -> Result<impl Responder> {
    info!("Checking current game state...");
    let game = server.game.lock().unwrap();
    let state: Vec<Vec<String>> = serialize_field(&game.field);
    info!("Fetched the game state");
    Ok(web::Json(state))
}

#[get("/game/current_votes")]
async fn get_votes(server: web::Data<Server>) -> Result<impl Responder> {
    info!("Checking game votes...");
    let votes: Vec<Vote> = server.db.get_votes().await;
    info!("Fetched game moves");
    Ok(web::Json(votes))
}

#[post("/game/finish")]
async fn finish_game(req: web::Json<FinishRequest>, server: web::Data<Server>) -> HttpResponse {
    info!("Finishing game...");
    let mut game = server.game.lock().unwrap();
    *game = Game::new();
    server.db.finish_game(&req.game_result, &req.game_id).await;
    info!("Finished DB game");
    server.db.create_game(&game.id.to_string()).await;
    info!("Created new DB game");
    HttpResponse::Ok().body("OK".to_string())
}

#[post("/game/validate")]
async fn validate_move(req: web::Json<MoveRequest>, server: web::Data<Server>) -> HttpResponse {
    info!("Validating move...");
    let game = server.game.lock().unwrap();
    if let Err(e) = game.validate_move(&req.from, &req.to, req.promotion) {
        error!("Not a valid move: {}", e);
        return HttpResponse::BadRequest().body(e);
    }
    info!("Move is valid");
    HttpResponse::Ok().body("OK".to_string())
}

#[post("/game/vote")]
async fn make_vote(req: web::Json<MoveRequest>, server: web::Data<Server>) -> HttpResponse {
    info!("Voting for a move...");
    let game = server.game.lock().unwrap();

    // kinda hacky to create new game just to get the move notation, wouldve been better to just
    // create a get_notation method but it would have been quite annoying to factor in all the
    // possible cases
    let mut game_clone = game.clone();
    game_clone.make_move(&req.from, &req.to, req.promotion);
    server.db.vote(&game_clone.previous_move).await;
    info!("Voted for move {}", &game_clone.previous_move);
    HttpResponse::Ok().body("OK".to_string())
}

#[post("/game/move")]
async fn make_move(req: web::Json<MoveRequest>, server: web::Data<Server>) -> HttpResponse {
    info!("Performing move...");
    let mut game = server.game.lock().unwrap();
    if let Err(e) = game.validate_and_make_move(&req.from, &req.to, req.promotion) {
        error!("Not a valid move: {}", e);
        return HttpResponse::BadRequest().body(e);
    }
    info!("Move {} is valid", &game.previous_move);

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
    info!("Inserted move into DB");

    if let Some(result) = &game.game_result {
        match result {
            GameResult::BlackWon => {
                info!("Black won, finishing game automatically...");
                server.db.finish_game("0-1", &game.id.to_string()).await;
            }
            GameResult::WhiteWon => {
                info!("White won, finishing game automatically...");
                server.db.finish_game("1-0", &game.id.to_string()).await;
            }
        }
        server.db.create_game(&game.id.to_string()).await;
        *game = Game::new();
        info!("Created new game");
    }

    HttpResponse::Ok().body("OK".to_string())
}
