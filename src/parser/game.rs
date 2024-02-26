use crate::parser::player::Player;
use std::collections::HashMap;
use std::fmt::Display;
use serde::Serialize;

const WORLD: u32 = 1022;

#[derive(Clone, Debug, Serialize)]
pub struct Game {
    pub total_kills: u32,
    #[serde(rename="players")]
    pub player_list: Vec<String>,
    #[serde(rename="kills")]
    pub kill_score: HashMap<String, i32>,
    pub means_of_death: Vec<(u32, u32)>,
    #[serde(skip_serializing)]
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            total_kills: 0,
            player_list: Vec::new(),
            kill_score: HashMap::new(),
            means_of_death: Vec::new(),
            players: Vec::new(),
        }
    }

    pub fn new_player(&mut self, id: u32) {
        self.players.push(Player::new(id));
    }

    pub fn rename_player(&mut self, id: u32, name: String) {
        match self.players.iter_mut().find(|p| p.id == id) {
            Some( p ) => { p.name = name },
            None => panic!("Player not found"),
        }
    }

    pub fn add_kill(&mut self, killer_id: u32, killed_id: u32, means_of_death: u32) {
        self.total_kills += 1;

        if killer_id != WORLD {
            match self.players.iter_mut().find(|p| p.id == killer_id) {
                Some(p) => p.kill_score += 1,
                None => (),
            }
        } else {
            match self.players.iter_mut().find(|p| p.id == killed_id) {
                Some(p) => p.kill_score -= 1,
                None => (),
            };
        }

        self.add_kill_mode(means_of_death);
    }

    fn add_kill_mode(&mut self, mode: u32) {
        for (kill_mode, kill_count) in self.means_of_death.iter_mut() {
            if *kill_mode == mode {
                *kill_count += 1;
                return;
            }
        }
        self.means_of_death.push((mode.clone(), 1));
    }

    pub fn finish_game(&mut self) {
        for player in &self.players {
            self.kill_score.insert(player.name.clone(), player.kill_score);
            self.player_list.push(player.name.clone());
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Total kills: {}\n", self.total_kills)?;
        write!(f, "Players: {:?}\n", self.players)?;
        write!(f, "Kills: {:?}\n", self.kill_score)?;
        Ok(())
    }
}

