use crate::GoSettings;

pub mod go;
pub mod listen;
mod moves;
pub mod options;
pub mod position;
mod setoption;

pub trait TEI {
    fn init(&mut self);

    fn shutdown(&mut self);

    fn name(&self) -> String;

    fn author(&self) -> String;

    fn version(&self) -> String;

    fn teinewgame(&mut self, size: i32, halfkomi: i32);

    fn isready(&mut self);

    fn position(&mut self, tps: &str);

    fn moves(&mut self, movestr: &str);

    fn go(&mut self, settings: &GoSettings);

    fn perft(&mut self, settings: &GoSettings);

    fn split(&mut self, settings: &GoSettings);

    fn stop(&mut self);

    fn print(&self);

    fn pprint(&self) {
        self.print();
    }

    fn print_size_komi(&self);

    fn print_options(&self);

    fn movelist(&self) {}

    fn set_option(&mut self, name: &str, value: &str);

    #[must_use]
    fn is_debug(&self) -> bool;
}
