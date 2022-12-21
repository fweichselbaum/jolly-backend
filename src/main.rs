use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use api::game::{get_game, new_game};
use crate::api::game::join_game;

use crate::model::game::GameList;

pub mod model;
pub mod api;
pub mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let games = GameList::default();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_header()
                .allow_any_method())
            .app_data(Data::new(games.clone()))
            .service(new_game)
            .service(get_game)
            .service(join_game)
            .service(Files::new("/", "./static").show_files_listing())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}