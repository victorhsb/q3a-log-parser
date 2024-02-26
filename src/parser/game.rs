use crate::parser::player::Player;
use serde::Serialize;
use std::collections::HashMap;

pub const WORLD: u32 = 1022;

#[warn(dead_code)]
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KillMode {
    ModUnknown,
    ModShotgun,
    ModGauntlet,
    ModMachinegun,
    ModGrenade,
    ModGrenadeSplash,
    ModRocket,
    ModRocketSplash,
    ModPlasma,
    ModPlasmaSplash,
    ModRailgun,
    ModLightning,
    ModBfg,
    ModBfgSplash,
    ModWater,
    ModSlime,
    ModLava,
    ModCrush,
    ModTelefrag,
    ModFalling,
    ModSuicide,
    ModTargetLaser,
    ModTriggerHurt,
    ModNail,
    ModChaingun,
    ModProximityMine,
    ModKamikaze,
    ModJuiced,
    ModGrapple,
}

impl KillMode {
    fn from_u32(n: u32) -> KillMode {
        match n {
            0 => KillMode::ModUnknown,
            1 => KillMode::ModShotgun,
            2 => KillMode::ModGauntlet,
            3 => KillMode::ModMachinegun,
            4 => KillMode::ModGrenade,
            5 => KillMode::ModGrenadeSplash,
            6 => KillMode::ModRocket,
            7 => KillMode::ModRocketSplash,
            8 => KillMode::ModPlasma,
            9 => KillMode::ModPlasmaSplash,
            10 => KillMode::ModRailgun,
            11 => KillMode::ModLightning,
            12 => KillMode::ModBfg,
            13 => KillMode::ModBfgSplash,
            14 => KillMode::ModWater,
            15 => KillMode::ModSlime,
            16 => KillMode::ModLava,
            17 => KillMode::ModCrush,
            18 => KillMode::ModTelefrag,
            19 => KillMode::ModFalling,
            20 => KillMode::ModSuicide,
            21 => KillMode::ModTargetLaser,
            22 => KillMode::ModTriggerHurt,
            23 => KillMode::ModNail,
            24 => KillMode::ModChaingun,
            25 => KillMode::ModProximityMine,
            26 => KillMode::ModKamikaze,
            27 => KillMode::ModJuiced,
            28 => KillMode::ModGrapple,
            _ => KillMode::ModUnknown,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Game {
    pub total_kills: u32,
    #[serde(rename = "players")]
    pub player_list: Vec<String>,
    #[serde(rename = "kills")]
    pub kill_score: HashMap<String, i32>,
    pub means_of_death: HashMap<KillMode, u32>,
    #[serde(skip_serializing)]
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            total_kills: 0,
            player_list: Vec::new(),
            kill_score: HashMap::new(),
            means_of_death: HashMap::new(),
            players: Vec::new(),
        }
    }

    pub fn new_player(&mut self, id: u32) {
        self.players.push(Player::new(id));
        self.player_list.push("".to_string());
    }

    pub fn rename_player(&mut self, id: u32, name: String) {
        match self.players.iter_mut().find(|p| p.id == id) {
            Some(p) => p.name = name,
            None => panic!("Player not found"),
        }

        // rebuild the player list
        self.player_list.clear();
        for player in &self.players {
            self.player_list.push(player.name.clone());
        }
    }

    pub fn add_kill(
        &mut self,
        killer_id: u32,
        killed_id: u32,
        means_of_death: u32,
    ) -> Result<(), &'static str> {
        self.total_kills += 1;

        if killer_id == WORLD {
            let player = match self.players.iter().find(|p| p.id == killed_id) {
                Some(p) => p,
                None => return Err("Killed player not found"),
            };
            match self.kill_score.get_mut(&player.name) {
                Some(score) => *score -= 1,
                None => {
                    self.kill_score.insert(player.name.clone(), -1);
                }
            };
        } else if killed_id != killer_id {
            let player = match self.players.iter().find(|p| p.id == killer_id) {
                Some(p) => p,
                None => return Err("Killer player not found"),
            };
            match self.kill_score.get_mut(&player.name) {
                Some(score) => *score += 1,
                None => {
                    self.kill_score.insert(player.name.clone(), 1);
                }
            };
        }

        self.add_kill_mode(KillMode::from_u32(means_of_death));
        Ok(())
    }

    fn add_kill_mode(&mut self, mode: KillMode) {
        let count = self.means_of_death.get(&mode.clone()).unwrap_or(&0);
        self.means_of_death.insert(mode.clone(), count + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let mut game = Game::new();
        game.new_player(1);
        game.new_player(2);
        game.new_player(3);

        let expected_player_list = vec!["".to_string(), "".to_string(), "".to_string()];
        let expected_players = vec![
            Player {
                name: "".to_string(),
                id: 1,
            },
            Player {
                name: "".to_string(),
                id: 2,
            },
            Player {
                name: "".to_string(),
                id: 3,
            },
        ];

        assert_eq!(game.players.len(), 3);
        assert_eq!(game.player_list, expected_player_list);
        assert_eq!(game.players, expected_players);
    }

    #[test]
    fn test_add_kill() -> Result<(), &'static str> {
        let mut game = Game::new();
        game.new_player(1);
        game.new_player(2);

        game.add_kill(1, 2, 1)?;
        game.add_kill(2, 1, 1)?;
        game.add_kill(WORLD, 1, 1)?;

        let expected_players = vec![
            Player {
                name: "".to_string(),
                id: 1,
            },
            Player {
                name: "".to_string(),
                id: 2,
            },
        ];

        assert_eq!(game.players.len(), 2);
        assert_eq!(game.players, expected_players);
        assert_eq!(game.total_kills, 3);
        Ok(())
    }

    #[test]
    fn test_kill_modes() -> Result<(), &'static str> {
        let mut game = Game::new();
        game.new_player(1);
        game.new_player(2);

        game.add_kill(1, 2, 1)?;
        game.add_kill(2, 1, 1)?;
        game.add_kill(WORLD, 1, 0)?;

        let mut expected_modes = HashMap::new();
        expected_modes.insert(KillMode::ModShotgun, 2);
        expected_modes.insert(KillMode::ModUnknown, 1);

        assert_eq!(game.means_of_death, expected_modes);
        Ok(())
    }

    #[test]
    fn test_json_format() {
        let mut game = Game::new();
        game.new_player(1);
        game.rename_player(1, "TestGuy".to_string());
        game.new_player(2);
        game.rename_player(2, "Testman".to_string());

        game.add_kill(2, 1, 10).unwrap();

        let json = serde_json::to_string(&game).unwrap();
        let expected = r#"{"total_kills":1,"players":["TestGuy","Testman"],"kills":{"Testman":1},"means_of_death":{"MOD_RAILGUN":1}}"#;
        assert_eq!(json, expected);
    }
}
