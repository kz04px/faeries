pub mod listen;

pub enum ManualGameResult {
    P1Win,
    P2Win,
    Draw,
}

pub trait Manual {
    fn print(&self);

    #[must_use]
    fn is_gameover(&self) -> bool;

    #[must_use]
    fn get_result(&self) -> Option<ManualGameResult>;

    fn makemove(&mut self, movestr: &str) -> bool;

    fn makenull(&mut self);

    fn play(&mut self, depth: i32);

    fn hint(&mut self, depth: i32);
}
