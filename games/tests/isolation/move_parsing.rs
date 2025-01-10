#[cfg(test)]
mod test {
    use games::{general::square::Square, isolation::IsolationMove};

    #[test]
    fn move_parsing() {
        let tests = [
            (
                "a1a6",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(0, 0),
                    remove: Square::<8, 6>::from_coords(0, 5),
                },
            ),
            (
                "a6h6",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(0, 5),
                    remove: Square::<8, 6>::from_coords(7, 5),
                },
            ),
            (
                "h6h1",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(7, 5),
                    remove: Square::<8, 6>::from_coords(7, 0),
                },
            ),
            (
                "h1a1",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(7, 0),
                    remove: Square::<8, 6>::from_coords(0, 0),
                },
            ),
        ];

        for (movestr, expected) in tests {
            if let Ok(mv) = IsolationMove::from_string(&movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move");
            }
        }
    }

    #[test]
    fn move_parsing_fail() {
        let tests = [
            "", "a1", "a1a7", "a7a1", "i1a1", "a1a2a3", "aa11", "0000", "test",
        ];

        for movestr in tests {
            assert!(IsolationMove::from_string(&movestr).is_err());
        }
    }
}
