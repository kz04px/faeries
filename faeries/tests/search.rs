#[cfg(test)]
mod search {
    use faeries::{
        prng,
        search::{SearchError, SearchInfo, flatmc::flatmc},
    };
    use games::{
        connect4::{Connect4Move, Connect4Position},
        gamerules::GameRules,
    };
    use protocols::GoSettings;

    fn info_handler(_: &Connect4Position, _: &SearchInfo<Connect4Move>) {}

    fn run_flatmc(settings: GoSettings) -> Result<Option<Connect4Move>, SearchError> {
        let mut prng = prng::XorshiftGenerator::new(0xe50076937a9e5b1c);
        flatmc(
            Connect4Position::startpos(),
            &settings,
            &info_handler,
            || prng.next(),
        )
    }

    #[test]
    fn flatmc_rejects_settings_without_a_limit() {
        let invalid = [
            GoSettings::default(),
            GoSettings::from_depth(3),
            // Player 1 is to move, so only their clock counts
            GoSettings::from_time(None, Some(1000), None, None),
            GoSettings::from_time(None, None, Some(100), Some(100)),
        ];

        for settings in invalid {
            assert!(
                matches!(run_flatmc(settings), Err(SearchError::NoLimit(_))),
                "expected an error"
            );
        }
    }

    #[test]
    fn flatmc_accepts_settings_with_a_limit() {
        let valid = [
            GoSettings::from_nodes(100),
            GoSettings {
                movetime: Some(10),
                ..GoSettings::default()
            },
            GoSettings::from_time(Some(300), None, None, None),
            GoSettings {
                depth: Some(3),
                nodes: Some(100),
                ..GoSettings::default()
            },
        ];

        for settings in valid {
            assert!(matches!(run_flatmc(settings), Ok(Some(_))));
        }
    }
}
