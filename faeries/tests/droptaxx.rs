#[cfg(test)]
mod droptaxx {
    use faeries::{
        droptaxx::{root::primary, ugi::as_ugi},
        prng,
        search::{alphabeta::alphabeta, minimax::minimax, random::random},
    };
    use games::{
        droptaxx::{DroptaxxMove, DroptaxxPosition},
        gamerules::GameRules,
    };
    use protocols::GoSettings;

    fn info_handler(
        _: &DroptaxxPosition,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<i32>,
        _: Option<u64>,
        _: Option<u128>,
        _: Option<i32>,
        _: &Vec<DroptaxxMove>,
    ) {
    }

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
