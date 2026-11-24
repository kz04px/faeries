use super::butterfly::ButterflyHistory;
use games::{
    ataxx::{AtaxxMove, AtaxxPosition},
    gamerules::GameRules,
    general::{bitboard::Bitboard, side::Side},
};

/// Larger than any spread of butterfly values, so that they only separate moves that change
/// the same number of pieces
const CHANGE_WEIGHT: i32 = 2 * ButterflyHistory::MAX + 1;

/// No position has more than 194 moves
const MAX_MOVES: usize = 200;

/// The moves of one stage of a node and their scores, kept between nodes so that move
/// generation doesn't allocate or initialise anything
pub struct MoveBuffer {
    moves: [AtaxxMove<7, 7>; MAX_MOVES],
    scores: [i32; MAX_MOVES],
}

impl Default for MoveBuffer {
    fn default() -> Self {
        Self {
            moves: [AtaxxMove::Pass; MAX_MOVES],
            scores: [0; MAX_MOVES],
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Stage {
    TtMove,
    Killer,
    GenerateBig,
    Big,
    Small,
    Done,
}

/// Hands out the moves of a position best first, generating each kind only when the ones
/// before it have all been tried
///
/// The table move comes first, then the killer, then the moves that change at least two
/// pieces, then the rest. Each group is handed out most pieces changed first, and among those the ones that have caused the most cutoffs.
pub struct MovePicker {
    stage: Stage,
    ttmove: Option<AtaxxMove<7, 7>>,
    killer: Option<AtaxxMove<7, 7>>,
    us: Bitboard<7, 7>,
    them: Bitboard<7, 7>,
    empty: Bitboard<7, 7>,
    side: Side,
    len: usize,
    idx: usize,
}

impl MovePicker {
    #[must_use]
    pub fn new(
        pos: &AtaxxPosition<7, 7>,
        ttmove: Option<AtaxxMove<7, 7>>,
        killer: Option<AtaxxMove<7, 7>>,
        buffer: &mut MoveBuffer,
    ) -> Self {
        let (us, them, empty) = (pos.get_us(), pos.get_them(), pos.get_empty());
        let mut picker = Self {
            stage: Stage::TtMove,
            ttmove,
            killer,
            us,
            them,
            empty,
            side: pos.get_turn(),
            len: 0,
            idx: 0,
        };

        // With nowhere to move, the only move is to pass, if the opponent can still move
        if (us.reach2() & empty).is_empty() {
            picker.stage = Stage::Done;
            if (them.reach2() & empty).is_occupied() {
                buffer.moves[0] = AtaxxMove::Pass;
                picker.len = 1;
            }
        }

        picker
    }

    /// Whether `mv` can be played from here
    fn is_legal(&self, mv: AtaxxMove<7, 7>) -> bool {
        match mv {
            AtaxxMove::Pass => false,
            AtaxxMove::Single(to) => {
                !(Bitboard::from_square(to) & self.empty & self.us.adjacent()).is_empty()
            }
            AtaxxMove::Double(from, to) => {
                !(Bitboard::from_square(from) & self.us).is_empty()
                    && !(Bitboard::from_square(to)
                        & self.empty
                        & Bitboard::from_square(from).dist2())
                    .is_empty()
            }
        }
    }

    /// Whether `mv` was already handed out as the table move or the killer
    fn is_tried(&self, mv: AtaxxMove<7, 7>) -> bool {
        (Some(mv) == self.ttmove && self.is_legal(mv))
            || (Some(mv) == self.killer && Some(mv) != self.ttmove && self.is_legal(mv))
    }

    /// Fill `buffer` with the moves that change at least two pieces, or with all the others
    ///
    /// A single move changes one more piece than the enemy pieces next to its target (it adds
    /// its own), and a double move changes exactly that many.
    fn generate(&mut self, buffer: &mut MoveBuffer, butterfly: &ButterflyHistory, big: bool) {
        self.len = 0;
        self.idx = 0;

        let mut add = |picker: &mut Self, mv: AtaxxMove<7, 7>, change: i32| {
            if (change >= 2) == big && !picker.is_tried(mv) {
                buffer.moves[picker.len] = mv;
                buffer.scores[picker.len] = change * CHANGE_WEIGHT + butterfly.get(picker.side, mv);
                picker.len += 1;
            }
        };

        for to in self.us.adjacent() & self.empty {
            let flipped = (Bitboard::from_square(to).adjacent() & self.them).count();
            add(self, AtaxxMove::Single(to), flipped + 1);
        }
        for from in self.us {
            for to in Bitboard::from_square(from).dist2() & self.empty {
                let flipped = (Bitboard::from_square(to).adjacent() & self.them).count();
                add(self, AtaxxMove::Double(from, to), flipped);
            }
        }
        debug_assert!(self.len <= MAX_MOVES);
    }

    pub fn next(
        &mut self,
        buffer: &mut MoveBuffer,
        butterfly: &ButterflyHistory,
    ) -> Option<AtaxxMove<7, 7>> {
        loop {
            match self.stage {
                Stage::TtMove => {
                    self.stage = Stage::Killer;
                    if let Some(mv) = self.ttmove
                        && self.is_legal(mv)
                    {
                        return Some(mv);
                    }
                }
                Stage::Killer => {
                    self.stage = Stage::GenerateBig;
                    if let Some(mv) = self.killer
                        && Some(mv) != self.ttmove
                        && self.is_legal(mv)
                    {
                        return Some(mv);
                    }
                }
                Stage::GenerateBig => {
                    self.stage = Stage::Big;
                    self.generate(buffer, butterfly, true);
                }
                Stage::Big | Stage::Small => {
                    if self.idx < self.len {
                        let mut best = self.idx;
                        for i in (self.idx + 1)..self.len {
                            if buffer.scores[i] > buffer.scores[best] {
                                best = i;
                            }
                        }
                        buffer.moves.swap(self.idx, best);
                        buffer.scores.swap(self.idx, best);

                        let mv = buffer.moves[self.idx];
                        self.idx += 1;
                        return Some(mv);
                    }

                    if self.stage == Stage::Big {
                        self.stage = Stage::Small;
                        self.generate(buffer, butterfly, false);
                    } else {
                        self.stage = Stage::Done;
                        self.len = 0;
                        self.idx = 0;
                    }
                }
                Stage::Done => {
                    if self.idx < self.len {
                        self.idx += 1;
                        return Some(buffer.moves[self.idx - 1]);
                    }
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use games::gamerules::GameRules;

    /// A small deterministic generator, so that the test needs no dependencies
    fn next_random(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    fn picked(
        pos: &AtaxxPosition<7, 7>,
        ttmove: Option<AtaxxMove<7, 7>>,
        killer: Option<AtaxxMove<7, 7>>,
    ) -> Vec<AtaxxMove<7, 7>> {
        let mut buffer = MoveBuffer::default();
        let butterfly = ButterflyHistory::default();
        let mut picker = MovePicker::new(pos, ttmove, killer, &mut buffer);
        let mut out = vec![];
        while let Some(mv) = picker.next(&mut buffer, &butterfly) {
            out.push(mv);
        }
        out
    }

    #[test]
    fn the_picker_yields_every_legal_move_once() {
        let fens = [
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/2-1-2/3-3/2-1-2/7/o5x o 0 1",
            "xxxxxxx/xxxxxxx/xxxxxxx/xxxxxxx/xxxxxxx/xxxxxxx/ooooooo x 0 1",
        ];
        let mut rng = 0x9e37_79b9_7f4a_7c15;
        let mut checked = 0;

        for fen in fens {
            for _ in 0..40 {
                let mut pos = AtaxxPosition::<7, 7>::from_fen(fen);
                while !pos.is_gameover() {
                    let legal = pos.legal_moves();
                    let pick =
                        |rng: &mut u64| legal[(next_random(rng) % legal.len() as u64) as usize];
                    let (a, b) = (pick(&mut rng), pick(&mut rng));

                    // Every combination of a legal, an unrelated and a missing hint
                    for (tt, killer) in [
                        (None, None),
                        (Some(a), None),
                        (None, Some(a)),
                        (Some(a), Some(b)),
                        (Some(a), Some(a)),
                        (Some(AtaxxMove::Pass), Some(a)),
                    ] {
                        let moves = picked(&pos, tt, killer);

                        assert_eq!(moves.len(), legal.len(), "{fen}");
                        for mv in &legal {
                            assert_eq!(moves.iter().filter(|m| *m == mv).count(), 1, "{fen}");
                        }
                        if let Some(tt) = tt
                            && legal.contains(&tt)
                        {
                            assert_eq!(moves[0], tt);
                        }
                        checked += 1;
                    }

                    pos.makemove(&a);
                }
            }
        }
        assert!(checked > 2_000);
    }
}
