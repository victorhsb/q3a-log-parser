use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub joined: bool,
}

impl Player {
    pub fn new(id: u32) -> Player {
        Player {
            id,
            name: String::new(),
            joined: false,
        }
    }
}
