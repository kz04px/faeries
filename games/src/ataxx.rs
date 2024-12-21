use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, side::Side, square::Square},
};

#[derive(Clone, Copy, Default)]
pub struct AtaxxPosition {
    pub pieces: [Bitboard<7, 7>; 2],
    pub blockers: Bitboard<7, 7>,
    pub turn: Side,
    pub halfmoves: i32,
    pub fullmoves: i32,
}

#[derive(Clone, Copy)]
pub enum AtaxxMove {
    Pass,
    Single(Square<7, 7>),
    Double(Square<7, 7>, Square<7, 7>),
}

impl AtaxxPosition {
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

impl GameRules for AtaxxPosition {
    type MoveType = AtaxxMove;
    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 7;

    fn startpos() -> Self {
        Self {
            pieces: [
                Bitboard::<7, 7>(0x40000000040),
                Bitboard::<7, 7>(0x1000000000001),
            ],
            blockers: Bitboard::<7, 7>(0x0),
            turn: Side::Player1,
            halfmoves: 0,
            fullmoves: 1,
        }
    }

    #[must_use]
    fn count_moves(&self) -> u64 {
        if self.is_gameover() {
            0
        } else {
            let mut nodes = 0;

            // Single moves
            nodes += (self.get_us().adjacent() & self.get_empty()).count();

            // Double moves
            for from in self.get_us() {
                nodes += (Bitboard::<7, 7>::from_index(from).dist2() & self.get_empty()).count();
            }

            // Pass
            if nodes == 0 {
                nodes = 1;
            }

            nodes as u64
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
        if self.is_gameover() {
            return;
        }

        let mut num_moves = 0;

        // Singles
        for sq in self.get_us().adjacent() & self.get_empty() {
            func(Self::MoveType::Single(Square::<7, 7>(sq)));
            num_moves += 1;
        }

        // Doubles
        for fr in self.get_us() {
            for to in Bitboard::<7, 7>::from_index(fr).dist2() & self.get_empty() {
                func(Self::MoveType::Double(
                    Square::<7, 7>(fr),
                    Square::<7, 7>(to),
                ));
                num_moves += 1;
            }
        }

        // Pass
        if num_moves == 0 {
            func(Self::MoveType::Pass);
        }
    }

    fn is_gameover(&self) -> bool {
        let both = self.get_black() | self.get_white();
        let reachable = (both.adjacent() | both.dist2()) & self.get_empty();
        self.halfmoves >= 100
            || self.get_black().is_empty()
            || self.get_white().is_empty()
            || reachable.is_empty()
    }

    fn get_result(&self) -> Option<GameResult> {
        let both = self.get_black() | self.get_white();
        let reachable = (both.adjacent() | both.dist2()) & self.get_empty();
        let num_player1 = self.get_black().count();
        let num_player2 = self.get_white().count();

        if num_player1 > 0 && num_player2 == 0 {
            Some(GameResult::Win(Side::Player1))
        } else if num_player2 > 0 && num_player1 == 0 {
            Some(GameResult::Win(Side::Player2))
        } else if num_player1 == 0 && num_player2 == 0 {
            Some(GameResult::Draw)
        } else if self.halfmoves >= 100 {
            Some(GameResult::Draw)
        } else if reachable.is_occupied() {
            None
        } else if num_player1 > num_player2 {
            Some(GameResult::Win(Side::Player1))
        } else if num_player2 > num_player1 {
            Some(GameResult::Win(Side::Player2))
        } else {
            Some(GameResult::Draw)
        }
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        match mv {
            AtaxxMove::Pass => self.halfmoves += 1,
            AtaxxMove::Single(to) => {
                let bb = Bitboard::<7, 7>::from_square(*to);
                let captured = bb.adjacent() & self.get_them();
                self.pieces[self.turn] ^= bb;
                self.pieces[self.turn] ^= captured;
                self.pieces[!self.turn] ^= captured;
                self.halfmoves = 0;
            }
            AtaxxMove::Double(fr, to) => {
                let bb = Bitboard::<7, 7>::from_square(*to);
                let captured = bb.adjacent() & self.get_them();
                self.pieces[self.turn] ^= bb;
                self.pieces[self.turn] ^= Bitboard::<7, 7>::from_square(*fr);
                self.pieces[self.turn] ^= captured;
                self.pieces[!self.turn] ^= captured;
                self.halfmoves += 1;
            }
        }
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
            "{} {} {} {}",
            self.get_board_fen(),
            match self.turn {
                Side::Player1 => "x",
                Side::Player2 => "o",
            },
            self.halfmoves,
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
                _ => panic!("Uh oh"),
            },
            // Halfmoves
            2 => self.halfmoves = part.parse::<i32>().unwrap(),
            // Fullmoves
            3 => self.fullmoves = part.parse::<i32>().unwrap(),
            _ => panic!("Invalid fen part index"),
        }
    }

    #[must_use]
    fn get_turn(&self) -> Side {
        self.turn
    }
}

impl std::fmt::Display for AtaxxPosition {
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
