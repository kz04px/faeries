#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, pijersi::PijersiPosition};

    #[test]
    fn perft() {
        let tests = [
            // General
            ("startpos", vec![1, 186, 34_054, 6_410_472]),
            (
                "srp-r-s-2/p-1s-1r-1p-/4p-1/1RPw-3sr/2w-1RSP-/P-2WWS-2/R-S-S-1PR1 w 0 1",
                vec![1, 176, 29_377, 4_963_057],
            ),
            (
                "1p-sr1p-r-/p-r-1w-w-s-p-/4r-1/3ss3/SPR-S-2R-/P-2WW2P-/R-2R-SPS- w 0 1",
                vec![1, 173, 36188, 6122967],
            ),
            (
                "1p-r-1p-r-/1r-s-sr3/ps1W-w-2/SP1W-1w-S-s-/4R-1/P-4R-P-/R-1SR1P-S- w 0 1",
                vec![1, 180, 27328, 4840719],
            ),
            (
                "s-p-r-1s-r-/p-r-s-wws-p-p-/4r-1/7/1P-W-3/1SRR-W-S-R-P-/1P-S-R-P-S- w 0 1",
                vec![1, 190, 37259, 7195115],
            ),
            (
                "sp1r-p-1r-/1prs-ww1s-p-/2s-1W-1/3r-W-PS1/2RS3/P-3R-2/R-P-S-R-1PS w 0 1",
                vec![1, 208, 30078, 6008231],
            ),
            (
                "s-p-r-1p-r-/2s-s-r-s-p-/rp2w-2/WW3RSw-1/5PS/1S-R-4/PRP-S-R-P-1 w 0 1",
                vec![1, 195, 39385, 7417961],
            ),
            (
                "s-1r-s-1r-/p-1s-1p-1p-/4rs1/3RPw-w-1/1rp1WW1P-/1S-2S-R-P-/RP1S-R-1S- w 0 1",
                vec![1, 157, 28631, 4383159],
            ),
            (
                "s-p-r-sp1r-/p-r-s-1p-2/5rs/1w-w-4/2W-W-2/PRS-R-1S-1SP/1SP1R-P-R- w 0 1",
                vec![1, 164, 31668, 5186044],
            ),
            (
                "1sp1rsp-1/p-2wwr-1pr/5s-/rs3R-2/1W-W-P-2/P-S-RP1S-2/R-1S-RP1S- w 0 1",
                vec![1, 171, 26607, 4605776],
            ),
            ("6/7/6/RRr-5/6/7/6 w 0 1", vec![1, 26]),
            ("6/6R-/r-5/7/6/7/6 w 0 1", vec![1, 3, 10]),
            // Can't jump over other pieces
            ("6/7/6/7/r-5/RRr-5/6 w 0 1", vec![1, 4]),
            ("6/5r-r-/4r-W-/5r-r-/6/5w-w-/4w-R- w 0 1", vec![1, 0]),
            ("6/5r-r-/4r-WW/5r-r-/6/5w-w-/4w-R- w 0 1", vec![1, 0]),
            // Wise can't capture or be captured
            ("6/7/6/W-r-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/W-p-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/W-s-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/W-w-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/R-w-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/r-W-5/6/7/6 b 0 1", vec![1, 2]),
            // R > S; S > P; P > R
            ("6/7/6/R-r-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/R-p-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/R-s-5/6/7/6 w 0 1", vec![1, 3]),
            ("6/7/6/r-R-5/6/7/6 b 0 1", vec![1, 2]),
            ("6/7/6/r-P-5/6/7/6 b 0 1", vec![1, 2]),
            ("6/7/6/r-S-5/6/7/6 b 0 1", vec![1, 3]),
            ("6/7/6/P-r-5/6/7/6 w 0 1", vec![1, 3]),
            ("6/7/6/P-p-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/P-s-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/p-R-5/6/7/6 b 0 1", vec![1, 3]),
            ("6/7/6/p-P-5/6/7/6 b 0 1", vec![1, 2]),
            ("6/7/6/p-S-5/6/7/6 b 0 1", vec![1, 2]),
            ("6/7/6/S-r-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/S-p-5/6/7/6 w 0 1", vec![1, 3]),
            ("6/7/6/S-s-5/6/7/6 w 0 1", vec![1, 2]),
            ("6/7/6/s-R-5/6/7/6 b 0 1", vec![1, 2]),
            ("6/7/6/s-P-5/6/7/6 b 0 1", vec![1, 3]),
            ("6/7/6/s-S-5/6/7/6 b 0 1", vec![1, 2]),
        ];

        for (fen, nodes) in tests {
            let mut pos = PijersiPosition::from_fen(fen);

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
