use games::ataxx::AtaxxPosition;
use games::chess::ChessPosition;
use games::connect4::Connect4Position;
use games::droptaxx::DroptaxxPosition;
use games::gomoku::GomokuPosition;
use games::isolation::IsolationPosition;
use games::pijersi::PijersiPosition;
use games::tak::TakPosition;

/// A simple static evaluation for the shared minimax and alphabeta searches.
pub trait Eval {
    #[must_use]
    fn eval(&self) -> i32;
}

impl Eval for AtaxxPosition<7, 7> {
    fn eval(&self) -> i32 {
        100 * self.get_us().count() - 100 * self.get_them().count()
    }
}

impl Eval for ChessPosition {
    fn eval(&self) -> i32 {
        100 * self.board.get_us().count() - 100 * self.board.get_them().count()
    }
}

impl Eval for Connect4Position {
    fn eval(&self) -> i32 {
        100 * self.get_us().count() - 100 * self.get_them().count()
    }
}

impl Eval for DroptaxxPosition {
    fn eval(&self) -> i32 {
        100 * self.get_us().count() - 100 * self.get_them().count()
    }
}

impl Eval for GomokuPosition {
    fn eval(&self) -> i32 {
        0
    }
}

impl Eval for IsolationPosition {
    fn eval(&self) -> i32 {
        0
    }
}

impl Eval for PijersiPosition {
    fn eval(&self) -> i32 {
        let us_lower = self.get_lower() & self.get_us();
        let us_upper = self.get_upper() & self.get_us();
        let num_us = us_lower.count() + us_upper.count();

        let them_lower = self.get_lower() & self.get_them();
        let them_upper = self.get_upper() & self.get_them();
        let num_them = them_lower.count() + them_upper.count();

        100 * (num_us - num_them)
    }
}

impl<const SIZE: usize> Eval for TakPosition<SIZE> {
    fn eval(&self) -> i32 {
        0
    }
}
