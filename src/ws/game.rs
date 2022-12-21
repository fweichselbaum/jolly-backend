use actix::{Actor, StreamHandler, Message, Addr};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::model::cards::{Card, CardInventory};
use crate::model::game::Game;

pub struct GameWs;

impl Actor for GameWs {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Message)]
//#[rtype(result = "()")]
pub struct UserJoin {
    pub game_id: Uuid,
    pub user_id: Uuid,
}

/*#[derive(Message)]
//#[rtype(result = "()")]
pub struct User {
    pub game_id: Uuid,
    pub user_id: Uuid,
    pub player_cards: CardInventory,
    pub draw_pile: Option<Vec<Card>>,
    pub discard_pile: Vec<Card>,
}*/

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}