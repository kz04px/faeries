pub mod go;
pub mod listen;
mod moves;
pub mod options;
mod position;
mod setoption;

use go::GoSettings;

pub enum UGIGameResult {
    P1Win,
    P2Win,
    Draw,
}

pub trait UGI {
    fn init(&mut self);

    fn shutdown(&mut self);

    fn name(&self) -> String;

    fn author(&self) -> String;

    fn uginewgame(&mut self);

    fn isready(&mut self);

    fn position(&mut self, fen: &str);

    fn moves(&mut self, movestr: &str);

    fn go(&mut self, settings: &GoSettings);

    fn perft(&mut self, settings: &GoSettings);

    fn split(&mut self, settings: &GoSettings);

    fn stop(&mut self);

    fn print(&self);

    fn print_options(&self);

    fn set_option(&mut self, name: &str, value: &str);

    #[must_use]
    fn is_debug(&self) -> bool;

    // Queries
    #[must_use]
    fn query_p1turn(&self) -> bool;

    #[must_use]
    fn query_gameover(&self) -> bool {
        self.query_result().is_some()
    }

    #[must_use]
    fn query_result(&self) -> Option<UGIGameResult>;
}