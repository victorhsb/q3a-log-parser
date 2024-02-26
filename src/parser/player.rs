use std::fmt::Display;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub kill_score: i32,
}

impl Player {
    pub fn new(id: u32) -> Player {
        Player {
            id,
            name: String::new(),
            kill_score: 0,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.kill_score)
    }
}
