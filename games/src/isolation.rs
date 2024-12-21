use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, side::Side, square::Square},
};

#[derive(Clone, Copy, Default)]
pub struct IsolationPosition {
    pub pieces: [Bitboard<8, 6>; 2],
    pub tiles: Bitboard<8, 6>,
    pub turn: Side,
    pub fullmoves: i32,
}

#[derive(Clone, Copy)]
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
        return 1;
    } else if depth == 1 {
        let adjacent = us.adjacent() & tiles & !them;
        let nodes = adjacent.count() * ((tiles & !them).count() - 1);
        return nodes as u64;
    }

    let mut nodes = 0u64;

    for to in us.adjacent() & tiles & !them {
        for remove in tiles & !them & !Bitboard::<8, 6>::from_index(to) {
            nodes += fast_perft(
                them,
                Bitboard::<8, 6>::from_index(to),
                tiles ^ Bitboard::<8, 6>::from_index(remove),
                depth - 1,
            );
        }
    }

    nodes
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

    #[must_use]
    fn count_moves(&self) -> u64 {
        let adjacent = self.get_us().adjacent() & self.get_tiles() & !self.get_them();
        let nodes = adjacent.count() * ((self.get_tiles() & !self.get_them()).count() - 1);
        nodes as u64
    }

    fn startpos() -> Self {
        Self {
            pieces: [Bitboard::<8, 6>(0x800000), Bitboard::<8, 6>(0x1000000)],
            tiles: Bitboard::<8, 6>(0xfffffe7fffff),
            turn: Side::Player1,
            fullmoves: 1,
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char) {
        let bb = Bitboard::<8, 6>::from_coords(x, y);
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
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType)) {
        for to in self.get_us().adjacent() & self.get_tiles() & !self.get_them() {
            for r in self.get_tiles() & !self.get_them() & !Bitboard::<8, 6>::from_index(to) {
                func(IsolationMove {
                    to: Square::<8, 6>(to),
                    remove: Square::<8, 6>(r),
                })
            }
        }
    }

    fn get_result(&self) -> Option<GameResult> {
        let moves_us = self.get_us().adjacent() & self.get_tiles();
        let moves_them = self.get_them().adjacent() & self.get_tiles();

        if (moves_us & !self.get_them()).is_empty() && (moves_them & !self.get_us()).is_empty() {
            Some(GameResult::Draw)
        } else if (moves_us & !self.get_us()).is_occupied() && moves_them.is_empty() {
            Some(GameResult::Win(self.turn))
        } else if (moves_us & !self.get_them()).is_empty() && moves_them.is_occupied() {
            Some(GameResult::Win(!self.turn))
        } else {
            None
        }
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        self.pieces[self.turn] = Bitboard::<8, 6>::from_square(mv.to);
        self.tiles ^= Bitboard::<8, 6>::from_square(mv.remove);
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let idx = (y * Self::WIDTH + x) as u8;
        if self.get_black().is_set(idx) {
            Some("P".to_owned())
        } else if self.get_red().is_set(idx) {
            Some("p".to_owned())
        } else if self.get_tiles().is_set(idx) {
            Some(".".to_owned())
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

    #[must_use]
    fn get_turn(&self) -> Side {
        self.turn
    }
}

impl std::fmt::Display for IsolationPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..6).rev() {
            for x in 0..8 {
                let idx = 8 * y + x;
                if self.get_black().is_set(idx) {
                    write!(f, "P")?;
                } else if self.get_red().is_set(idx) {
                    write!(f, "p")?;
                } else if self.get_tiles().is_set(idx) {
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
