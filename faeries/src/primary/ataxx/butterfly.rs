use games::{ataxx::AtaxxMove, general::side::Side};
use tunable::tunable;

const SQUARES: usize = 49;
const MAX_HISTORY: i32 = 16384;
#[tunable(min = 1, max = 256, step = 2)]
const BONUS_SCALE: i32 = 32;
#[tunable(min = 100, max = 8000, step = 100)]
const MAX_BONUS: i32 = 4000;

pub struct ButterflyHistory {
    table: Vec<i16>,
}

impl Default for ButterflyHistory {
    fn default() -> Self {
        Self {
            table: vec![0; 2 * SQUARES * SQUARES],
        }
    }
}

#[must_use]
fn index(side: Side, mv: AtaxxMove<7, 7>) -> Option<usize> {
    let (from, to) = match mv {
        AtaxxMove::Pass => return None,
        AtaxxMove::Single(to) => (to.get_index(), to.get_index()),
        AtaxxMove::Double(from, to) => (from.get_index(), to.get_index()),
    };
    let side = usize::from(side != Side::Player1);
    Some((side * SQUARES + from) * SQUARES + to)
}

impl ButterflyHistory {
    pub const MAX: i32 = MAX_HISTORY;

    pub fn clear(&mut self) {
        self.table.fill(0);
    }

    #[must_use]
    pub fn get(&self, side: Side, mv: AtaxxMove<7, 7>) -> i32 {
        index(side, mv).map_or(0, |i| i32::from(self.table[i]))
    }

    fn apply(&mut self, side: Side, mv: AtaxxMove<7, 7>, bonus: i32) {
        if let Some(i) = index(side, mv) {
            let h = i32::from(self.table[i]);
            self.table[i] = (h + bonus - h * bonus.abs() / MAX_HISTORY) as i16;
        }
    }

    pub fn update(
        &mut self,
        side: Side,
        depth: i32,
        best: AtaxxMove<7, 7>,
        tried: &[AtaxxMove<7, 7>],
    ) {
        let bonus = (BONUS_SCALE.get() * depth * depth).min(MAX_BONUS.get());
        self.apply(side, best, bonus);
        for &mv in tried {
            self.apply(side, mv, -bonus);
        }
    }
}
