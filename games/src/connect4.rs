use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, side::Side, square::Square},
};

#[derive(Clone, Copy, Default)]
pub struct Connect4Position {
    pub pieces: [Bitboard<7, 6>; 2],
    pub turn: Side,
    pub fullmoves: i32,
}

#[derive(Clone, Copy)]
pub struct Connect4Move(pub u8);

impl Connect4Position {
    #[must_use]
    pub fn get_us(&self) -> Bitboard<7, 6> {
        self.pieces[self.turn]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard<7, 6> {
        self.pieces[!self.turn]
    }

    #[must_use]
    pub fn get_red(&self) -> Bitboard<7, 6> {
        self.pieces[Side::Player1]
    }

    #[must_use]
    pub fn get_yellow(&self) -> Bitboard<7, 6> {
        self.pieces[Side::Player2]
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard<7, 6> {
        !(self.get_red() | self.get_yellow())
    }

    #[must_use]
    pub fn get_occupied(&self) -> Bitboard<7, 6> {
        self.get_red() | self.get_yellow()
    }
}

impl GameRules for Connect4Position {
    type MoveType = Connect4Move;
    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 6;

    fn startpos() -> Self {
        Self {
            pieces: [Bitboard::<7, 6>(0x0), Bitboard::<7, 6>(0x0)],
            turn: Side::Player1,
            fullmoves: 1,
        }
    }

    #[must_use]
    fn count_moves(&self) -> u64 {
        if self.is_gameover() {
            0
        } else {
            let mask = (!self.get_empty().south().north() & self.get_empty())
                | (self.get_occupied().north() & self.get_empty());
            mask.count() as u64
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char) {
        let bb = Bitboard::<7, 6>::from_coords(x, y);
        match c {
            'r' => self.pieces[0] ^= bb,
            'y' => self.pieces[1] ^= bb,
            _ => panic!("Unknown piece {}", c),
        }
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType)) {
        if self.is_gameover() {
            return;
        }

        let mask = (!self.get_empty().south().north() & self.get_empty())
            | (self.get_occupied().north() & self.get_empty());
        for sq in mask {
            func(Connect4Move(sq % 7));
        }
    }

    fn get_result(&self) -> Option<GameResult> {
        for side in [Side::Player1, Side::Player2] {
            // Up-Down
            if (self.pieces[side as usize]
                & self.pieces[side as usize].north()
                & self.pieces[side as usize].north().north()
                & self.pieces[side as usize].north().north().north())
            .is_occupied()
            {
                return Some(GameResult::Win(side));
            }

            // Left-Right
            if (self.pieces[side as usize]
                & self.pieces[side as usize].east()
                & self.pieces[side as usize].east().east()
                & self.pieces[side as usize].east().east().east())
            .is_occupied()
            {
                return Some(GameResult::Win(side));
            }

            // UpRight-DownLeft
            if (self.pieces[side as usize]
                & self.pieces[side as usize].ne()
                & self.pieces[side as usize].ne().ne()
                & self.pieces[side as usize].ne().ne().ne())
            .is_occupied()
            {
                return Some(GameResult::Win(side));
            }

            // UpLeft-DownRight
            if (self.pieces[side as usize]
                & self.pieces[side as usize].nw()
                & self.pieces[side as usize].nw().nw()
                & self.pieces[side as usize].nw().nw().nw())
            .is_occupied()
            {
                return Some(GameResult::Win(side));
            }
        }

        if self.get_empty().is_empty() {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        let file = Bitboard::<7, 6>::file_of_index(mv.0);
        let legal = (!self.get_empty().south().north() & self.get_empty())
            | (self.get_occupied().north() & self.get_empty());
        let bb = file & legal;
        self.pieces[self.turn] ^= bb;
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let sq = Square::<7, 6>::from_coords(x, y);

        if self.get_red().is_set(sq.0) {
            Some("r".to_string())
        } else if self.get_yellow().is_set(sq.0) {
            Some("y".to_string())
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
                Side::Player1 => "r",
                Side::Player2 => "y",
            },
            self.fullmoves
        )
    }

    fn parse_fen_part(&mut self, idx: usize, part: &str) {
        match idx {
            0 => {}
            1 => match part {
                "r" => self.turn = Side::Player1,
                "y" => self.turn = Side::Player2,
                _ => panic!("Uh oh {}", part),
            },
            2 => self.fullmoves = part.parse::<i32>().unwrap(),
            _ => panic!("Invalid fen part index"),
        }
    }

    #[must_use]
    fn get_turn(&self) -> Side {
        self.turn
    }
}

impl std::fmt::Display for Connect4Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..6).rev() {
            for x in 0..7 {
                let idx = 7 * y + x;
                if self.get_red().is_set(idx) {
                    write!(f, "r")?;
                } else if self.get_yellow().is_set(idx) {
                    write!(f, "y")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        match self.turn {
            Side::Player1 => writeln!(f, "Turn: r")?,
            Side::Player2 => writeln!(f, "Turn: y")?,
        }

        Ok(())
    }
}
