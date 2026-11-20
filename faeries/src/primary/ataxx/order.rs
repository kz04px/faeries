use games::ataxx::{AtaxxMove, AtaxxPosition};
use games::gamerules::GameRules;

const TT_MOVE_SCORE: i32 = i32::MAX;
const KILLER_SCORE: i32 = i32::MAX - 1;

pub struct MovePicker {
    moves: Vec<AtaxxMove<7, 7>>,
    scores: Vec<i32>,
    idx: usize,
}

impl MovePicker {
    #[must_use]
    pub fn new(
        pos: &AtaxxPosition<7, 7>,
        ttmove: Option<AtaxxMove<7, 7>>,
        killer: Option<AtaxxMove<7, 7>>,
    ) -> Self {
        let moves = pos.legal_moves();
        let scores = moves
            .iter()
            .map(|mv| {
                if Some(*mv) == ttmove {
                    TT_MOVE_SCORE
                } else if Some(*mv) == killer {
                    KILLER_SCORE
                } else {
                    pos.piece_change(mv)
                }
            })
            .collect();
        Self {
            moves,
            scores,
            idx: 0,
        }
    }
}

impl Iterator for MovePicker {
    type Item = AtaxxMove<7, 7>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.moves.len() {
            return None;
        }

        let mut best = self.idx;
        for i in (self.idx + 1)..self.moves.len() {
            if self.scores[i] > self.scores[best] {
                best = i;
            }
        }
        self.moves.swap(self.idx, best);
        self.scores.swap(self.idx, best);

        let mv = self.moves[self.idx];
        self.idx += 1;
        Some(mv)
    }
}
