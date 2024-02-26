use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parser::raw_game::{ RawGame, Action };

pub fn parse(buf: BufReader<File>) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    buf.lines().for_each(|rline| {
        let line = rline.unwrap();
        let parts = line.trim().split(' ').collect::<Vec<&str>>();
        if parts.len() >= 2 {
            match parts[1] {
                "InitGame:" => {
                    actions.push(Action::InitGame);
                }
                "ShutdownGame:" => {
                    actions.push(Action::ShutdownGame);
                }
                "Kill:" => {
                    if parts.len() < 5 {
                        panic!("wrong number of parts on kill command");
                    }
                    let killer = parts[2].parse::<u32>().unwrap();
                    let killed = parts[3].parse::<u32>().unwrap();
                    let means_of_death = parts[4].trim_matches(':').parse::<u32>().unwrap();
                    actions.push(Action::Kill(killer, killed, means_of_death));
                }
                "ClientConnect:" => match parts[2].parse::<u32>() {
                    Ok(client) => actions.push(Action::ClientConnect(client)),
                    Err(_) => (),
                },
                "ClientUserinfoChanged:" => match parts[2].parse::<u32>() {
                    Ok(client) => {
                        actions.push(Action::ClientUserinfoChanged(client, parts[3..].join(" ")))
                    }
                    Err(e) => {
                        panic!("Could not parse client id: {}", e);
                    }
                },
                "ClientDisconnect:" => match parts[2].parse::<u32>() {
                    Ok(client) => actions.push(Action::ClientDisconnect(client)),
                    Err(e) => panic!("Could not parse client id: {}", e),
                },
                _ => (),
            }
        }
    });

    actions
}

pub fn group(actions: Vec<Action>) -> Vec<RawGame> {
    let mut games: Vec<RawGame> = Vec::new();
    let mut game = RawGame {
        actions: Vec::new(),
    };
    for action in actions {
        match action {
            Action::InitGame => {
                if game.actions.len() > 0 {
                    // in case there's still an unfinished game
                    games.push(game.clone());
                }
                game = RawGame {
                    actions: Vec::new(),
                };
                game.actions.push(action);
            }
            Action::ShutdownGame => {
                game.actions.push(action);
                games.push(game.clone());
                game = RawGame {
                    actions: Vec::new(),
                };
            }
            _ => {
                game.actions.push(action);
            }
        }
    }

    games
}
