use std::collections::HashMap;
use actix_web::{get, HttpResponse, post, Responder, web};
use log::{debug, error, info, Level, log_enabled, warn};
use uuid::{Uuid, uuid};

use crate::GameList;
use crate::model::cards::{Card, CardInventory, CardSymbol, CardValue};
use crate::model::game::Game;
use crate::model::user::{User, UserInput};

#[get("/games/{game_id}")]
pub async fn get_game(game_id: web::Path<String>, games: web::Data<GameList>) -> impl Responder {
    let game_id = game_id.into_inner();
    let search_id = match Uuid::parse_str(&game_id) {
        Ok(search_id) => search_id,
        Err(_) => {
            warn!("{} is not a valid uuid", game_id);
            return HttpResponse::BadRequest().finish();
        }
    };
    let games = games.lock().unwrap();
    let search_result = games.iter()
        .enumerate()
        .find(|&game| game.1.id == search_id);

    match search_result {
        Some((_, game)) => {
            info!("game {:?} found", &game);
            HttpResponse::Ok().json(game)
        }
        None => {
            warn!("game {} not found", game_id);
            return HttpResponse::NotFound().finish();
        }
    }
}

#[post("/games/new")]
pub async fn new_game(games: web::Data<GameList>) -> impl Responder {
    let created = Game::new();
    let response = HttpResponse::Created().json(&created);
    info!("Created {:?}", &created);
    games.lock().unwrap().push(created);
    response
}

#[post("/games/{game_id}")]
pub async fn join_game(game_id: web::Path<String>, user: web::Json<UserInput>, games: web::Data<GameList>) -> impl Responder {
    let game_id = game_id.into_inner();
    let search_id = match Uuid::parse_str(&game_id) {
        Ok(search_id) => search_id,
        Err(_) => {
            warn!("{} is not a valid uuid", game_id);
            return HttpResponse::BadRequest().finish();
        }
    };
    let mut games = games.lock().unwrap();
    let search_result = games.iter_mut()
        .enumerate()
        .find(|game| game.1.id == search_id);

    match search_result {
        Some((_, game)) => {
            info!("game {:?} found", game);
            let new_user: User = user.into_inner().into();
            game.add_user(new_user);
            HttpResponse::Ok().json(game)
        }
        None => {
            warn!("game {} not found", game_id);
            return HttpResponse::NotFound().finish();
        }
    }
}