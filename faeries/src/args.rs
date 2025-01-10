use std::env;

#[derive(Clone, Copy)]
pub enum GameType {
    Ataxx,
    Chess,
    Connect4,
    Droptaxx,
    Gomoku,
    Isolation,
    Pijersi,
    Tak,
}

#[derive(Default)]
pub struct Args {
    pub game: Option<GameType>,
    pub depth: i32,
}

#[must_use]
fn parse_game_name(name: &str) -> Option<GameType> {
    match name {
        "ataxx" => Some(GameType::Ataxx),
        "chess" => Some(GameType::Chess),
        "connect4" => Some(GameType::Connect4),
        "droptaxx" => Some(GameType::Droptaxx),
        "gomoku" => Some(GameType::Gomoku),
        "isolation" => Some(GameType::Isolation),
        "pijersi" => Some(GameType::Pijersi),
        "tak" => Some(GameType::Tak),
        _ => None,
    }
}

#[must_use]
pub fn parse_args() -> std::io::Result<Args> {
    let args: Vec<String> = env::args().collect();
    let mut parsed = Args::default();

    let mut i = 0;
    while i < args.len() {
        // Pairs
        if i + 1 < args.len() {
            match (args[i].as_str(), args[i + 1].as_str()) {
                ("--game", value) => {
                    parsed.game = parse_game_name(value);
                }
                ("--depth", value) => {
                    parsed.depth = value.parse::<i32>().unwrap();
                }
                _ => {}
            }
        }

        // Singles

        i += 1;
    }

    if parsed.game.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid 'game' parameter",
        ));
    }

    Ok(parsed)
}
