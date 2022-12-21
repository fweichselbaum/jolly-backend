use std::collections::{HashMap};
use std::iter::Map;
use std::sync::{Arc, Mutex};

use enum_iterator::all;
use log::info;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::cards::{Card, CardInventory, CardSymbol, CardValue};
use crate::model::cards::CardValue::{Ace, Joker};
use crate::model::user::User;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: Uuid,
    pub users: Vec<User>,
    pub player_cards: HashMap<Uuid, CardInventory>,
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        let mut draw_pile = vec![];
        for value in all::<CardValue>() {
            if value == Joker {
                draw_pile.push(Card { value, symbol: CardSymbol::Hearts });
                draw_pile.push(Card { value, symbol: CardSymbol::Hearts });
                draw_pile.push(Card { value, symbol: CardSymbol::Spades });
                draw_pile.push(Card { value, symbol: CardSymbol::Spades });
                continue;
            }
            for symbol in all::<CardSymbol>() {
                draw_pile.push(Card { value, symbol });
                draw_pile.push(Card { value, symbol });
            }
        }
        draw_pile.shuffle(&mut thread_rng());
        Self {
            id: Uuid::new_v4(),
            users: vec![],
            player_cards: HashMap::new(),
            discard_pile: vec![],
            draw_pile,
        }
    }

    pub fn add_user(&mut self, user: User) {
        let mut user_cards = vec![];
        self.draw_pile.drain(..=12).for_each(|card| user_cards.push(card));
        self.player_cards.insert(user.id, CardInventory {
            cards_in_hand: user_cards,
            combinations_on_table: vec![]
        });
        self.users.push(user);
    }
}

pub type GameList = Arc<Mutex<Vec<Game>>>;

impl PartialEq<Self> for Game {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Game {}

