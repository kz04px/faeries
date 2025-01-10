#[cfg(test)]
mod tests {
    use games::{chess::ChessPosition, gamerules::GameRules};

    #[test]
    fn move_parsing() {
        let fens = [
            "startpos",
            "r1bq1rk1/pp2ppbp/2n2np1/3p4/3NP3/2N1BP2/PPPQ2PP/2KR1B1R w - - 0 10",
            "r1bq1rk1/pp2ppbp/2n2np1/3P4/3N4/2N1BP2/PPPQ2PP/2KR1B1R b - - 0 10",
            // En passant
            "rnbqkb1r/ppp1pppp/5n2/2PpP3/8/8/PP1P1PPP/RNBQKBNR w KQkq d6 0 5",
            "rnbqkbnr/pp1p1ppp/8/8/2pPp3/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 5",
            // Castling
            "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1",
            "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1",
            // Promotions
            "1nn1k3/1P6/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/1p6/1NN1K3 b - - 0 1",
        ];

        for fen in fens {
            let pos = ChessPosition::from_fen(fen);
            for mv in pos.legal_moves() {
                let movestr = pos.move_to_string(&mv);
                assert_eq!(mv, pos.parse_movestr(&movestr).unwrap());
            }
        }
    }
}
