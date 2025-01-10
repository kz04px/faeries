use crate::{
    gamerules::{GameResult, GameRules},
    general::{
        bitboard::{Bitboard, floodfill},
        side::Side,
        square::Square,
    },
};

#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Flat,
    Standing,
    Cap,
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Stack {
    set: u64,
    height: u8,
}

#[derive(Clone)]
pub struct TakPosition<const S: usize> {
    pub stacks: [[Stack; S]; S],
    pub num_pieces: [i32; 2],
    pub num_caps: [i32; 2],
    pub turn: Side,
    pub fullmoves: i32,
    pub piece_masks: [Bitboard<S, S>; 3],
    pub side_masks: [Bitboard<S, S>; 2],
}

#[derive(Clone, Copy, Debug)]
pub enum TakMove<const S: usize> {
    Drop(Square<S, S>, PieceType),
    Spread(Square<S, S>, Dir, u8, u8, bool),
}

impl Stack {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.height == 0
    }

    #[must_use]
    pub fn is_occupied(&self) -> bool {
        !self.is_empty()
    }

    #[must_use]
    pub fn is_player1(&self, idx: usize) -> bool {
        (self.set >> idx) & 1 == 1
    }

    #[must_use]
    pub fn get_owner(&self) -> Option<Side> {
        if self.is_empty() {
            None
        } else if (self.set >> (self.height - 1)) & 1 == 1 {
            Some(Side::Player1)
        } else {
            Some(Side::Player2)
        }
    }

    #[must_use]
    pub fn get_height(&self) -> u8 {
        self.height
    }

    #[must_use]
    pub fn pop_top(&mut self, count: usize) -> u64 {
        debug_assert!(count <= self.height as usize);
        let new_height = self.height as usize - count;
        let top = self.set >> new_height;
        self.set &= (1 << new_height) - 1;
        self.height = new_height as u8;
        top
    }

    pub fn push_top(&mut self, top: u64, count: usize) {
        self.set ^= top << self.height;
        self.height += count as u8;
    }

    pub fn pop(&mut self) {
        debug_assert!(self.height > 0);
        self.height -= 1;
        self.set &= !(1 << self.height);
    }

    pub fn pop_n(&mut self, n: u8) {
        debug_assert!(self.height >= n);
        self.height -= n as u8;
        self.set &= (0x1u64 << self.height) - 1;
    }

    pub fn push(&mut self, is_p1: bool) {
        self.set ^= (is_p1 as u64) << self.height;
        self.height += 1;
    }
}

impl<const SIZE: usize> Default for TakPosition<SIZE> {
    fn default() -> Self {
        Self {
            stacks: [[Stack::default(); SIZE]; SIZE],
            num_pieces: match SIZE {
                3 => [10, 10],
                4 => [15, 15],
                5 => [21, 21],
                6 => [30, 30],
                7 => [40, 40],
                8 => [50, 50],
                _ => panic!("Uh oh"),
            },
            num_caps: match SIZE {
                3 => [0, 0],
                4 => [0, 0],
                5 => [1, 1],
                6 => [1, 1],
                7 => [2, 2],
                8 => [2, 2],
                _ => panic!("Uh oh"),
            },
            turn: Side::Player1,
            fullmoves: 1,
            piece_masks: [Bitboard::<SIZE, SIZE>(0x0); 3],
            side_masks: [Bitboard::<SIZE, SIZE>(0x0); 2],
        }
    }
}

impl<const SIZE: usize> TakPosition<SIZE> {
    #[must_use]
    pub fn get_height(&self, sq: Square<SIZE, SIZE>) -> i32 {
        self.stacks[sq.get_file()][sq.get_rank()].get_height() as i32
    }

    #[must_use]
    pub fn get_piece_on(&self, sq: Square<SIZE, SIZE>) -> Option<PieceType> {
        if self.piece_masks[PieceType::Cap as usize].is_square_set(sq) {
            Some(PieceType::Cap)
        } else if self.piece_masks[PieceType::Standing as usize].is_square_set(sq) {
            Some(PieceType::Standing)
        } else if self.stacks[sq.get_file()][sq.get_rank()].is_occupied() {
            Some(PieceType::Flat)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_owner(&self, sq: Square<SIZE, SIZE>) -> Option<Side> {
        self.stacks[sq.get_file()][sq.get_rank()].get_owner()
    }

    #[must_use]
    pub fn get_us(&self) -> Bitboard<SIZE, SIZE> {
        self.side_masks[self.turn]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard<SIZE, SIZE> {
        self.side_masks[!self.turn]
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard<SIZE, SIZE> {
        !(self.side_masks[Side::Player1] | self.side_masks[Side::Player2])
    }

    #[must_use]
    fn stones_played(&self) -> i32 {
        -self.num_pieces[Side::Player1]
            - self.num_pieces[Side::Player2]
            - self.num_caps[Side::Player1]
            - self.num_caps[Side::Player2]
            + match SIZE {
                3 => 10 + 10,
                4 => 15 + 15,
                5 => 21 + 21 + 1 + 1,
                6 => 30 + 30 + 1 + 1,
                7 => 40 + 40 + 2 + 2,
                8 => 50 + 50 + 2 + 2,
                _ => panic!("Uh oh"),
            }
    }
}

impl<const SIZE: usize> GameRules for TakPosition<SIZE> {
    type MoveType = TakMove<SIZE>;
    const WIDTH: i32 = 0;
    const HEIGHT: i32 = 0;

    fn startpos() -> Self {
        TakPosition::default()
    }

    fn set_piece(&mut self, _x: i32, _y: i32, _c: char, _idx: usize) -> bool {
        panic!("Not needed");
    }

    fn set_fen(&mut self, fen: &str) {
        if fen == "startpos" {
            *self = Self::startpos();
            return;
        }

        *self = Self::default();
        let parts: Vec<&str> = fen.split(' ').collect();

        let mut x: i32 = 0;
        let mut y: i32 = SIZE as i32 - 1;
        let mut is_spaces = false;
        let mut last_side = Side::Player1;
        for c in parts[0].chars() {
            let idx = (y * SIZE as i32 + x) as usize;
            match c {
                '1'..='9' => {
                    if is_spaces {
                        x += c.to_string().parse::<i32>().unwrap() - 1;
                        is_spaces = false;
                    } else if c == '1' {
                        self.stacks[x as usize][y as usize].push(true);
                        self.num_pieces[Side::Player1] -= 1;
                        last_side = Side::Player1;
                        self.piece_masks[PieceType::Flat as usize] |= Bitboard::from_index(idx);
                        self.side_masks[Side::Player1] |= Bitboard::from_index(idx);
                        self.side_masks[Side::Player2] &= !Bitboard::from_index(idx);
                    } else if c == '2' {
                        self.stacks[x as usize][y as usize].push(false);
                        self.num_pieces[Side::Player2] -= 1;
                        last_side = Side::Player2;
                        self.piece_masks[PieceType::Flat as usize] |= Bitboard::from_index(idx);
                        self.side_masks[Side::Player1] &= !Bitboard::from_index(idx);
                        self.side_masks[Side::Player2] |= Bitboard::from_index(idx);
                    } else {
                        panic!("Illegal fen character");
                    }
                }
                'C' => {
                    self.piece_masks[PieceType::Flat as usize] &= !Bitboard::from_index(idx);
                    self.piece_masks[PieceType::Cap as usize] ^= Bitboard::from_index(idx);
                    self.num_pieces[last_side as usize] += 1;
                    self.num_caps[last_side as usize] -= 1;
                }
                'S' => {
                    self.piece_masks[PieceType::Flat as usize] &= !Bitboard::from_index(idx);
                    self.piece_masks[PieceType::Standing as usize] ^= Bitboard::from_index(idx);
                }
                '/' => {
                    x = 0;
                    y -= 1;
                    is_spaces = false;
                }
                'x' => is_spaces = true,
                ',' => {
                    x += 1;
                    is_spaces = false;
                }
                _ => panic!("Unknown fen character"),
            }
        }

        // Side to move
        match parts[1] {
            "1" => self.turn = Side::Player1,
            "2" => self.turn = Side::Player2,
            _ => panic!("Unknown side to move"),
        }

        // Fullmoves
        if let Ok(moves) = parts[2].parse::<i32>() {
            self.fullmoves = moves;
        } else {
            panic!("Uh oh");
        }

        debug_assert!(
            self.is_valid().is_ok(),
            "set_fen error: {}",
            self.is_valid().unwrap_err()
        );
    }

    fn get_board_fen(&self) -> String {
        let mut fen = String::from("");

        for y in (0..SIZE).rev() {
            let mut spaces = 0;

            for x in 0..SIZE {
                if let Some(a) = self.get_square_string(x as i32, y as i32) {
                    if spaces > 0 {
                        fen += "x";
                        if spaces > 1 {
                            fen += &spaces.to_string();
                        }
                        if x < SIZE {
                            fen += ",";
                        }
                        spaces = 0;
                    }
                    fen += &a;

                    if x + 1 < SIZE {
                        fen += ",";
                    }
                } else {
                    spaces += 1;
                }
            }

            if spaces > 0 {
                fen += "x";
                if spaces > 1 {
                    fen += &spaces.to_string();
                }
            }

            if y > 0 {
                fen += "/";
            }
        }

        fen
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        if self.stacks[x as usize][y as usize].is_empty() {
            None
        } else {
            let mut out = String::new();
            for i in 0..self.stacks[x as usize][y as usize].height {
                if self.stacks[x as usize][y as usize].is_player1(i as usize) {
                    out += "1";
                } else {
                    out += "2";
                }
            }

            if self.piece_masks[PieceType::Cap as usize].is_index_set(y * SIZE as i32 + x) {
                out += "C";
            }

            if self.piece_masks[PieceType::Standing as usize].is_index_set(y * SIZE as i32 + x) {
                out += "S";
            }

            Some(out)
        }
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        if self.is_gameover() {
            return;
        }

        let is_opening = self.stones_played() < 2;

        // Drops
        for sq in self.get_empty() {
            if self.num_pieces[self.turn] > 0 {
                func(Self::MoveType::Drop(sq, PieceType::Flat));
            }
            if !is_opening && self.num_pieces[self.turn] > 0 {
                func(Self::MoveType::Drop(sq, PieceType::Standing));
            }
            if !is_opening && self.num_caps[self.turn] > 0 {
                func(Self::MoveType::Drop(sq, PieceType::Cap));
            }
        }

        if is_opening {
            return;
        }

        // Unstack
        for sq in self.get_us() {
            let x = sq.get_file().0 as usize;
            let y = sq.get_rank().0 as usize;
            let stack_height = self.stacks[x][y].get_height() as usize;
            let max_picked = SIZE.min(stack_height);
            let stack_offset = 8 - max_picked;

            debug_assert!(stack_height > 0);
            debug_assert!(max_picked > 0);
            debug_assert!(max_picked <= SIZE);
            debug_assert!(max_picked <= stack_height);
            debug_assert!(stack_offset < 8);

            // Up
            let dist_up = {
                let mut i = y + 1;
                while i < SIZE {
                    match self.get_piece_on(Square::from_coords(x as i32, i as i32)) {
                        Some(PieceType::Flat) => {}
                        Some(PieceType::Standing) => break,
                        Some(PieceType::Cap) => break,
                        None => {}
                    }
                    i += 1;
                }
                i - y - 1
            };
            debug_assert!(y + dist_up < SIZE);
            if dist_up > 0 {
                for perm in 0b1..=(0b1111_1111 >> stack_offset) as u8 {
                    if perm.count_ones() <= dist_up as u32 {
                        let height = max_picked as u8 - perm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Up, height, perm, false));
                    }
                }
            }

            let can_crush_up = y + dist_up + 1 < SIZE
                && dist_up + 1 <= stack_height
                && self.get_piece_on(sq) == Some(PieceType::Cap)
                && self.get_piece_on(Square::from_coords(x as i32, (y + dist_up + 1) as i32))
                    == Some(PieceType::Standing);
            if can_crush_up {
                for perm in 0b0..=(0b0111_1111 >> stack_offset) as u8 {
                    let nperm = (0b1000_0000 >> stack_offset) | perm;
                    if nperm.count_ones() == dist_up as u32 + 1 {
                        let height = max_picked as u8 - nperm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Up, height, nperm, true));
                    }
                }
            }

            // Down
            let dist_down = {
                let mut i = y as i32 - 1;
                while i >= 0 {
                    match self.get_piece_on(Square::from_coords(x as i32, i as i32)) {
                        Some(PieceType::Flat) => {}
                        Some(PieceType::Standing) => break,
                        Some(PieceType::Cap) => break,
                        None => {}
                    }
                    i -= 1;
                }
                y - (i + 1) as usize
            };
            debug_assert!(y >= dist_down);
            if dist_down > 0 {
                for perm in 0b1..=(0b1111_1111 >> stack_offset) as u8 {
                    if perm.count_ones() <= dist_down as u32 {
                        let height = max_picked as u8 - perm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Down, height, perm, false));
                    }
                }
            }

            let can_crush_down = y >= dist_down + 1
                && dist_down + 1 <= stack_height
                && self.get_piece_on(sq) == Some(PieceType::Cap)
                && self.get_piece_on(Square::from_coords(x as i32, (y - dist_down - 1) as i32))
                    == Some(PieceType::Standing);
            if can_crush_down {
                for perm in 0b0..=(0b0111_1111 >> stack_offset) as u8 {
                    let nperm = (0b1000_0000 >> stack_offset) | perm;
                    if nperm.count_ones() == dist_down as u32 + 1 {
                        let height = max_picked as u8 - nperm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Down, height, nperm, true));
                    }
                }
            }

            // Right
            let dist_right = {
                let mut i = x + 1;
                while i < SIZE {
                    match self.get_piece_on(Square::from_coords(i as i32, y as i32)) {
                        Some(PieceType::Flat) => {}
                        Some(PieceType::Standing) => break,
                        Some(PieceType::Cap) => break,
                        None => {}
                    }
                    i += 1;
                }
                i - x - 1
            };
            debug_assert!(x + dist_right < SIZE);
            if dist_right > 0 {
                for perm in 0b1..=(0b1111_1111 >> stack_offset) as u8 {
                    if perm.count_ones() <= dist_right as u32 {
                        let height = max_picked as u8 - perm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Right, height, perm, false));
                    }
                }
            }

            let can_crush_right = x + dist_right + 1 < SIZE
                && dist_right + 1 <= stack_height
                && self.get_piece_on(sq) == Some(PieceType::Cap)
                && self.get_piece_on(Square::from_coords((x + dist_right + 1) as i32, y as i32))
                    == Some(PieceType::Standing);
            if can_crush_right {
                for perm in 0b0..=(0b0111_1111 >> stack_offset) as u8 {
                    let nperm = (0b1000_0000 >> stack_offset) | perm;
                    if nperm.count_ones() == dist_right as u32 + 1 {
                        let height = max_picked as u8 - nperm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Right, height, nperm, true));
                    }
                }
            }

            // Left
            let dist_left = {
                let mut i = x as i32 - 1;
                while i >= 0 {
                    match self.get_piece_on(Square::from_coords(i, y as i32)) {
                        Some(PieceType::Flat) => {}
                        Some(PieceType::Standing) => break,
                        Some(PieceType::Cap) => break,
                        None => {}
                    }
                    i -= 1;
                }
                x - (i + 1) as usize
            };
            debug_assert!(x >= dist_left);
            if dist_left > 0 {
                for perm in 0b1..=(0b1111_1111 >> stack_offset) as u8 {
                    if perm.count_ones() <= dist_left as u32 {
                        let height = max_picked as u8 - perm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Left, height, perm, false));
                    }
                }
            }

            let can_crush_left = x >= dist_left + 1
                && dist_left + 1 <= stack_height
                && self.get_piece_on(sq) == Some(PieceType::Cap)
                && self.get_piece_on(Square::from_coords((x - dist_left - 1) as i32, y as i32))
                    == Some(PieceType::Standing);
            if can_crush_left {
                for perm in 0b0..=(0b0111_1111 >> stack_offset) as u8 {
                    let nperm = (0b1000_0000 >> stack_offset) | perm;
                    if nperm.count_ones() == dist_left as u32 + 1 {
                        let height = max_picked as u8 - nperm.trailing_zeros() as u8;
                        debug_assert!(height <= stack_height as u8);
                        func(Self::MoveType::Spread(sq, Dir::Left, height, nperm, true));
                    }
                }
            }
        }
    }

    fn get_turn(&self) -> Side {
        self.turn
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        let is_flipped = self.stones_played() < 2;

        match mv {
            Self::MoveType::Drop(sq, PieceType::Flat) => {
                debug_assert!(self.get_owner(*sq).is_none());
                debug_assert!(self.get_piece_on(*sq).is_none());
                debug_assert!(!self.piece_masks[PieceType::Flat as usize].is_square_set(*sq));
                debug_assert!(!self.piece_masks[PieceType::Standing as usize].is_square_set(*sq));
                debug_assert!(!self.piece_masks[PieceType::Cap as usize].is_square_set(*sq));

                let play_side = if is_flipped { !self.turn } else { self.turn };
                self.stacks[sq.get_file()][sq.get_rank()].push(play_side == Side::Player1);
                self.piece_masks[PieceType::Flat as usize] ^= Bitboard::from_square(*sq);
                self.side_masks[play_side] ^= Bitboard::from_square(*sq);
                self.num_pieces[self.turn] -= 1;
            }
            Self::MoveType::Drop(sq, PieceType::Standing) => {
                debug_assert!(self.get_owner(*sq).is_none());
                debug_assert!(self.get_piece_on(*sq).is_none());
                debug_assert!(!self.piece_masks[PieceType::Flat as usize].is_square_set(*sq));
                debug_assert!(!self.piece_masks[PieceType::Standing as usize].is_square_set(*sq));
                debug_assert!(!self.piece_masks[PieceType::Cap as usize].is_square_set(*sq));

                self.stacks[sq.get_file()][sq.get_rank()].push(self.turn == Side::Player1);
                self.piece_masks[PieceType::Standing as usize] ^= Bitboard::from_square(*sq);
                self.side_masks[self.turn as usize] ^= Bitboard::from_square(*sq);
                self.num_pieces[self.turn] -= 1;
            }
            Self::MoveType::Drop(sq, PieceType::Cap) => {
                debug_assert!(self.get_owner(*sq).is_none());
                debug_assert!(self.get_piece_on(*sq).is_none());
                debug_assert!(!self.piece_masks[PieceType::Flat as usize].is_square_set(*sq));
                debug_assert!(!self.piece_masks[PieceType::Standing as usize].is_square_set(*sq));
                debug_assert!(!self.piece_masks[PieceType::Cap as usize].is_square_set(*sq));

                self.stacks[sq.get_file()][sq.get_rank()].push(self.turn == Side::Player1);
                self.piece_masks[PieceType::Cap as usize] ^= Bitboard::from_square(*sq);
                self.side_masks[self.turn as usize] ^= Bitboard::from_square(*sq);
                self.num_caps[self.turn] -= 1;
            }
            &Self::MoveType::Spread(sq, dir, height, mut spread, is_crush) => {
                debug_assert!(height > 0);
                debug_assert!(height as usize <= SIZE);
                debug_assert!(spread != 0);
                debug_assert!(spread.count_ones() <= height as u32);
                debug_assert!(height <= self.stacks[sq.get_file()][sq.get_rank()].get_height());
                debug_assert_eq!(self.get_owner(sq), Some(self.turn));

                let mut x = sq.get_file().0 as usize;
                let mut y = sq.get_rank().0 as usize;
                let stack_height = self.get_height(sq);
                let mut to_move =
                    self.stacks[sq.get_file()][sq.get_rank()].set >> (stack_height - height as i32);

                self.stacks[sq.get_file()][sq.get_rank()].pop_n(height);

                while spread & 1 == 0 {
                    spread = spread >> 1;
                }

                for _ in 0..height {
                    let next_square = spread & 1 == 1;

                    if next_square {
                        match dir {
                            Dir::Up => y += 1,
                            Dir::Down => y -= 1,
                            Dir::Left => x -= 1,
                            Dir::Right => x += 1,
                        }

                        debug_assert!(x < SIZE, "{:?}", mv);
                        debug_assert!(y < SIZE, "{:?}", mv);
                    }

                    let is_player1 = to_move & 1 == 1;
                    let idx = y * SIZE + x;

                    self.stacks[x][y].push(is_player1);
                    self.piece_masks[PieceType::Flat as usize] |= Bitboard::from_index(idx);
                    self.side_masks[!is_player1 as usize] |= Bitboard::from_index(idx);
                    self.side_masks[is_player1 as usize] &= !Bitboard::from_index(idx);

                    to_move = to_move >> 1;
                    spread = spread >> 1;
                }

                let dest = y * SIZE + x;
                let is_standing = self.piece_masks[PieceType::Standing as usize].is_square_set(sq);
                let is_cap = self.piece_masks[PieceType::Cap as usize].is_square_set(sq);

                // Crushing moves have to be made by cap stones
                debug_assert!(!is_crush || is_cap);

                // Crushing moves have to finish on a standing stone
                debug_assert!(
                    !is_crush
                        || self.piece_masks[PieceType::Standing as usize].is_index_set(dest as i32)
                );

                // Clear old mask
                self.piece_masks[PieceType::Standing as usize] &= !Bitboard::from_square(sq);
                self.piece_masks[PieceType::Cap as usize] &= !Bitboard::from_square(sq);

                // Set new mask
                self.piece_masks[PieceType::Flat as usize] ^=
                    Bitboard(((is_standing | is_cap) as u64) << dest);
                self.piece_masks[PieceType::Standing as usize] ^=
                    Bitboard((is_standing as u64) << dest);
                self.piece_masks[PieceType::Cap as usize] ^= Bitboard((is_cap as u64) << dest);

                // Crush?
                self.piece_masks[PieceType::Standing as usize] ^=
                    Bitboard((is_crush as u64) << dest);

                match self.stacks[sq.get_file()][sq.get_rank()].get_owner() {
                    Some(Side::Player1) => {
                        self.side_masks[Side::Player1] |= Bitboard::from_square(sq);
                        self.side_masks[Side::Player2] &= !Bitboard::from_square(sq);
                        self.piece_masks[PieceType::Flat as usize] |= Bitboard::from_square(sq);
                    }
                    Some(Side::Player2) => {
                        self.side_masks[Side::Player1] &= !Bitboard::from_square(sq);
                        self.side_masks[Side::Player2] |= Bitboard::from_square(sq);
                        self.piece_masks[PieceType::Flat as usize] |= Bitboard::from_square(sq);
                    }
                    None => {
                        self.side_masks[Side::Player1] &= !Bitboard::from_square(sq);
                        self.side_masks[Side::Player2] &= !Bitboard::from_square(sq);
                        self.piece_masks[PieceType::Flat as usize] &= !Bitboard::from_square(sq);
                    }
                }
            }
        }

        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;

        debug_assert!(
            self.is_valid().is_ok(),
            "makemove error: {}",
            self.is_valid().unwrap_err()
        );
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        if self.num_pieces[0] < 0 {
            return Err("Invalid piece count");
        }

        if self.num_pieces[1] < 0 {
            return Err("Invalid piece count");
        }

        if self.num_caps[0] < 0 {
            return Err("Invalid cap count");
        }

        if self.num_caps[1] < 0 {
            return Err("Invalid cap count");
        }

        // Mask overlaps
        if (self.side_masks[Side::Player1] & self.side_masks[Side::Player2]).is_occupied() {
            return Err("p1 & p2 mask overlap");
        }
        if (self.piece_masks[PieceType::Flat as usize]
            & self.piece_masks[PieceType::Standing as usize])
            .is_occupied()
        {
            return Err("Flat & Standing mask overlaps");
        }
        if (self.piece_masks[PieceType::Standing as usize]
            & self.piece_masks[PieceType::Cap as usize])
            .is_occupied()
        {
            return Err("Standing & Cap mask overlaps");
        }
        if (self.piece_masks[PieceType::Flat as usize] & self.piece_masks[PieceType::Cap as usize])
            .is_occupied()
        {
            return Err("Flat & Cap mask overlaps");
        }

        for y in 0..SIZE {
            for x in 0..SIZE {
                let idx = (y * SIZE + x) as i32;
                let p1_set = self.side_masks[Side::Player1].is_index_set(idx);
                let p2_set = self.side_masks[Side::Player2].is_index_set(idx);
                let flat_set = self.piece_masks[PieceType::Flat as usize].is_index_set(idx);
                let standing_set = self.piece_masks[PieceType::Standing as usize].is_index_set(idx);
                let cap_set = self.piece_masks[PieceType::Cap as usize].is_index_set(idx);

                match self.stacks[x][y].get_owner() {
                    Some(Side::Player1) => {
                        if !p1_set {
                            return Err("p1 mask not set");
                        }
                        if p2_set {
                            return Err("p2 mask set");
                        }
                        if !flat_set && !standing_set && !cap_set {
                            return Err("no masks set for occupied stack");
                        }
                    }
                    Some(Side::Player2) => {
                        if p1_set {
                            return Err("p1 mask set");
                        }
                        if !p2_set {
                            return Err("p2 mask not set");
                        }
                        if !flat_set && !standing_set && !cap_set {
                            return Err("no masks set for occupied stack");
                        }
                    }
                    None => {
                        if p1_set {
                            return Err("p1 mask set");
                        }
                        if p2_set {
                            return Err("p2 mask set");
                        }
                        if flat_set {
                            return Err("flat mask set");
                        }
                        if standing_set {
                            return Err("standing mask set");
                        }
                        if cap_set {
                            return Err("cap mask set");
                        }
                    }
                }
            }
        }

        for sq in self.piece_masks[PieceType::Standing as usize] {
            if self.get_owner(sq).is_none() {
                return Err("Standing mask & stack mismatch");
            }

            if self.get_height(sq) == 0 {
                return Err("Stack height zero");
            }

            if self.get_piece_on(sq) != Some(PieceType::Standing) {
                return Err("Standing mask not standing stack");
            }
        }

        for sq in self.piece_masks[PieceType::Cap as usize] {
            if self.get_owner(sq).is_none() {
                return Err("Caps mask & stack mismatch");
            }

            if self.get_height(sq) == 0 {
                return Err("Stack height zero");
            }

            if self.get_piece_on(sq) != Some(PieceType::Cap) {
                return Err("Caps mask not caps stack");
            }
        }

        Ok(())
    }

    fn undomove(&mut self, mv: &Self::MoveType) {
        self.turn = !self.turn;
        self.fullmoves -= (self.turn == Side::Player2) as i32;
        let is_flipped = self.stones_played() <= 2;

        match mv {
            Self::MoveType::Drop(sq, PieceType::Flat) => {
                let play_side = if is_flipped { !self.turn } else { self.turn };
                self.stacks[sq.get_file()][sq.get_rank()].pop();
                self.piece_masks[PieceType::Flat as usize] ^= Bitboard::from_square(*sq);
                self.side_masks[play_side] ^= Bitboard::from_square(*sq);
                self.num_pieces[self.turn] += 1;
            }
            Self::MoveType::Drop(sq, PieceType::Standing) => {
                self.stacks[sq.get_file()][sq.get_rank()].pop();
                self.piece_masks[PieceType::Standing as usize] ^= Bitboard::from_square(*sq);
                self.side_masks[self.turn] ^= Bitboard::from_square(*sq);
                self.num_pieces[self.turn] += 1;
            }
            Self::MoveType::Drop(sq, PieceType::Cap) => {
                self.stacks[sq.get_file()][sq.get_rank()].pop();
                self.piece_masks[PieceType::Cap as usize] ^= Bitboard::from_square(*sq);
                self.side_masks[self.turn] ^= Bitboard::from_square(*sq);
                self.num_caps[self.turn] += 1;
            }
            &Self::MoveType::Spread(sq, dir, height, mut spread, is_crush) => {
                debug_assert!(height > 0);
                debug_assert!(height as usize <= SIZE);
                debug_assert!(spread != 0);

                let mut moved = 0;
                let dist = spread.count_ones() as i32;
                let dest = sq.get_index() as i32
                    + match dir {
                        Dir::Up => SIZE as i32 * dist,
                        Dir::Down => SIZE as i32 * -dist,
                        Dir::Left => -dist,
                        Dir::Right => dist,
                    };
                let is_flat = self.piece_masks[PieceType::Flat as usize].is_index_set(dest);
                let is_standing = self.piece_masks[PieceType::Standing as usize].is_index_set(dest);
                let is_cap = self.piece_masks[PieceType::Cap as usize].is_index_set(dest);
                let mut x = sq.get_file().0;
                let mut y = sq.get_rank().0;

                // Clear old mask
                self.piece_masks[PieceType::Flat as usize] &= !Bitboard::from_index(dest as usize);
                self.piece_masks[PieceType::Standing as usize] &=
                    !Bitboard::from_index(dest as usize);
                self.piece_masks[PieceType::Cap as usize] &= !Bitboard::from_index(dest as usize);

                while (spread & 1) == 0 {
                    spread = spread >> 1;
                }
                spread = spread >> 1;

                let mut yee = 0;
                for _ in 0..dist {
                    match dir {
                        Dir::Up => y += 1,
                        Dir::Down => y -= 1,
                        Dir::Left => x -= 1,
                        Dir::Right => x += 1,
                    }

                    let idx = (y as usize * SIZE + x as usize) as usize;
                    let count = if spread != 0 {
                        spread.trailing_zeros() + 1
                    } else {
                        height as u32 - yee as u32
                    };
                    debug_assert!(count > 0);

                    let top = self.stacks[x as usize][y as usize].pop_top(count as usize);

                    match self.stacks[x as usize][y as usize].get_owner() {
                        Some(Side::Player1) => {
                            self.side_masks[Side::Player1] &= !Bitboard::from_index(idx);
                            self.side_masks[Side::Player2] &= !Bitboard::from_index(idx);
                            self.side_masks[Side::Player1] |= Bitboard::from_index(idx);
                            self.piece_masks[PieceType::Flat as usize] |= Bitboard::from_index(idx);
                        }
                        Some(Side::Player2) => {
                            self.side_masks[Side::Player1] &= !Bitboard::from_index(idx);
                            self.side_masks[Side::Player2] &= !Bitboard::from_index(idx);
                            self.side_masks[Side::Player2] |= Bitboard::from_index(idx);
                            self.piece_masks[PieceType::Flat as usize] |= Bitboard::from_index(idx);
                        }
                        None => {
                            self.side_masks[Side::Player1] &= !Bitboard::from_index(idx);
                            self.side_masks[Side::Player2] &= !Bitboard::from_index(idx);
                            self.piece_masks[PieceType::Flat as usize] &=
                                !Bitboard::from_index(idx);
                        }
                    }

                    moved |= top << yee;
                    yee += count as u32;
                    spread = spread >> count;
                }

                self.stacks[sq.get_file()][sq.get_rank()].push_top(moved, height as usize);
                self.piece_masks[PieceType::Flat as usize] &= !Bitboard::from_square(sq);

                // Set new mask
                self.piece_masks[PieceType::Flat as usize] |=
                    Bitboard((is_flat as u64) << sq.get_index());
                self.piece_masks[PieceType::Standing as usize] |=
                    Bitboard((is_standing as u64) << sq.get_index());
                self.piece_masks[PieceType::Cap as usize] |=
                    Bitboard((is_cap as u64) << sq.get_index());

                // Uncrush?
                self.piece_masks[PieceType::Flat as usize] ^= Bitboard((is_crush as u64) << dest);
                self.piece_masks[PieceType::Standing as usize] ^=
                    Bitboard((is_crush as u64) << dest);

                match self.stacks[sq.get_file()][sq.get_rank()].get_owner() {
                    Some(Side::Player1) => {
                        self.side_masks[Side::Player1] &= !Bitboard::from_square(sq);
                        self.side_masks[Side::Player2] &= !Bitboard::from_square(sq);
                        self.side_masks[Side::Player1] |= Bitboard::from_square(sq);
                    }
                    Some(Side::Player2) => {
                        self.side_masks[Side::Player1] &= !Bitboard::from_square(sq);
                        self.side_masks[Side::Player2] &= !Bitboard::from_square(sq);
                        self.side_masks[Side::Player2] |= Bitboard::from_square(sq);
                    }
                    None => {
                        self.side_masks[Side::Player1] &= !Bitboard::from_square(sq);
                        self.side_masks[Side::Player2] &= !Bitboard::from_square(sq);
                    }
                }
            }
        }

        debug_assert!(
            self.is_valid().is_ok(),
            "undomove error: {}",
            self.is_valid().unwrap_err()
        );
    }

    fn makenull(&mut self) {
        todo!()
    }

    fn undonull(&mut self) {
        todo!()
    }

    fn get_result(&self) -> Option<GameResult> {
        // Roads
        let has_road_p1 = {
            let us = (self.piece_masks[PieceType::Flat as usize]
                | self.piece_masks[PieceType::Cap as usize])
                & self.side_masks[Side::Player1];
            let flooded_horizontal = floodfill(us & Bitboard::get_left_edge(), us);
            let flooded_vertical = floodfill(us & Bitboard::get_top_edge(), us);
            (flooded_horizontal & Bitboard::get_right_edge()).is_occupied()
                | (flooded_vertical & Bitboard::get_bottom_edge()).is_occupied()
        };
        let has_road_p2 = {
            let us = (self.piece_masks[PieceType::Flat as usize]
                | self.piece_masks[PieceType::Cap as usize])
                & self.side_masks[Side::Player2];
            let flooded_horizontal = floodfill(us & Bitboard::get_left_edge(), us);
            let flooded_vertical = floodfill(us & Bitboard::get_top_edge(), us);
            (flooded_horizontal & Bitboard::get_right_edge()).is_occupied()
                | (flooded_vertical & Bitboard::get_bottom_edge()).is_occupied()
        };
        if has_road_p1 && has_road_p2 {
            return Some(GameResult::Win(!self.turn));
        }
        if has_road_p1 && !has_road_p2 {
            return Some(GameResult::Win(Side::Player1));
        }
        if has_road_p2 && !has_road_p1 {
            return Some(GameResult::Win(Side::Player2));
        }

        let occupied = self.side_masks[Side::Player1] | self.side_masks[Side::Player2];
        let is_filled = occupied.count() == (SIZE * SIZE) as i32;
        let out_of_pieces = self.num_pieces[0] + self.num_caps[0] == 0
            || self.num_pieces[1] + self.num_caps[1] == 0;

        // Filled or no pieces/caps left
        if is_filled || out_of_pieces {
            let p1_flats =
                self.side_masks[Side::Player1] & self.piece_masks[PieceType::Flat as usize];
            let p2_flats =
                self.side_masks[Side::Player2] & self.piece_masks[PieceType::Flat as usize];

            if p1_flats.count() > p2_flats.count() {
                return Some(GameResult::Win(Side::Player1));
            } else if p2_flats.count() > p1_flats.count() {
                return Some(GameResult::Win(Side::Player2));
            } else {
                return Some(GameResult::Draw);
            }
        }

        None
    }

    fn get_fen(&self) -> String {
        format!(
            "{} {} {}",
            self.get_board_fen(),
            match self.turn {
                Side::Player1 => "1",
                Side::Player2 => "2",
            },
            self.fullmoves,
        )
    }

    fn parse_fen_part(&mut self, _idx: usize, _part: &str) {
        panic!("Not required");
    }
}

impl<const SIZE: usize> std::fmt::Display for TakPosition<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..SIZE).rev() {
            for x in 0..SIZE {
                let sq = Square::<SIZE, SIZE>::from_coords(x as i32, y as i32);
                match (self.get_piece_on(sq), self.get_owner(sq)) {
                    (Some(PieceType::Flat), Some(Side::Player1)) => write!(f, "F")?,
                    (Some(PieceType::Flat), Some(Side::Player2)) => write!(f, "f")?,
                    (Some(PieceType::Standing), Some(Side::Player1)) => write!(f, "S")?,
                    (Some(PieceType::Standing), Some(Side::Player2)) => write!(f, "s")?,
                    (Some(PieceType::Cap), Some(Side::Player1)) => write!(f, "C")?,
                    (Some(PieceType::Cap), Some(Side::Player2)) => write!(f, "c")?,
                    (_, _) => write!(f, ".")?,
                }
            }
            writeln!(f, "")?;
        }

        match self.turn {
            Side::Player1 => writeln!(f, "Turn: w")?,
            Side::Player2 => writeln!(f, "Turn: b")?,
        }

        writeln!(f, "Fullmoves: {}", self.fullmoves)?;
        writeln!(f, "Size: {}x{}", SIZE, SIZE)?;
        writeln!(
            f,
            "Pieces: {}, {}",
            self.num_pieces[Side::Player1],
            self.num_pieces[Side::Player2]
        )?;
        writeln!(
            f,
            "Caps: {}, {}",
            self.num_caps[Side::Player1],
            self.num_caps[Side::Player2]
        )?;
        writeln!(f, "Valid: {:?}", self.is_valid())?;
        writeln!(f, "Result: {:?}", self.get_result())?;

        Ok(())
    }
}
