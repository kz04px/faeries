use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, file::File, side::Side, square::Square},
};

#[derive(Clone, Default)]
pub struct Connect4Position {
    pub pieces: [Bitboard<7, 6>; 2],
    pub turn: Side,
    pub fullmoves: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Connect4Move(pub File<7>);

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
    pub fn get_both(&self) -> Bitboard<7, 6> {
        self.get_red() | self.get_yellow()
    }
}

impl GameRules for Connect4Position {
    type MoveType = Connect4Move;
    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 6;

    fn startpos() -> Self {
        Self {
            pieces: [Bitboard(0x0), Bitboard(0x0)],
            turn: Side::Player1,
            fullmoves: 1,
        }
    }

    fn count_moves(&self) -> u64 {
        if self.is_gameover() {
            0
        } else {
            (Bitboard::get_top_edge() & self.get_empty()).count() as u64
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, _: usize) -> bool {
        let bb = Bitboard::from_coords(x, y);
        match c {
            'r' => self.pieces[0] ^= bb,
            'y' => self.pieces[1] ^= bb,
            _ => panic!("Unknown piece {}", c),
        }
        true
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        if self.is_gameover() {
            return;
        }

        let mask = Bitboard::get_top_edge() & self.get_empty();
        for sq in mask {
            func(Connect4Move(sq.get_file()));
        }
    }

    fn get_result(&self) -> Option<GameResult> {
        for side in [Side::Player1, Side::Player2] {
            // Up-Down
            if (self.pieces[side]
                & self.pieces[side].north()
                & self.pieces[side].north().north()
                & self.pieces[side].north().north().north())
            .is_occupied()
            {
                return Some(GameResult::Win(side));
            }

            // Left-Right
            if (self.pieces[side]
                & self.pieces[side].east()
                & self.pieces[side].east().east()
                & self.pieces[side].east().east().east())
            .is_occupied()
            {
                return Some(GameResult::Win(side));
            }

            // UpRight-DownLeft
            if (self.pieces[side]
                & self.pieces[side].ne()
                & self.pieces[side].ne().ne()
                & self.pieces[side].ne().ne().ne())
            .is_occupied()
            {
                return Some(GameResult::Win(side));
            }

            // UpLeft-DownRight
            if (self.pieces[side]
                & self.pieces[side].nw()
                & self.pieces[side].nw().nw()
                & self.pieces[side].nw().nw().nw())
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
        let bb = (Bitboard::from_file(mv.0) & self.get_empty()).lsb_bb();
        self.pieces[self.turn] ^= bb;
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undomove(&mut self, mv: &Self::MoveType) {
        self.turn = !self.turn;
        let file = Bitboard::from_file(mv.0);
        let sq = (file & self.pieces[self.turn]).msb();
        let bb = Bitboard::from_square(sq);
        self.pieces[self.turn] ^= bb;
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
        if self.get_red().is_square_set(sq) {
            Some("r".to_owned())
        } else if self.get_yellow().is_square_set(sq) {
            Some("y".to_owned())
        } else {
            None
        }
    }

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

    fn get_turn(&self) -> Side {
        self.turn
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        if (self.get_red() & self.get_yellow()).is_occupied() {
            Err("red & yellow overlap")
        } else if (self.get_red().south() & self.get_empty()).is_occupied() {
            Err("floating red")
        } else if (self.get_yellow().south() & self.get_empty()).is_occupied() {
            Err("floating yellow")
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

                for sq in Bitboard::get_top_edge() & self.get_empty() {
                    let bb = (Bitboard::from_file(sq.get_file()) & self.get_empty()).lsb_bb();

                    // makemove
                    self.pieces[self.turn] ^= bb;
                    self.turn = !self.turn;

                    nodes += self.perft(depth - 1);

                    // undomove
                    self.turn = !self.turn;
                    self.pieces[self.turn] ^= bb;
                }

                nodes
            }
        }
    }
}

impl Connect4Move {
    #[must_use]
    pub fn from_string(movestr: &str) -> Result<Self, &'static str> {
        let f = File::<7>::from_char(movestr.chars().nth(0).unwrap())?;
        Ok(Connect4Move(f))
    }
}

impl std::fmt::Display for Connect4Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..Self::HEIGHT).rev() {
            for x in 0..Self::WIDTH {
                let sq = Square::from_coords(x, y);
                if self.get_red().is_square_set(sq) {
                    write!(f, "r")?;
                } else if self.get_yellow().is_square_set(sq) {
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
