#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, isolation::IsolationPosition};

    #[test]
    fn fens() {
        let tests = [
            "8/8/8/8/8/8 b 1",
            "8/8/8/8/8/8 r 1",
            "8/8/8/8/8/8 b 100",
            "8/8/8/8/8/8 r 100",
            "8/8/8/8/8/pP6 b 1",
            "8/8/8/8/8/p.P5 b 1",
            "8/8/8/8/8/p..P4 b 1",
            "8/8/8/8/8/p...P3 b 1",
            "8/8/8/8/8/p....P2 b 1",
        ];

        for fen in tests {
            let pos = IsolationPosition::from_fen(fen);
            assert_eq!(pos.get_fen(), fen);
        }
    }
}
