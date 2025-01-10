#[cfg(test)]
mod pijersi {
    use faeries::{
        pijersi::{root::primary, ugi::as_ugi},
        prng,
        search::{alphabeta::alphabeta, flatmc::flatmc, minimax::minimax, random::random},
    };
    use games::{
        gamerules::GameRules,
        pijersi::{PijersiMove, PijersiPosition},
    };
    use protocols::GoSettings;

    fn info_handler(
        _: &PijersiPosition,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<u64>,
        _: Option<u128>,
        _: Option<i32>,
        _: &Vec<PijersiMove>,
    ) {
    }

    #[must_use]
    fn eval(pos: &PijersiPosition) -> i32 {
        let us_lower = pos.get_lower() & pos.get_us();
        let us_upper = pos.get_upper() & pos.get_us();
        let num_us = us_lower.count() + us_upper.count();

        let them_lower = pos.get_lower() & pos.get_them();
        let them_upper = pos.get_upper() & pos.get_them();
        let num_them = them_lower.count() + them_upper.count();

        num_us - num_them
    }

    #[test]
    fn win_in_one() {
        let tests = [
            // Get home
            ("5r-/R-6/6/7/6/7/6 w 0 1", "f1g1"),
            ("6/R-p-5/p-5/7/6/7/6 w 0 1", "f1g1"),
            ("6/7/6/7/6/r-6/5R- b 0 1", "b1a1"),
            ("6/7/6/7/P-5/r-P-5/6 b 0 1", "b1a1"),
            // 20 ply rule
            ("6/R-6/s-s-5/7/6/7/6 w 19 1", "f1g1"),
            ("6/R-6/ssss5/7/6/7/6 w 19 1", "f1g1"),
            // Win by material
            ("6/7/s-5/R-6/6/7/6 w 0 1", "d1e1"),
            ("6/7/6/R-6/s-5/7/6 w 0 1", "d1c1"),
        ];

        let settings = GoSettings::from_depth(3);
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);

        for (fen, movestr) in tests {
            let pos = PijersiPosition::from_fen(fen);

            // Primary
            if let Some(mv) = primary(pos.clone(), &settings, &info_handler) {
                assert_eq!(movestr, as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Minimax
            if let Some(mv) = minimax(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // FlatMC
            if let Some(mv) = flatmc(
                pos.clone(),
                &GoSettings::from_nodes(1000),
                &info_handler,
                || prng.next(),
            ) {
                assert_eq!(movestr, as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Random
            assert!(random(&pos, || prng.next()).is_some(), "{}", fen);
        }
    }

    #[test]
    fn easy_moves() {
        let tests = [
            // Win soon
            ("r-5/R-6/6/7/6/7/6 w 0 1", "f1f2"),
            ("r-5/R-5W-/6/7/6/7/6 w 0 1", "f1f2"),
            // No distractions
            ("r-5/R-6/s-6/7/6/7/6 w 0 1", "f1f2"),
            ("r-5/R-6/ss6/7/6/7/6 w 0 1", "f1f2"),
            // 20 ply rule
            ("r-5/R-6/s-4R-/7/6/7/6 w 19 1", "f1e1"),
            ("r-5/R-6/ss4R-/7/6/7/6 w 19 1", "f1e1"),
        ];

        let settings = GoSettings::from_depth(3);
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);

        for (fen, movestr) in tests {
            let pos = PijersiPosition::from_fen(fen);

            // Primary
            if let Some(mv) = primary(pos.clone(), &settings, &info_handler) {
                assert_eq!(movestr, as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Minimax
            if let Some(mv) = minimax(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Random
            assert!(random(&pos, || prng.next()).is_some(), "{}", fen);
        }
    }
}
