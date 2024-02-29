use serde::Serialize;

use super::game::Game;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum Action {
    InitGame,
    Kill(u32, u32, u32),
    ClientConnect(u32),
    ClientBegin(u32),
    ClientUserinfoChanged(u32, String),
    ClientDisconnect(u32),
    ShutdownGame,
}

impl Action {
    pub fn parse(&self, game: &mut Game) -> Result<(), &'static str> {
        match self {
            Action::InitGame => Ok(()),
            Action::ClientConnect(client) => {
                game.new_player(*client);
                Ok(())
            },
            Action::Kill(killer, killed, means_of_death) => {
                game.add_kill(*killer, *killed, *means_of_death)?;
                Ok(())
            }
            Action::ClientUserinfoChanged(player, metadata) => {
                // reference n\Isgalamido\t\0\model\xian/default\hmodel\xian/default\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
                let parts: Vec<&str> = metadata.splitn(3, "\\").collect();
                if parts.len() < 3 {
                    return Err("Could not parse userinfo");
                }

                game.rename_player(*player, parts[1].to_string())
            }
            Action::ClientBegin(id) => {
                game.player_joined(*id)
            },
            Action::ClientDisconnect(_) => Ok(()),
            Action::ShutdownGame => Ok(()),
        }

    }

    pub fn parse_game(actions: Vec<Action>) -> Result<Game, &'static str> {
        let mut game = Game::new();
        for action in actions {
            action.parse(&mut game)?;
        }

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::player::Player;
    use super::*;
    use crate::parser::game::Game;

    #[test]
    fn test_parse_init_game() {
        let mut game = Game::new();
        let action = Action::InitGame;
        assert_eq!(action.parse(&mut game), Ok(()));
    }

    #[test]
    fn test_parse_client_connect() {
        let mut game = Game::new();
        let action = Action::ClientConnect(1);
        action.parse(&mut game).unwrap();
        assert_eq!(game.players.len(), 1);
        assert_eq!(game.players[0].id, 1);
    }

    #[test]
    fn test_parse_kill() {
        let mut game = Game::new();
        game.new_player(1);
        game.player_joined(1).unwrap();
        game.rename_player(1, "Test".to_string()).unwrap();
        game.new_player(2);
        game.player_joined(2).unwrap();
        let action = Action::Kill(1, 2, 0);
        action.parse(&mut game).unwrap();
        assert_eq!(game.kill_score.get("Test"), Some(&1));
    }

    #[test]
    fn test_parse_client_userinfo_changed() {
        let mut game = Game::new();
        game.new_player(1);
        let action = Action::ClientUserinfoChanged(1, "n\\Test\\t".to_string());
        action.parse(&mut game).unwrap();
        assert_eq!(game.players[0].name, "Test");
    }

    #[test]
    fn test_parse_client_begin() {
        let mut game = Game::new();
        game.new_player(1);
        let action = Action::ClientBegin(1);
        action.parse(&mut game).unwrap();
        assert_eq!(game.players[0].joined, true);
    }

    #[test]
    fn test_to_game() {
        let actions = vec![
                Action::InitGame,
                Action::ClientConnect(2),
                Action::ClientConnect(3),
                Action::ClientConnect(4),
                Action::ClientUserinfoChanged(2, "n\\Testing\\t".to_string()),
                Action::ClientUserinfoChanged(3, "n\\Test\\t".to_string()),
                Action::ClientBegin(2),
                Action::ClientBegin(3),
                Action::Kill(2, 3, 1),
                Action::Kill(crate::parser::game::WORLD, 3, 1),
                Action::ShutdownGame,
            ];

        let expected_players = vec![
            Player {
                name: "Testing".to_string(),
                id: 2,
                joined: true,
            },
            Player {
                name: "Test".to_string(),
                id: 3,
                joined: true,
            },
            Player {
                name: "".to_string(),
                id: 4,
                joined: false,
            },
        ];
        let expected_player_list = vec!["Testing".to_string(), "Test".to_string()];

        let game = Action::parse_game(actions).unwrap();
        assert_eq!(game.players, expected_players);
        assert_eq!(game.player_list, expected_player_list);
    }

    #[test]
    #[should_panic]
    fn test_invalid_game() {
        let actions= vec![
                Action::InitGame,
                Action::ClientConnect(2),
                Action::Kill(3, 2, 1),
                Action::ShutdownGame,
            ];

        Action::parse_game(actions).unwrap();
    }
}
