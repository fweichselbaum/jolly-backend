use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    pub name: String,
    pub avatar_id: u8,
}

#[derive(Debug, Serialize, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub avatar_id: u8,
}

impl From<UserInput> for User {
    fn from(input: UserInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: input.name,
            avatar_id: input.avatar_id,
        }
    }
}

pub type UserList = Arc<Mutex<Vec<User>>>;

impl PartialEq<Self> for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

