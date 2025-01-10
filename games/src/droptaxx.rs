use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, side::Side, square::Square},
};

#[derive(Clone)]
struct Irrecoverable {
    pub pieces: [Bitboard<7, 7>; 2],
}

#[derive(Clone, Default)]
pub struct DroptaxxPosition {
    stack: Vec<Irrecoverable>,
    pub pieces: [Bitboard<7, 7>; 2],
    pub blockers: Bitboard<7, 7>,
    pub turn: Side,
    pub fullmoves: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DroptaxxMove(pub Square<7, 7>);

impl DroptaxxPosition {
    #[must_use]
    pub fn get_us(&self) -> Bitboard<7, 7> {
        self.pieces[self.turn]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard<7, 7> {
        self.pieces[!self.turn]
    }

    #[must_use]
    pub fn get_black(&self) -> Bitboard<7, 7> {
        self.pieces[Side::Player1]
    }

    #[must_use]
    pub fn get_white(&self) -> Bitboard<7, 7> {
        self.pieces[Side::Player2]
    }

    #[must_use]
    pub fn get_blockers(&self) -> Bitboard<7, 7> {
        self.blockers
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard<7, 7> {
        !(self.get_black() | self.get_white() | self.get_blockers())
    }
}

impl GameRules for DroptaxxPosition {
    type MoveType = DroptaxxMove;
    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 7;

    fn startpos() -> Self {
        Self {
            stack: vec![],
            pieces: [Bitboard(0x0), Bitboard(0x0)],
            blockers: Bitboard(0x0),
            turn: Side::Player1,
            fullmoves: 1,
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, _: usize) -> bool {
        let bb = Bitboard::from_coords(x, y);
        match c {
            'x' => self.pieces[0] ^= bb,
            'o' => self.pieces[1] ^= bb,
            '-' => self.blockers ^= bb,
            _ => {}
        }
        true
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        for sq in self.get_empty() {
            func(DroptaxxMove(sq));
        }
    }

    fn is_gameover(&self) -> bool {
        self.get_empty().is_empty()
    }

    fn get_result(&self) -> Option<GameResult> {
        let empty = !(self.get_us() | self.get_them());
        let num_player1 = self.get_black().count();
        let num_player2 = self.get_white().count();

        if empty.is_empty() {
            if num_player1 > num_player2 {
                Some(GameResult::Win(Side::Player1))
            } else if num_player2 > num_player1 {
                Some(GameResult::Win(Side::Player2))
            } else {
                Some(GameResult::Draw)
            }
        } else {
            None
        }
    }

    fn count_moves(&self) -> u64 {
        self.get_empty().count() as u64
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        self.stack.push(Irrecoverable {
            pieces: self.pieces,
        });
        let bb = Bitboard::from_square(mv.0);
        let captured = bb.adjacent() & self.get_them();
        self.pieces[self.turn] ^= bb;
        self.pieces[self.turn] ^= captured;
        self.pieces[!self.turn] ^= captured;
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undomove(&mut self, _mv: &Self::MoveType) {
        let data = self
            .stack
            .pop()
            .expect("Can't undo a move that was never made");
        self.pieces = data.pieces;
        self.turn = !self.turn;
        self.fullmoves -= (self.turn == Side::Player2) as i32;
    }

    fn makenull(&mut self) {
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undonull(&mut self) {
        self.turn = !self.turn;
        self.fullmoves -= (self.turn == Side::Player2) as i32;
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let sq = Square::from_coords(x, y);
        if self.get_black().is_square_set(sq) {
            Some("x".to_owned())
        } else if self.get_white().is_square_set(sq) {
            Some("o".to_owned())
        } else if self.get_blockers().is_square_set(sq) {
            Some("-".to_owned())
        } else {
            None
        }
    }

    fn get_fen(&self) -> String {
        format!(
            "{} {} {}",
            self.get_board_fen(),
            match self.turn {
                Side::Player1 => "x",
                Side::Player2 => "o",
            },
            self.fullmoves,
        )
    }

    fn parse_fen_part(&mut self, idx: usize, part: &str) {
        match idx {
            0 => {}
            // Side to move
            1 => match part {
                "x" => self.turn = Side::Player1,
                "o" => self.turn = Side::Player2,
                _ => panic!("Uh oh {}", part),
            },
            // Fullmoves
            2 => self.fullmoves = part.parse::<i32>().unwrap(),
            _ => panic!("Invalid fen part index"),
        }
    }

    fn get_turn(&self) -> Side {
        self.turn
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        if (self.get_black() & self.get_white()).is_occupied() {
            Err("black & white overlap")
        } else if (self.get_black() & self.get_blockers()).is_occupied() {
            Err("black & blockers overlap")
        } else if (self.get_white() & self.get_blockers()).is_occupied() {
            Err("white & blockers overlap")
        } else {
            Ok(())
        }
    }

    fn perft(&mut self, depth: i32) -> u64 {
        match depth {
            0 => 1,
            1 => self.count_moves(),
            _ => {
                if self.is_gameover() {
                    return 0;
                }

                let mut nodes = 0;

                for sq in self.get_empty() {
                    let bb = Bitboard::from_square(sq);
                    let captured = bb.adjacent() & self.get_them();

                    // makemove
                    self.pieces[self.turn] ^= bb;
                    self.pieces[self.turn] ^= captured;
                    self.pieces[!self.turn] ^= captured;
                    self.turn = !self.turn;

                    nodes += self.perft(depth - 1);

                    // undomove
                    self.turn = !self.turn;
                    self.pieces[self.turn] ^= bb;
                    self.pieces[self.turn] ^= captured;
                    self.pieces[!self.turn] ^= captured;
                }

                nodes
            }
        }
    }
}

impl DroptaxxMove {
    #[must_use]
    pub fn from_string(movestr: &str) -> Result<Self, &'static str> {
        let sq = Square::from_string(&movestr)?;
        Ok(DroptaxxMove(sq))
    }
}

impl std::fmt::Display for DroptaxxPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut height = 6;
        let mut width = 6;

        loop {
            let mut yeet = true;
            for x in 0..Self::WIDTH {
                let sq = Square::from_coords(x, height);
                if !self.get_blockers().is_square_set(sq) {
                    yeet = false;
                    break;
                }
            }
            if !yeet {
                break;
            }
            height -= 1;
        }

        loop {
            let mut yeet = true;
            for y in 0..Self::HEIGHT {
                let sq = Square::from_coords(width, y);
                if !self.get_blockers().is_square_set(sq) {
                    yeet = false;
                    break;
                }
            }
            if !yeet {
                break;
            }
            width -= 1;
        }

        for y in (0..=height).rev() {
            for x in 0..=width {
                let sq = Square::from_coords(x, y);
                if self.get_black().is_square_set(sq) {
                    write!(f, "x")?;
                } else if self.get_white().is_square_set(sq) {
                    write!(f, "o")?;
                } else if self.get_blockers().is_square_set(sq) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        match self.turn {
            Side::Player1 => writeln!(f, "Turn: x")?,
            Side::Player2 => writeln!(f, "Turn: o")?,
        }

        Ok(())
    }
}
