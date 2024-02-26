use serde::Serialize;

use crate::parser::game::Game;

#[derive(Clone, Debug, Serialize)]
pub enum Action {
    InitGame,
    Kill(u32, u32, u32),
    ClientConnect(u32),
    ClientUserinfoChanged(u32, String),
    ClientDisconnect(u32),
    ShutdownGame,
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Action::InitGame, Action::InitGame) => true,
            (Action::Kill(k1, k2, k3), Action::Kill(o1, o2, o3)) => {
                k1 == o1 && k2 == o2 && k3 == o3
            }
            (Action::ClientConnect(k), Action::ClientConnect(o)) => k == o,
            (Action::ClientUserinfoChanged(k, k2), Action::ClientUserinfoChanged(o, o2)) => {
                k == o && k2 == o2
            }
            (Action::ClientDisconnect(k), Action::ClientDisconnect(o)) => k == o,
            (Action::ShutdownGame, Action::ShutdownGame) => true,
            _ => false,
        }
    }
}

pub fn parse_actions(actions: Vec<Action>) -> Result<Game, &'static str> {
    let mut game = Game::new();
    for action in actions {
        match action {
            Action::InitGame => (),
            Action::ClientConnect(client) => game.new_player(client),
            Action::Kill(killer, killed, means_of_death) => {
                game.add_kill(killer, killed, means_of_death).unwrap();
            }
            Action::ClientUserinfoChanged(player, metadata) => {
                // n\Isgalamido\t\0\model\xian/default\hmodel\xian/default\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
                let parts: Vec<&str> = metadata.splitn(3, "\\").collect();
                if parts.len() < 3 {
                    panic!("Could not parse userinfo");
                }
                game.rename_player(player, parts[1].to_string());
            }
            Action::ClientDisconnect(_) => (),
            Action::ShutdownGame => (),
        }
    }

    Ok(game)
}

#[cfg(test)]
mod tests {
    use crate::parser::player::Player;

    use super::*;

    #[test]
    fn test_to_game() {
        let actions = vec![
                Action::InitGame,
                Action::ClientConnect(2),
                Action::ClientConnect(3),
                Action::ClientConnect(4),
                Action::ClientUserinfoChanged(2, "n\\Testing\\t".to_string()),
                Action::ClientUserinfoChanged(3, "n\\Test\\t".to_string()),
                Action::Kill(2, 3, 1),
                Action::Kill(crate::parser::game::WORLD, 3, 1),
                Action::ShutdownGame,
            ];

        let expected_players = vec![
            Player {
                name: "Testing".to_string(),
                id: 2,
            },
            Player {
                name: "Test".to_string(),
                id: 3,
            },
            Player {
                name: "".to_string(),
                id: 4,
            },
        ];
        let expected_player_list = vec!["Testing".to_string(), "Test".to_string(), "".to_string()];

        let game = parse_actions(actions).unwrap();
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

        parse_actions(actions).unwrap();
    }
}
