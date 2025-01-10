use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, side::Side, square::Square},
};

#[derive(Clone)]
struct Irrecoverable {
    pieces: [Bitboard<8, 6>; 2],
    tiles: Bitboard<8, 6>,
}

#[derive(Clone, Default)]
pub struct IsolationPosition {
    stack: Vec<Irrecoverable>,
    pub pieces: [Bitboard<8, 6>; 2],
    pub tiles: Bitboard<8, 6>,
    pub turn: Side,
    pub fullmoves: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct IsolationMove {
    pub to: Square<8, 6>,
    pub remove: Square<8, 6>,
}

#[must_use]
pub fn fast_perft(
    us: Bitboard<8, 6>,
    them: Bitboard<8, 6>,
    tiles: Bitboard<8, 6>,
    depth: i32,
) -> u64 {
    if depth == 0 {
        1
    } else if depth == 1 {
        let adjacent = us.adjacent() & tiles & !them;
        let nodes = adjacent.count() * ((tiles & !them).count() - 1);
        nodes as u64
    } else {
        let mut nodes = 0u64;

        for to in us.adjacent() & tiles & !them {
            for remove in tiles & !them & !Bitboard::<8, 6>::from_square(to) {
                nodes += fast_perft(
                    them,
                    Bitboard::<8, 6>::from_square(to),
                    tiles ^ Bitboard::<8, 6>::from_square(remove),
                    depth - 1,
                );
            }
        }

        nodes
    }
}

impl IsolationPosition {
    #[must_use]
    pub fn get_us(&self) -> Bitboard<8, 6> {
        self.pieces[self.turn]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard<8, 6> {
        self.pieces[!self.turn]
    }

    #[must_use]
    pub fn get_black(&self) -> Bitboard<8, 6> {
        self.pieces[Side::Player1]
    }

    #[must_use]
    pub fn get_red(&self) -> Bitboard<8, 6> {
        self.pieces[Side::Player2]
    }

    #[must_use]
    pub fn get_tiles(&self) -> Bitboard<8, 6> {
        self.tiles
    }
}

impl GameRules for IsolationPosition {
    type MoveType = IsolationMove;
    const WIDTH: i32 = 8;
    const HEIGHT: i32 = 6;

    fn count_moves(&self) -> u64 {
        let adjacent = self.get_us().adjacent() & self.get_tiles() & !self.get_them();
        let nodes = adjacent.count() * ((self.get_tiles() & !self.get_them()).count() - 1);
        nodes as u64
    }

    fn startpos() -> Self {
        Self {
            stack: vec![],
            pieces: [Bitboard(0x800000), Bitboard(0x1000000)],
            tiles: Bitboard(0xfffffe7fffff),
            turn: Side::Player1,
            fullmoves: 1,
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, _: usize) -> bool {
        let bb = Bitboard::from_coords(x, y);
        match c {
            'P' => {
                self.pieces[0] ^= bb;
                self.tiles ^= bb;
            }
            'p' => {
                self.pieces[1] ^= bb;
                self.tiles ^= bb;
            }
            '.' => self.tiles ^= bb,
            _ => {}
        }
        true
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        for to in self.get_us().adjacent() & self.get_tiles() & !self.get_them() {
            for remove in self.get_tiles() & !self.get_them() & !Bitboard::from_square(to) {
                func(IsolationMove { to, remove });
            }
        }
    }

    fn get_result(&self) -> Option<GameResult> {
        let available = self.get_tiles() & !(self.get_us() | self.get_them());
        let can_move_us = (self.get_us().adjacent() & available).is_occupied();
        let could_move_them = (self.get_them().adjacent() & self.get_tiles()).is_occupied();
        if !can_move_us {
            Some(GameResult::Win(!self.turn))
        } else if !could_move_them {
            Some(GameResult::Win(self.turn))
        } else {
            None
        }
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        self.stack.push(Irrecoverable {
            pieces: self.pieces,
            tiles: self.tiles,
        });
        self.pieces[self.turn] = Bitboard::from_square(mv.to);
        self.tiles ^= Bitboard::from_square(mv.remove);
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undomove(&mut self, _mv: &Self::MoveType) {
        let data = self
            .stack
            .pop()
            .expect("Can't undo a move that was never made");
        self.pieces = data.pieces;
        self.tiles = data.tiles;
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
            Some("P".to_owned())
        } else if self.get_red().is_square_set(sq) {
            Some("p".to_owned())
        } else if self.get_tiles().is_square_set(sq) {
            Some(".".to_owned())
        } else {
            None
        }
    }

    fn get_fen(&self) -> String {
        format!(
            "{} {} {}",
            self.get_board_fen(),
            match self.turn {
                Side::Player1 => "b",
                Side::Player2 => "r",
            },
            self.fullmoves,
        )
    }

    fn parse_fen_part(&mut self, idx: usize, part: &str) {
        match idx {
            0 => {}
            // Side to move
            1 => match part {
                "b" => self.turn = Side::Player1,
                "r" => self.turn = Side::Player2,
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
        let start_black = Square::from_coords(7, 2);
        let start_red = Square::from_coords(0, 3);
        let is_black_on_start = self.get_black().is_square_set(start_black);
        let is_red_on_start = self.get_red().is_square_set(start_red);

        if (self.get_black() & self.get_red()).is_occupied() {
            Err("black & red overlap")
        } else if self.get_black().count() != 1 {
            Err("Must be one black piece")
        } else if self.get_red().count() != 1 {
            Err("Must be one red piece")
        } else if is_black_on_start && self.fullmoves != 1 {
            Err("Fullmove counter should be 1 if black is on the starting square")
        } else if is_red_on_start && self.fullmoves != 1 {
            Err("Fullmove counter should be 1 if red is on the starting square")
        } else if !is_black_on_start && (self.get_black() & self.get_tiles()).is_empty() {
            Err("Black must be on a tile or on the starting square")
        } else if !is_red_on_start && (self.get_red() & self.get_tiles()).is_empty() {
            Err("Red must be on a tile or on the starting square")
        } else {
            Ok(())
        }
    }

    fn perft(&mut self, depth: i32) -> u64 {
        fast_perft(self.get_us(), self.get_them(), self.get_tiles(), depth)
    }
}

impl IsolationMove {
    #[must_use]
    pub fn from_string(movestr: &str) -> Result<Self, &'static str> {
        if movestr.len() < 4 {
            Err("Wrong move string length")
        } else {
            let to = Square::<8, 6>::from_string(&movestr[0..2])?;
            let remove = Square::<8, 6>::from_string(&movestr[2..])?;
            Ok(IsolationMove { to, remove })
        }
    }
}

impl std::fmt::Display for IsolationPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..Self::HEIGHT).rev() {
            for x in 0..Self::WIDTH {
                let sq = Square::from_coords(x, y);
                if self.get_black().is_square_set(sq) {
                    write!(f, "P")?;
                } else if self.get_red().is_square_set(sq) {
                    write!(f, "p")?;
                } else if self.get_tiles().is_square_set(sq) {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }

        match self.turn {
            Side::Player1 => writeln!(f, "Turn: b")?,
            Side::Player2 => writeln!(f, "Turn: r")?,
        }

        Ok(())
    }
}
