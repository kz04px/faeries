#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, pijersi::PijersiPosition};

    #[test]
    fn fens() {
        let tests = [
            "s-p-r-s-p-r-/p-r-s-wwr-s-/6/6/6/P-S-R-WWS-R-/R-P-S-R-P-S- w 0 1",
            "s-p-r-s-p-r-/p-r-s-wwr-s-/6/6/P-5/1S-R-WWS-R-/R-P-S-R-P-S- b 1 1",
        ];

        for fen in tests {
            assert_eq!(PijersiPosition::from_fen(fen).get_fen(), fen);
        }
    }
}
