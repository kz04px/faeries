#[cfg(test)]
mod droptaxx {
    use faeries::{
        prng,
        search::{SearchInfo, alphabeta::alphabeta, minimax::minimax, random::random},
        ugi::UgiGame,
    };
    use games::{
        droptaxx::{DroptaxxMove, DroptaxxPosition},
        gamerules::GameRules,
    };
    use protocols::GoSettings;

    fn info_handler(_: &DroptaxxPosition, _: &SearchInfo<DroptaxxMove>) {}

    #[must_use]
    fn eval(pos: &DroptaxxPosition) -> i32 {
        pos.get_us().count() - pos.get_them().count()
    }

    #[test]
    fn easy_moves() {
        let tests = [
            ("7/3o3/2ooo2/2o1o2/2ooo2/7/7 x 1", "d4"),
            ("7/3x3/2xxx2/2x1x2/2xxx2/7/7 o 1", "d4"),
        ];

        let settings = GoSettings::from_depth(3);
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);

        for (fen, movestr) in tests {
            let pos = DroptaxxPosition::from_fen(fen);

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
}
