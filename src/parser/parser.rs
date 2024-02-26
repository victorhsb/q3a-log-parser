use crate::parser::actions::Action;
use super::game::Game;
use super::actions::parse_actions;

/// parses a vector of strings into a vector of actions that can be grouped and parsed
///
/// parse will first parse the string vector into a plain Actions vector, and then group them into
/// games and finally proceed to parse the games into a vector of Game structs;
///
/// # Example
/// ```
/// use parser::parse;
/// let input = vec![
/// "  0:00 ------------------------------------------------------------".to_string(),
/// "  0:00 InitGame: ".to_string(),
/// "  0:00 ClientConnect: 2".to_string(),
/// "  0:00 ClientUserinfoChanged: 2 n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\g_redteam\\g_blue".to_string(),
/// "  0:00 ClientConnect: 3".to_string(),
/// "  0:00 ClientUserinfoChanged: 3 n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string(),
/// "  0:00 Kill: 2 3 7: Isgalamido killed Dono da Bola by MOD_ROCKET".to_string(),
/// "  0:00 ShutdownGame: ".to_string(),
/// "  0:00 ------------------------------------------------------------".to_string(),
/// ];
///
/// let games = parse(input).unwrap();
/// ````
///
/// # Panics
///
/// Panics if any of the lines are not in the expected format
///
pub fn parse(buf: Vec<String>) -> Result<Vec<Game>, &'static str> {
    let actions = parse_into_actions(buf);

    let grouped_actions = group(actions);

    parse_grouped_actions(grouped_actions)
}

fn parse_into_actions(buf: Vec<String>) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    for line in buf {
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
                    let killer = parts[2].parse::<u32>().expect("could not parse killer id");
                    let killed = parts[3].parse::<u32>().expect("could not parse killed id");
                    let means_of_death = parts[4].trim_matches(':').parse::<u32>().expect("could not parse means of death id");
                    actions.push(Action::Kill(killer, killed, means_of_death));
                }
                "ClientConnect:" => match parts[2].parse::<u32>() {
                    Ok(client) => actions.push(Action::ClientConnect(client)),
                    Err(_) => (),
                },
                "ClientBegin:" => match parts[2].parse::<u32>() {
                    Ok(client) => actions.push(Action::ClientBegin(client)),
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
    };

    actions
}

fn group(actions: Vec<Action>) -> Vec<Vec<Action>> {
    let mut games: Vec<Vec<Action>> = Vec::new();
    let mut game: Vec<Action> = Vec::new();
    for action in actions {
        match action {
            Action::InitGame => {
                if game.len() > 0 {
                    // in case there's still an unfinished game
                    games.push(game.clone());
                }
                game.clear();
                game.push(action);
            }
            Action::ShutdownGame => {
                game.push(action);
                games.push(game.clone());
                game.clear();
            }
            _ => {
                game.push(action);
            }
        }
    }

    games
}

fn parse_grouped_actions(games: Vec<Vec<Action>>) -> Result<Vec<Game>, &'static str> {
    let mut response: Vec<Game> = Vec::new();
    for game_actions in games {
        response.push(parse_actions(game_actions)?);
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = vec![
            "  0:00 ------------------------------------------------------------".to_string(),
            "  0:00 InitGame: ".to_string(),
            "  0:00 ClientConnect: 2".to_string(),
            "  0:00 ClientUserinfoChanged: 2 n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\g_redteam\\g_blue".to_string(),
            "  0:00 ClientConnect: 3".to_string(),
            "  0:00 ClientUserinfoChanged: 3 n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string(),
            "  0:00 Kill: 2 3 7: Isgalamido killed Dono da Bola by MOD_ROCKET".to_string(),
            "  0:00 ShutdownGame: ".to_string(),
            "  0:00 ------------------------------------------------------------".to_string(),
            "  0:00 ClientConnect: 3".to_string(),
            "  0:00 ClientUserinfoChanged: 3 n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string(),
            "  0:00 Kill: 2 3 7: Isgalamido killed Dono da Bola by MOD_ROCKET".to_string(),
            "  0:00 ShutdownGame: ".to_string(),
            "  0:00 ------------------------------------------------------------".to_string(),
        ];
        let expected = vec![
            Action::InitGame,
            Action::ClientConnect(2),
            Action::ClientUserinfoChanged(2, "n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\g_redteam\\g_blue".to_string()),
            Action::ClientConnect(3),
            Action::ClientUserinfoChanged(3, "n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string()),
            Action::Kill(2, 3, 7),
            Action::ShutdownGame,
            Action::ClientConnect(3),
            Action::ClientUserinfoChanged(3, "n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string()),
            Action::Kill(2, 3, 7),
            Action::ShutdownGame,
        ];
        assert_eq!(parse_into_actions(input), expected);
    }

    #[test]
    fn test_group() {
        let given = vec![
            Action::InitGame,
            Action::ClientConnect(2),
            Action::ClientUserinfoChanged(2, "n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\g_redteam\\g_blue".to_string()),
            Action::ClientConnect(3),
            Action::ClientUserinfoChanged(3, "n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string()),
            Action::Kill(2, 3, 7),
            Action::ShutdownGame,
            Action::ClientConnect(3),
            Action::ClientUserinfoChanged(3, "n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string()),
            Action::Kill(2, 3, 7),
            Action::ShutdownGame,
        ];
        let expected = vec![
            vec![
                Action::InitGame,
                Action::ClientConnect(2),
                Action::ClientUserinfoChanged(2, "n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\g_redteam\\g_blue".to_string()),
                Action::ClientConnect(3),
                Action::ClientUserinfoChanged(3, "n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string()),
                Action::Kill(2, 3, 7),
                Action::ShutdownGame,
            ],
            vec![
                Action::ClientConnect(3),
                Action::ClientUserinfoChanged(3, "n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string()),
                Action::Kill(2, 3, 7),
                Action::ShutdownGame,
            ]
        ]; 

        assert_eq!(group(given), expected);
    }

    use super::super::player::Player;
    use super::super::game::KillMode;

    #[test]
    fn test_parse_grouped_actions() {
        let given = vec![
            vec![
                Action::InitGame,
                Action::ClientConnect(2),
                Action::ClientUserinfoChanged(2, "n\\Isgalamido\\t\\0\\model\\uriel/zael\\hmodel\\uriel/zael\\g_redteam\\g_redteam\\g_blue".to_string()),
                Action::ClientBegin(2),
                Action::ClientConnect(3),
                Action::ClientUserinfoChanged(3, "n\\Dono da Bola\\t\\0\\model\\sarge/krusade\\hmodel\\sarge/krusade\\g_redteam\\g_redteam\\g_blu".to_string()),
                Action::ClientBegin(3),
                Action::Kill(2, 3, 7),
                Action::ShutdownGame,
            ],
        ]; 
        let mut means_of_death = std::collections::HashMap::new();
        means_of_death.insert(KillMode::ModRocketSplash, 1);
        let mut kill_score = std::collections::HashMap::new();
        kill_score.insert("Isgalamido".to_string(), 1);

        let expected = vec![
            Game {
                total_kills: 1,
                players: vec![
                    Player{id: 2, name: "Isgalamido".to_string(), joined: true},
                    Player{id: 3, name: "Dono da Bola".to_string(), joined: true}
                ],
                player_list: vec!["Isgalamido".to_string(), "Dono da Bola".to_string()],
                kill_score,
                means_of_death,
            }
        ];

        assert_eq!(parse_grouped_actions(given).unwrap(), expected);
    }
}


