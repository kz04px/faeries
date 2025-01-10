use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, file::File, rank::Rank, side::Side, square::Square},
};

#[derive(Clone, Copy)]
struct Irrecoverable {
    pub pieces: [Bitboard<7, 7>; 2],
    pub halfmoves: i32,
}

#[derive(Clone, Default)]
pub struct AtaxxPosition {
    stack: Vec<Irrecoverable>,
    pub pieces: [Bitboard<7, 7>; 2],
    pub blockers: Bitboard<7, 7>,
    pub turn: Side,
    pub halfmoves: i32,
    pub fullmoves: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AtaxxMove {
    Pass,
    Single(Square<7, 7>),
    Double(Square<7, 7>, Square<7, 7>),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Black,
    White,
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
    pub fn get_both(&self) -> Bitboard<7, 7> {
        self.get_black() | self.get_white()
    }

    #[must_use]
    pub fn get_blockers(&self) -> Bitboard<7, 7> {
        self.blockers
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard<7, 7> {
        !(self.get_black() | self.get_white() | self.get_blockers())
    }

    #[must_use]
    pub fn get_piece_on(&self, sq: Square<7, 7>) -> Option<Piece> {
        if self.get_black().is_square_set(sq) {
            Some(Piece::Black)
        } else if self.get_white().is_square_set(sq) {
            Some(Piece::White)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_piece_on_optional(&self, sq: Option<Square<7, 7>>) -> Option<Piece> {
        if sq.is_none() {
            None
        } else {
            self.get_piece_on(sq.unwrap())
        }
    }
}

impl GameRules for AtaxxPosition {
    type MoveType = AtaxxMove;
    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 7;

    fn startpos() -> Self {
        Self {
            stack: vec![],
            pieces: [Bitboard(0x40000000040), Bitboard(0x1000000000001)],
            blockers: Bitboard(0x0),
            turn: Side::Player1,
            halfmoves: 0,
            fullmoves: 1,
        }
    }

    fn count_moves(&self) -> u64 {
        if self.is_gameover() {
            0
        } else {
            let mut nodes = 0;

            // Single moves
            nodes += (self.get_us().adjacent() & self.get_empty()).count();

            // Double moves
            for from in self.get_us() {
                nodes += (Bitboard::from_square(from).dist2() & self.get_empty()).count();
            }

            // Pass
            if nodes == 0 {
                nodes = 1;
            }

            nodes as u64
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
        if self.is_gameover() {
            return;
        }

        let mut num_moves = 0;

        // Singles
        for sq in self.get_us().adjacent() & self.get_empty() {
            func(Self::MoveType::Single(sq));
            num_moves += 1;
        }

        // Doubles
        for fr in self.get_us() {
            for to in Bitboard::from_square(fr).dist2() & self.get_empty() {
                func(Self::MoveType::Double(fr, to));
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
        self.stack.push(Irrecoverable {
            pieces: self.pieces,
            halfmoves: self.halfmoves,
        });
        match mv {
            AtaxxMove::Pass => self.halfmoves += 1,
            AtaxxMove::Single(to) => {
                let bb = Bitboard::from_square(*to);
                let captured = bb.adjacent() & self.get_them();
                self.pieces[self.turn] ^= bb;
                self.pieces[self.turn] ^= captured;
                self.pieces[!self.turn] ^= captured;
                self.halfmoves = 0;
            }
            AtaxxMove::Double(fr, to) => {
                let bb = Bitboard::from_square(*to);
                let captured = bb.adjacent() & self.get_them();
                self.pieces[self.turn] ^= bb;
                self.pieces[self.turn] ^= Bitboard::from_square(*fr);
                self.pieces[self.turn] ^= captured;
                self.pieces[!self.turn] ^= captured;
                self.halfmoves += 1;
            }
        }
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undomove(&mut self, _mv: &Self::MoveType) {
        let data = self
            .stack
            .pop()
            .expect("Can't undo a move that was never made");
        self.pieces = data.pieces;
        self.halfmoves = data.halfmoves;
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
                _ => panic!("Unknown side to move token: {}", part),
            },
            // Halfmoves
            2 => self.halfmoves = part.parse::<i32>().unwrap(),
            // Fullmoves
            3 => self.fullmoves = part.parse::<i32>().unwrap(),
            _ => panic!("Invalid fen part index: {}", idx),
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
}

impl AtaxxMove {
    #[must_use]
    pub fn from_string(movestr: &str) -> Result<Self, &'static str> {
        if movestr == "0000" {
            return Ok(AtaxxMove::Pass);
        }

        match movestr.len() {
            2 => {
                let to = Square::from_string(&movestr[0..2])?;
                Ok(AtaxxMove::Single(to))
            }
            4 => {
                let fr = Square::from_string(&movestr[0..2])?;
                let to = Square::from_string(&movestr[2..4])?;
                Ok(AtaxxMove::Double(fr, to))
            }
            _ => Err("Illegal move string"),
        }
    }
}

impl std::fmt::Display for AtaxxPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut height = 6;
        let mut width = 6;
        let keep = self.get_black() | self.get_white() | self.get_empty();

        while height > 0 {
            let rank_bb = Bitboard::from_rank(Rank(height));
            if (rank_bb & keep).is_occupied() {
                break;
            }
            height -= 1;
        }

        while width > 0 {
            let rank_bb = Bitboard::from_file(File(width));
            if (rank_bb & keep).is_occupied() {
                break;
            }
            width -= 1;
        }

        for y in (0..=height).rev() {
            for x in 0..=width {
                let sq = Square::from_coords(x as i32, y as i32);
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
