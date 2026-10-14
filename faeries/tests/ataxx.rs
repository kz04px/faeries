#[cfg(test)]
mod ataxx {
    use faeries::{
        prng,
        search::{
            SearchInfo, alphabeta::alphabeta, flatmc::flatmc, minimax::minimax, random::random,
        },
        ugi::UgiGame,
    };
    use games::{
        ataxx::{AtaxxMove, AtaxxPosition},
        gamerules::GameRules,
    };
    use protocols::GoSettings;

    fn info_handler(_: &AtaxxPosition<7, 7>, _: &SearchInfo<AtaxxMove<7, 7>>) {}

    #[must_use]
    fn eval(pos: &AtaxxPosition<7, 7>) -> i32 {
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

            // Minimax
            if let Some(mv) = minimax(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, pos.as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, pos.as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // FlatMC
            if let Some(mv) = flatmc(
                pos.clone(),
                &GoSettings::from_nodes(1000),
                &info_handler,
                || prng.next(),
            )
            .unwrap()
            {
                assert_eq!(movestr, pos.as_ugi(&mv), "{}", fen);
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

            // Minimax
            if let Some(mv) = minimax(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, pos.as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, pos.as_ugi(&mv), "{}", fen);
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

            // Minimax
            if let Some(mv) = minimax(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, pos.as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(pos.clone(), &settings, &info_handler, &eval) {
                assert_eq!(movestr, pos.as_ugi(&mv), "{}", fen);
            } else {
                panic!("Fail");
            }

            // Random
            assert!(random(&pos, || prng.next()).is_some(), "{}", fen);
        }
    }

    #[test]
    fn gameover() {
        let fen = "ooooooo/ooooooo/ooooooo/ooo-xxx/xxxxxxx/xxxxxxx/xxxxxxx x 0 1";
        let settings = GoSettings::from_depth(3);
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);
        let pos = AtaxxPosition::from_fen(fen);

        assert_eq!(minimax(pos.clone(), &settings, &info_handler, &eval), None);
        assert_eq!(
            alphabeta(pos.clone(), &settings, &info_handler, &eval),
            None
        );
        assert_eq!(
            flatmc(
                pos.clone(),
                &GoSettings::from_nodes(10000),
                &info_handler,
                || prng.next()
            ),
            Ok(None)
        );
        assert_eq!(random(&pos, || prng.next()), None);
    }
}
