#[cfg(test)]
mod tests {
    use faeries::{
        ataxx::ugi::as_ugi,
        search::{alphabeta::alphabeta, minimax::minimax},
    };
    use games::{
        ataxx::{AtaxxMove, AtaxxPosition},
        gamerules::GameRules,
    };
    use ugi::go::GoSettings;

    fn info_handler(
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

        let settings = GoSettings::from_depth(1);
        for (fen, movestr) in tests {
            let pos = AtaxxPosition::from_fen(fen);

            // Minimax
            if let Some(mv) = minimax(&pos, &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv));
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(&pos, &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv));
            } else {
                panic!("Fail");
            }
        }
    }

    #[test]
    fn easy_moves() {
        let tests = [
            ("7/3o3/2ooo2/2o1o2/2ooo2/3x3/7 x 0 1", "d2d4"),
            ("7/3x3/2xxx2/2x1x2/2xxx2/3o3/7 o 0 1", "d2d4"),
        ];

        let settings = GoSettings::from_depth(1);
        for (fen, movestr) in tests {
            let pos = AtaxxPosition::from_fen(fen);

            // Minimax
            if let Some(mv) = minimax(&pos, &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv));
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(&pos, &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv));
            } else {
                panic!("Fail");
            }
        }
    }

    #[test]
    fn delay() {
        let tests = [
            (
                "ooooooo/ooooooo/ooooooo/ooooooo/ooxxxxx/ooxxxxx/ooxx1xx x 0 1",
                "c1e1",
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxxxx/xxooooo/xxooooo/xxoo1oo o 0 1",
                "c1e1",
            ),
        ];

        let settings = GoSettings::from_depth(1);
        for (fen, movestr) in tests {
            let pos = AtaxxPosition::from_fen(fen);

            // Minimax
            if let Some(mv) = minimax(&pos, &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv));
            } else {
                panic!("Fail");
            }

            // Alphabeta
            if let Some(mv) = alphabeta(&pos, &settings, &info_handler, &eval) {
                assert_eq!(movestr, as_ugi(&mv));
            } else {
                panic!("Fail");
            }
        }
    }
}
