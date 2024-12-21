use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, side::Side, square::Square},
};

#[derive(Clone, Copy, Default)]
pub struct DroptaxxPosition {
    pub pieces: [Bitboard<7, 7>; 2],
    pub blockers: Bitboard<7, 7>,
    pub turn: Side,
    pub fullmoves: i32,
}

#[derive(Clone, Copy)]
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
}

impl GameRules for DroptaxxPosition {
    type MoveType = DroptaxxMove;
    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 7;

    fn startpos() -> Self {
        Self {
            pieces: [Bitboard::<7, 7>(0x0), Bitboard::<7, 7>(0x0)],
            blockers: Bitboard::<7, 7>(0x0),
            turn: Side::Player1,
            fullmoves: 1,
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char) {
        let bb = Bitboard::<7, 7>::from_coords(x, y);
        match c {
            'x' => self.pieces[0] ^= bb,
            'o' => self.pieces[1] ^= bb,
            '-' => self.blockers ^= bb,
            _ => {}
        }
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType)) {
        let empty = !(self.get_us() | self.get_them());
        for sq in empty {
            func(Self::MoveType {
                0: Square::<7, 7>(sq),
            });
        }
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

    fn makemove(&mut self, mv: &Self::MoveType) {
        let bb = Bitboard::<7, 7>::from_square(mv.0);
        let captured = bb.adjacent() & self.get_them();
        self.pieces[self.turn] ^= bb;
        self.pieces[self.turn] ^= captured;
        self.pieces[!self.turn] ^= captured;
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let idx = (y * Self::WIDTH + x) as u8;
        if self.get_black().is_set(idx) {
            Some("x".to_owned())
        } else if self.get_white().is_set(idx) {
            Some("o".to_owned())
        } else if self.get_blockers().is_set(idx) {
            Some("-".to_owned())
        } else {
            None
        }
    }

    #[must_use]
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

    #[must_use]
    fn get_turn(&self) -> Side {
        self.turn
    }
}

impl std::fmt::Display for DroptaxxPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut height = 6;
        let mut width = 6;

        loop {
            let mut yeet = true;
            for x in 0..7 {
                let idx = 7 * height + x;
                if !self.get_blockers().is_set(idx) {
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
            for y in 0..7 {
                let idx = 7 * y + width;
                if !self.get_blockers().is_set(idx) {
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
                let idx = 7 * y + x;
                if self.get_black().is_set(idx) {
                    write!(f, "x")?;
                } else if self.get_white().is_set(idx) {
                    write!(f, "o")?;
                } else if self.get_blockers().is_set(idx) {
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
