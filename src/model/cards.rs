use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use enum_iterator::Sequence;
use crate::model::user::User;

#[derive(Debug, Hash, Serialize, PartialEq, Eq, Sequence, Copy, Clone)]
pub enum CardSymbol {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, Hash, Serialize, PartialEq, Eq, Sequence, Copy, Clone)]
pub enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}


#[derive(Debug, Hash, Serialize, PartialEq, Eq)]
pub struct Card {
    pub value: CardValue,
    pub symbol: CardSymbol,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CardInventory {
    pub cards_in_hand: Vec<Card>,
    pub combinations_on_table: Vec<CardCombination>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CardCombination {
    pub cards: HashMap<Card, Option<Uuid>>,
}
