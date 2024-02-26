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

#[derive(Clone, Debug, Serialize)]
pub struct RawGame {
    pub actions: Vec<Action>,
}

impl RawGame {
    pub fn to_game(&self) -> Game {
        let mut game = Game::new();
        for action in &self.actions {
            match action {
                Action::InitGame => (),
                Action::ClientConnect(client) => game.new_player(*client),
                Action::Kill(killer, killed, means_of_death) => {
                    game.add_kill(*killer, *killed, *means_of_death);
                }
                Action::ClientUserinfoChanged(player, metadata) => {
                    // n\Isgalamido\t\0\model\xian/default\hmodel\xian/default\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
                    let parts: Vec<&str> = metadata.splitn(3, "\\").collect();
                    if parts.len() < 3 {
                        panic!("Could not parse userinfo");
                    }
                    game.rename_player(*player, parts[1].to_string());
                }
                Action::ClientDisconnect(_) => (),
                Action::ShutdownGame => {
                    game.finish_game()
                },
            }
        }

        game
    }
}
