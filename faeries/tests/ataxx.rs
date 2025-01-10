#[cfg(test)]
mod ataxx {
    use faeries::{
        ataxx::{root::primary, ugi::as_ugi},
        prng,
        search::{alphabeta::alphabeta, flatmc::flatmc, minimax::minimax, random::random},
    };
    use games::{
        ataxx::{AtaxxMove, AtaxxPosition},
        gamerules::GameRules,
    };
    use protocols::GoSettings;

    fn info_handler(
        _: &AtaxxPosition,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<u64>,
        _: Option<u128>,
        _: Option<i32>,
        _: &Vec<AtaxxMove>,
    ) {
    }

    #[must_use]
    fn eval(pos: &AtaxxPosition) -> i32 {
        pos.get_us().count() - pos.get_them().count()
    }

    #[test]
    fn win_in_one() {
        let tests = [
            ("7/7/7/3o3/7/7/x6 x 0 1", "a1c3"),
            ("7/7/7/3o3/7/7/x6 o 0 1", "d4b2"),
            ("7/7/2ooo2/2o1o2/2ooo2/3x3/7 x 0 1", "d2d4"),
            ("7/7/2xxx2/2x1x2/2xxx2/3o3/7 o 0 1", "d2d4"),
        ];

        let settings = GoSettings::from_depth(3);
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);

        for (fen, movestr) in tests {
            let pos = AtaxxPosition::from_fen(fen);

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
            ("7/3o3/2ooo2/2o1o2/2ooo2/3x3/7 x 0 1", "d2d4"),
            ("7/3x3/2xxx2/2x1x2/2xxx2/3o3/7 o 0 1", "d2d4"),
        ];

        let settings = GoSettings::from_depth(3);
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);

        for (fen, movestr) in tests {
            let pos = AtaxxPosition::from_fen(fen);

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

    #[test]
    fn delay() {
        let tests = [
            (
                "ooooooo/ooooooo/ooooooo/ooooooo/ooxxxxx/ooxxxxx/ooxx1xx x 0 1",
                "g1e1",
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxxxx/xxooooo/xxooooo/xxoo1oo o 0 1",
                "g1e1",
            ),
        ];

        let settings = GoSettings::from_depth(3);
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);

        for (fen, movestr) in tests {
            let pos = AtaxxPosition::from_fen(fen);

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
