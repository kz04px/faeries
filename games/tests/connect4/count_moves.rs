#[cfg(test)]
mod tests {
    use games::{connect4::Connect4Position, gamerules::GameRules};

    #[test]
    fn count_moves() {
        let tests = [
            "startpos",
            "7/7/7/7/7/rrrr3 r 1",
            "7/7/7/7/7/rrrr3 y 1",
            "7/7/7/7/7/rrr4 r 1",
            "7/7/7/7/7/rrr4 y 1",
            "7/yyrryyr/rryyrry/yyrryyr/rryyrry/yyrryyr r 1",
            "7/yyrryyr/rryyrry/yyrryyr/rryyrry/yyrryyr y 1",
        ];

        for fen in tests {
            let pos = Connect4Position::from_fen(fen);
            assert_eq!(pos.legal_moves().len() as u64, pos.count_moves());
        }
    }
}
