use games::ataxx::AtaxxPosition;

const GRAIN: i32 = 256;
const MAX_CORRECTION: i32 = 150;
const WEIGHT_SCALE: i32 = 64;
const MAX_WEIGHT: i32 = 16;
const MAX_PIECES: usize = 49;
const MATERIAL_SIZE: usize = 2 * MAX_PIECES + 1;
const PHASE_BUCKET: usize = 7;
const PHASE_SIZE: usize = MAX_PIECES / PHASE_BUCKET + 1;

pub struct CorrectionHistory {
    table: Vec<i32>,
}

impl Default for CorrectionHistory {
    fn default() -> Self {
        Self {
            table: vec![0; MATERIAL_SIZE * PHASE_SIZE],
        }
    }
}

#[must_use]
fn index(pos: &AtaxxPosition<7, 7>) -> usize {
    let us = pos.get_us().count() as usize;
    let them = pos.get_them().count() as usize;
    let material = us + MAX_PIECES - them;
    let phase = (us + them) / PHASE_BUCKET;
    material * PHASE_SIZE + phase
}

impl CorrectionHistory {
    pub fn clear(&mut self) {
        self.table.fill(0);
    }

    #[must_use]
    pub fn get(&self, pos: &AtaxxPosition<7, 7>) -> i32 {
        self.table[index(pos)] / GRAIN
    }

    pub fn update(&mut self, pos: &AtaxxPosition<7, 7>, diff: i32, depth: i32) {
        let weight = (depth + 1).min(MAX_WEIGHT);
        let entry = &mut self.table[index(pos)];
        let target = diff.clamp(-MAX_CORRECTION, MAX_CORRECTION) * GRAIN;
        *entry += (target - *entry) * weight / WEIGHT_SCALE;
        *entry = (*entry).clamp(-MAX_CORRECTION * GRAIN, MAX_CORRECTION * GRAIN);
    }
}
