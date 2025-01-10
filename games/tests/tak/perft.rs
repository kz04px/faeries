#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, tak::TakPosition};

    #[test]
    fn perft_3x3() {
        let tests = [(
            "startpos",
            vec![
                1, 9, 72, 1_200, 17_792, 271_812, 3_712_952, /*52_364_896, 679_639_648*/
            ],
        )];

        for (fen, nodes) in tests {
            let mut pos = TakPosition::<3>::from_fen(fen);

            for (depth, expected) in nodes.iter().enumerate() {
                let got = pos.perft(depth as i32);

                assert_eq!(
                    got, *expected,
                    "depth {} expected {} got {} fen {}",
                    depth, expected, got, fen
                );
            }
        }
    }

    #[test]
    fn perft_6x6() {
        let tests = [
            (
                "startpos",
                vec![1, 36, 1_260, 132_720, 13_586_048, /*1_253_506_520*/],
            ),
            (
                "2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/11,x5 1 1",
                vec![1, 18, 317, 4_243, 64_855, 754_479, 11_320_295, /*130_812_445, 2_042_784_845, 24_765_415_103*/],
            ),
            (
                "2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/x5,11 1 1",
                vec![1, 18],
            ),
            (
                "11,x5/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S 1 1",
                vec![1, 18],
            ),
            (
                "x5,11/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S 1 1",
                vec![1, 18],
            ),
            (
                "2,2,21S,2,2,2/2,x,222221,2,2,x/1,1,2221C,x,111112C,2S/x,1,2S,x2,121211212/1,1,1212S,1S,2,1S/x2,2,1,21,1 1 42",
                vec![1, 140, 21_402, 2_774_593, /*395_359_484, 48_986_506_534*/],
            ),
        ];

        for (fen, nodes) in tests {
            let mut pos = TakPosition::<6>::from_fen(fen);

            for (depth, expected) in nodes.iter().enumerate() {
                let got = pos.perft(depth as i32);

                assert_eq!(
                    got, *expected,
                    "depth {} expected {} got {} fen {}",
                    depth, expected, got, fen
                );
            }
        }
    }
}
