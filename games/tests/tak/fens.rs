#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, tak::TakPosition};

    #[test]
    fn fens_3x3() {
        let tests = [
            "x3/x3/x3 1 1",
            "x3/x3/x3 2 1",
            "x3/x3/x3 1 100",
            "x3/x3/x3 2 100",
        ];

        for fen in tests {
            println!("{}", fen);
            assert_eq!(TakPosition::<3>::from_fen(fen).get_fen(), fen);
        }
    }

    #[test]
    fn fens_6x6() {
        let tests = [
            "x6/x6/x6/x6/x6/x6 1 1",
            // Side to move
            "x6/x6/x6/x6/x6/x6 2 1",
            // Counters
            "x6/x6/x6/x6/x6/x6 1 100",
            "x6/x6/x6/x6/x6/x6 2 100",
            // Flat
            "2,x5/x6/x6/x6/x6/x6 2 1",
            "x,2,x4/x6/x6/x6/x6/x6 2 1",
            "x2,2,x3/x6/x6/x6/x6/x6 2 1",
            "x3,2,x2/x6/x6/x6/x6/x6 2 1",
            "x5,2/x4,1,x/x3,2,x2/x2,1,x3/x,2,x4/1,x5 1 4",
            "x4,1,2/x3,2,1,x/x2,1,2,x2/x,2,1,x3/1,2,x4/1,x5 2 6",
            "x3,2,x,1/x,1,x3,2/x,1,1,1,1,x/x,2,2,2,2,x/x3,1,x2/x6 2 7",
            // Standing
            "2,1,x4/1S,2S,x4/x6/x6/x6/x6 1 3",
            // Caps
            "2,1,x4/1C,2C,x4/x6/x6/x6/x6 1 3",
            // Mixed
            "x2,2C,2S,2,x/x2,1C,1S,1,x/x6/x6/x6/x6 2 4",
            // Stacks
            "21,12,x4/x6/x6/x6/x6/x6 1 4",
            "211,122,x4/x6/x6/x6/x6/x6 1 6",
            "21S,12S,x4/x6/x6/x6/x6/x6 1 4",
            "211S,122S,x4/x6/x6/x6/x6/x6 1 6",
            "21C,12C,x4/x6/x6/x6/x6/x6 1 4",
            "211C,122C,x4/x6/x6/x6/x6/x6 1 6",
            // Adjacent spaces
            // "x6/x6/x6/x6/x6/x1,x5 1 1",
            // "x6/x6/x6/x6/x6/x2,x4 1 1",
            // "x6/x6/x6/x6/x6/x3,x3 1 1",
            // "x6/x6/x6/x6/x6/x4,x2 1 1",
            // "x6/x6/x6/x6/x6/x5,x1 1 1",
            // Shorthand
            "x6/x6/x6/x6/x6/1,x,2,x,1,x 1 1",
            "x6/x6/x6/x6/x6/x,1,x,2,x,1 1 1",
            // Other
            "x,2,1,x3/x,111S,22,2S,222111,1/x2,2,1S,2,2S/x,121S,1,2S,2C,2/2,x,212S,2,2,1S/x,1221C,2,x,21,x 2 29",
            "x3,12,2S,x/x,22S,22C,11,21,x/121,212,12,1121C,1212S,x/21S,1,21,211S,12S,x/x,21S,2,x3/x6 1 26",
            "1,x,1,1,1,x/1,11112C,111121C,2S,x,1/2,x,1122,2S,1,1/2,x,2S,x2,2/2,2,1S,2,2221S,2/2,x,112,x2,2 1 33",
            "1,x,1,x3/x,2,1121C,x3/1112,x,2,x3/2,2,x,2,x2/x2,1212,x3/2C,1,1,x3 1 18",
            "2,x2,1,2,1/x3,1,2C,1/x3,121C,12,1/x2,2,2,2,2/x4,1,x/x4,1,x 1 12",
            "1,x,1,x3/x2,1111212,2,x2/x,1,21,212,x,1/x2,21,2,2,2C/x2,21C,x2,2/2,x5 2 22",
            "x2,2,x2,1/x,122,121C,212,x2/x2,2,2,2,2C/1,x2,2,x,1/x2,1,x2,1/1,x2,2111112,x,1 2 24",
            "2,x,222212,x,2,12C/x,221S,x2,2,x/21,x3,1,x/22,221,1121S,x,1,x/2112,221S,x4/x,1C,1,1,x2 2 48",
            "2,x2,2,1,x/2,x,12,x,1,112S/2,21S,221C,211111,2,1/x,1S,x,22212C,x2/1S,2,2,x2,21/x,21121,2,12,2,2 2 36",
        ];

        for fen in tests {
            println!("{}", fen);
            assert_eq!(TakPosition::<6>::from_fen(fen).get_fen(), fen);
        }
    }
}
