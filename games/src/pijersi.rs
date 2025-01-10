use crate::{
    gamerules::{GameResult, GameRules},
    general::{hex::Hex, hexbitboard::HexBitboard, side::Side},
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[must_use]
pub enum Piece {
    Rock,
    Paper,
    Scissors,
    Wise,
}

#[derive(Clone, Copy, PartialEq)]
#[must_use]
pub enum Layer {
    Lower,
    Upper,
}

#[derive(Clone, Copy)]
struct Irrecoverable {
    pub sides: [HexBitboard<6, 7>; 2],
    pub pieces: [[HexBitboard<6, 7>; 2]; 4],
    pub halfmoves: i32,
}

#[derive(Default, Clone)]
pub struct PijersiPosition {
    stack: Vec<Irrecoverable>,
    pub sides: [HexBitboard<6, 7>; 2],
    pub pieces: [[HexBitboard<6, 7>; 2]; 4],
    pub turn: Side,
    pub halfmoves: i32,
    pub fullmoves: i32,
}

#[derive(Clone, Copy)]
pub enum PijersiMove {
    SoloMove(Hex<6, 7>, Hex<6, 7>),
    SoloStack(Hex<6, 7>, Hex<6, 7>),
    SoloStackMove(Hex<6, 7>, Hex<6, 7>, Hex<6, 7>),
    StackMove(Hex<6, 7>, Hex<6, 7>),
    StackMoveDestack(Hex<6, 7>, Hex<6, 7>, Hex<6, 7>),
    StackDestack(Hex<6, 7>, Hex<6, 7>),
    StackStack(Hex<6, 7>, Hex<6, 7>),
    StackStackMove(Hex<6, 7>, Hex<6, 7>, Hex<6, 7>),
}

impl PijersiPosition {
    #[must_use]
    pub fn get_white(&self) -> HexBitboard<6, 7> {
        self.sides[Side::Player1]
    }

    #[must_use]
    pub fn get_black(&self) -> HexBitboard<6, 7> {
        self.sides[Side::Player2]
    }

    #[must_use]
    pub fn get_us(&self) -> HexBitboard<6, 7> {
        self.sides[self.turn]
    }

    #[must_use]
    pub fn get_them(&self) -> HexBitboard<6, 7> {
        self.sides[!self.turn]
    }

    #[must_use]
    pub fn get_empty(&self) -> HexBitboard<6, 7> {
        !(self.get_white() | self.get_black())
    }

    #[must_use]
    pub fn get_rock(&self) -> HexBitboard<6, 7> {
        self.pieces[Piece::Rock as usize][Layer::Lower as usize]
            | self.pieces[Piece::Rock as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_paper(&self) -> HexBitboard<6, 7> {
        self.pieces[Piece::Paper as usize][Layer::Lower as usize]
            | self.pieces[Piece::Paper as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_scissors(&self) -> HexBitboard<6, 7> {
        self.pieces[Piece::Scissors as usize][Layer::Lower as usize]
            | self.pieces[Piece::Scissors as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_rps(&self) -> HexBitboard<6, 7> {
        self.get_rock() | self.get_paper() | self.get_scissors()
    }

    #[must_use]
    pub fn get_wise(&self) -> HexBitboard<6, 7> {
        self.pieces[Piece::Wise as usize][Layer::Lower as usize]
            | self.pieces[Piece::Wise as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_lower(&self) -> HexBitboard<6, 7> {
        self.pieces[Piece::Rock as usize][Layer::Lower as usize]
            | self.pieces[Piece::Paper as usize][Layer::Lower as usize]
            | self.pieces[Piece::Scissors as usize][Layer::Lower as usize]
            | self.pieces[Piece::Wise as usize][Layer::Lower as usize]
    }

    #[must_use]
    pub fn get_upper(&self) -> HexBitboard<6, 7> {
        self.pieces[Piece::Rock as usize][Layer::Upper as usize]
            | self.pieces[Piece::Paper as usize][Layer::Upper as usize]
            | self.pieces[Piece::Scissors as usize][Layer::Upper as usize]
            | self.pieces[Piece::Wise as usize][Layer::Upper as usize]
    }

    #[must_use]
    pub fn get_short(&self) -> HexBitboard<6, 7> {
        self.get_lower() ^ self.get_upper()
    }

    #[must_use]
    pub fn get_tall(&self) -> HexBitboard<6, 7> {
        self.get_upper()
    }

    #[must_use]
    pub fn get_visible(&self, piece: Piece) -> HexBitboard<6, 7> {
        self.pieces[piece as usize][Layer::Upper as usize]
            | (self.pieces[piece as usize][Layer::Lower as usize] & !self.get_upper())
    }

    #[must_use]
    pub fn get_piece_on(&self, layer: Layer, hex: Hex<6, 7>) -> Option<Piece> {
        if self.pieces[Piece::Rock as usize][layer as usize].is_set(&hex) {
            Some(Piece::Rock)
        } else if self.pieces[Piece::Paper as usize][layer as usize].is_set(&hex) {
            Some(Piece::Paper)
        } else if self.pieces[Piece::Scissors as usize][layer as usize].is_set(&hex) {
            Some(Piece::Scissors)
        } else if self.pieces[Piece::Wise as usize][layer as usize].is_set(&hex) {
            Some(Piece::Wise)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_side_on(&self, hex: Hex<6, 7>) -> Option<Side> {
        if self.sides[Side::Player1].is_set(&hex) {
            Some(Side::Player1)
        } else if self.sides[Side::Player2].is_set(&hex) {
            Some(Side::Player2)
        } else {
            None
        }
    }
}

fn get_moves(
    us: HexBitboard<6, 7>,
    them: HexBitboard<6, 7>,
    short: HexBitboard<6, 7>,
    tall: HexBitboard<6, 7>,
    piece_mask: HexBitboard<6, 7>,
    stackable: HexBitboard<6, 7>,
    capturable: HexBitboard<6, 7>,
    func: &mut impl FnMut(PijersiMove) -> bool,
) -> bool {
    let empty = !(us | them);

    // SoloMove
    for solo in piece_mask & short {
        for mv in HexBitboard::<6, 7>::from_hex(&solo).adjacent() & (empty | capturable) {
            if func(PijersiMove::SoloMove(solo, mv)) {
                return true;
            }
        }
    }

    // SoloStack
    for solo in piece_mask & short {
        for stack in HexBitboard::from_hex(&solo).adjacent() & us & short & stackable {
            if func(PijersiMove::SoloStack(solo, stack)) {
                return true;
            }
        }
    }

    // SoloStackMove
    for solo in piece_mask & short {
        for stack in HexBitboard::from_hex(&solo).adjacent() & us & short & stackable {
            let bb = HexBitboard::from_hex(&stack);

            let dist1 = bb.adjacent() & (empty | capturable | HexBitboard::from_hex(&solo));
            for mv in dist1 {
                if func(PijersiMove::SoloStackMove(solo, stack, mv)) {
                    return true;
                }
            }

            let blockers = (us | them) ^ HexBitboard::from_hex(&solo);
            let dist2 = bb.doubles(blockers) & (empty | capturable);
            for mv in dist2 {
                if func(PijersiMove::SoloStackMove(solo, stack, mv)) {
                    return true;
                }
            }
        }
    }

    // StackMove
    for stack in piece_mask & tall {
        let bb = HexBitboard::from_hex(&stack);

        let dist1 = bb.adjacent() & (empty | capturable);
        for mv in dist1 {
            if func(PijersiMove::StackMove(stack, mv)) {
                return true;
            }
        }

        let dist2 = bb.doubles(us | them) & (empty | capturable);
        for mv in dist2 {
            if func(PijersiMove::StackMove(stack, mv)) {
                return true;
            }
        }
    }

    // StackMoveDestack
    for stack in piece_mask & tall {
        let bb = HexBitboard::from_hex(&stack);

        let dist1 = bb.adjacent() & (empty | capturable);
        for mv in dist1 {
            for destack in
                HexBitboard::from_hex(&mv).adjacent() & (empty | stackable | capturable | bb)
            {
                if func(PijersiMove::StackMoveDestack(stack, mv, destack)) {
                    return true;
                }
            }
        }

        let dist2 = bb.doubles(us | them) & (empty | capturable);
        for mv in dist2 {
            for destack in HexBitboard::from_hex(&mv).adjacent() & (empty | stackable | capturable)
            {
                if func(PijersiMove::StackMoveDestack(stack, mv, destack)) {
                    return true;
                }
            }
        }
    }

    // StackDestack
    for stack in piece_mask & tall {
        for destack in HexBitboard::from_hex(&stack).adjacent() & (empty | capturable) {
            if func(PijersiMove::StackDestack(stack, destack)) {
                return true;
            }
        }
    }

    // StackStack
    for stack1 in piece_mask & tall {
        for stack2 in HexBitboard::from_hex(&stack1).adjacent() & us & short & stackable {
            if func(PijersiMove::StackStack(stack1, stack2)) {
                return true;
            }
        }
    }

    // StackStackMove
    for stack1 in piece_mask & tall {
        for stack2 in HexBitboard::from_hex(&stack1).adjacent() & us & short & stackable {
            let bb = HexBitboard::from_hex(&stack2);

            let dist1 = bb.adjacent() & (empty | capturable);
            for mv in dist1 {
                if func(PijersiMove::StackStackMove(stack1, stack2, mv)) {
                    return true;
                }
            }

            let dist2 = bb.doubles(us | them) & (empty | capturable);
            for mv in dist2 {
                if func(PijersiMove::StackStackMove(stack1, stack2, mv)) {
                    return true;
                }
            }
        }
    }

    false
}

impl GameRules for PijersiPosition {
    type MoveType = PijersiMove;
    const WIDTH: i32 = 6;
    const HEIGHT: i32 = 7;

    fn startpos() -> Self {
        Self {
            stack: vec![],
            sides: [HexBitboard(0x1fff), HexBitboard(0x1fff00000000)],
            pieces: [
                [HexBitboard(0x121200000909), HexBitboard(0x0)],
                [HexBitboard(0x94100001052), HexBitboard(0x0)],
                [HexBitboard(0x4a4000004a4), HexBitboard(0x0)],
                [HexBitboard(0x800000200), HexBitboard(0x800000200)],
            ],
            turn: Side::Player1,
            halfmoves: 0,
            fullmoves: 1,
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, yee: usize) -> bool {
        let hex = Hex::from_coords(x, y);
        let bb = HexBitboard::from_hex(&hex);
        let layer = if yee == 0 { Layer::Lower } else { Layer::Upper };
        match c {
            'R' => {
                self.sides[Side::Player1] |= bb;
                self.pieces[Piece::Rock as usize][layer as usize] ^= bb;
            }
            'P' => {
                self.sides[Side::Player1] |= bb;
                self.pieces[Piece::Paper as usize][layer as usize] ^= bb;
            }
            'S' => {
                self.sides[Side::Player1] |= bb;
                self.pieces[Piece::Scissors as usize][layer as usize] ^= bb;
            }
            'W' => {
                self.sides[Side::Player1] |= bb;
                self.pieces[Piece::Wise as usize][layer as usize] ^= bb;
            }
            'r' => {
                self.sides[Side::Player2] |= bb;
                self.pieces[Piece::Rock as usize][layer as usize] ^= bb;
            }
            'p' => {
                self.sides[Side::Player2] |= bb;
                self.pieces[Piece::Paper as usize][layer as usize] ^= bb;
            }
            's' => {
                self.sides[Side::Player2] |= bb;
                self.pieces[Piece::Scissors as usize][layer as usize] ^= bb;
            }
            'w' => {
                self.sides[Side::Player2] |= bb;
                self.pieces[Piece::Wise as usize][layer as usize] ^= bb;
            }
            _ => {}
        }
        yee >= 1
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let hex = Hex::from_coords(x, y);
        if self.get_empty().is_set(&hex) {
            return None;
        }

        let mut out = String::new();
        if self.get_white().is_set(&hex) {
            out += match self.get_piece_on(Layer::Lower, hex) {
                Some(Piece::Rock) => "R",
                Some(Piece::Paper) => "P",
                Some(Piece::Scissors) => "S",
                Some(Piece::Wise) => "W",
                None => "?",
            };
            out += match self.get_piece_on(Layer::Upper, hex) {
                Some(Piece::Rock) => "R",
                Some(Piece::Paper) => "P",
                Some(Piece::Scissors) => "S",
                Some(Piece::Wise) => "W",
                None => "-",
            };
        } else if self.get_black().is_set(&hex) {
            out += match self.get_piece_on(Layer::Lower, hex) {
                Some(Piece::Rock) => "r",
                Some(Piece::Paper) => "p",
                Some(Piece::Scissors) => "s",
                Some(Piece::Wise) => "w",
                None => "?",
            };
            out += match self.get_piece_on(Layer::Upper, hex) {
                Some(Piece::Rock) => "r",
                Some(Piece::Paper) => "p",
                Some(Piece::Scissors) => "s",
                Some(Piece::Wise) => "w",
                None => "-",
            };
        }

        debug_assert!(!out.is_empty());
        debug_assert!(out.len() == 2);

        Some(out)
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        if self.is_gameover() {
            return;
        }

        // Rock > Scissors
        if get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Rock) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Scissors) & self.get_them(),
            &mut func,
        ) {
            return;
        }

        // Paper > Rock
        if get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Paper) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Rock) & self.get_them(),
            &mut func,
        ) {
            return;
        }

        // Scissors > Paper
        if get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Scissors) & self.get_us(),
            self.get_us() & self.get_short(),
            self.get_visible(Piece::Paper) & self.get_them(),
            &mut func,
        ) {
            return;
        }

        // Wise > All
        get_moves(
            self.get_us(),
            self.get_them(),
            self.get_short(),
            self.get_tall(),
            self.get_visible(Piece::Wise) & self.get_us(),
            self.get_us() & self.get_short() & self.get_wise(),
            HexBitboard::empty(),
            &mut func,
        );
    }

    fn get_turn(&self) -> Side {
        self.turn
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        self.stack.push(Irrecoverable {
            sides: self.sides,
            pieces: self.pieces,
            halfmoves: self.halfmoves,
        });

        match mv {
            PijersiMove::SoloMove(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());

                let piece = self.get_piece_on(Layer::Lower, *fr).unwrap();

                // Halfmove counter
                if self.get_them().is_set(to) {
                    self.halfmoves = 0;
                } else {
                    self.halfmoves += 1;
                }

                // Remove captured
                self.sides[!self.turn as usize] &= !HexBitboard::from_hex(to);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);

                // Remove piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(fr);
                self.pieces[piece as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(fr);

                // Add piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(to);
                self.pieces[piece as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(to);

                debug_assert!(self.get_side_on(*fr).is_none());
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());
            }
            PijersiMove::SoloStack(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());

                let piece = self.get_piece_on(Layer::Lower, *fr).unwrap();

                // Halfmove counter
                self.halfmoves += 1;

                // Remove piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(fr);
                self.pieces[piece as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(fr);

                // Add piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(to);

                debug_assert!(self.get_side_on(*fr).is_none());
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_some());
            }
            PijersiMove::SoloStackMove(fr, hex1, hex2) => {
                debug_assert_ne!(fr, hex1);
                debug_assert_ne!(hex1, hex2);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*hex1).unwrap() == self.turn);
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *hex1).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *hex1).is_none());

                let piece1 = self.get_piece_on(Layer::Lower, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Lower, *hex1).unwrap();

                // Halfmove counter
                if self.get_them().is_set(hex2) {
                    self.halfmoves = 0;
                } else {
                    self.halfmoves += 1;
                }

                // Remove piece1
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(fr);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(fr);

                // Remove piece2
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(hex1);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(hex1);

                // Remove captured
                self.sides[!self.turn as usize] &= !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);

                // Add pieces
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(hex2);
                self.pieces[piece1 as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(hex2);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(hex2);

                debug_assert!(self.get_side_on(*hex1).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *hex1).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *hex1).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *hex2).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *hex2).is_some());
            }
            PijersiMove::StackMove(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());

                let piece1 = self.get_piece_on(Layer::Lower, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Upper, *fr).unwrap();

                // Halfmove counter
                if self.get_them().is_set(to) {
                    self.halfmoves = 0;
                } else {
                    self.halfmoves += 1;
                }

                // Remove captured
                self.sides[!self.turn as usize] &= !HexBitboard::from_hex(to);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);

                // Remove piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(fr);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(fr);
                self.pieces[piece2 as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(fr);

                // Add piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(to);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(to);
                self.pieces[piece2 as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(to);

                debug_assert!(self.get_side_on(*fr).is_none());
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_some());
            }
            PijersiMove::StackMoveDestack(fr, hex1, hex2) => {
                debug_assert_ne!(fr, hex1);
                debug_assert_ne!(hex1, hex2);
                debug_assert!(self.get_side_on(*fr).unwrap() == self.turn);
                debug_assert!(self.get_side_on(*hex1) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());

                let piece1 = self.get_piece_on(Layer::Lower, *fr).unwrap();
                let piece2: Piece = self.get_piece_on(Layer::Upper, *fr).unwrap();
                let destack_layer = if self.get_us().is_set(hex2) && fr != hex2 {
                    Layer::Upper
                } else {
                    Layer::Lower
                };

                let capture_2 = HexBitboard::from_hex(hex2) & self.get_them();

                // Halfmove counter
                if self.get_them().is_set(hex1) || self.get_them().is_set(hex2) {
                    self.halfmoves = 0;
                } else {
                    self.halfmoves += 1;
                }

                // Remove captured
                self.sides[!self.turn as usize] &= !HexBitboard::from_hex(hex1);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex1);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex1);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex1);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex1);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex1);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex1);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex1);

                // Remove captured
                self.sides[!self.turn as usize] &= !capture_2;
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &= !capture_2;
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &= !capture_2;
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &= !capture_2;
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &= !capture_2;
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &= !capture_2;
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &= !capture_2;
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &= !capture_2;

                // Remove pieces
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(fr);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(fr);
                self.pieces[piece2 as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(fr);

                // Add piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(hex1);
                self.pieces[piece1 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(hex1);

                // Add piece
                self.sides[self.turn as usize] |= HexBitboard::from_hex(hex2);
                self.pieces[piece2 as usize][destack_layer as usize] ^= HexBitboard::from_hex(hex2);

                debug_assert!(self.get_side_on(*hex1) == Some(self.turn));
                debug_assert!(self.get_side_on(*hex2) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *hex1).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *hex1).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *hex2).is_some());
            }
            PijersiMove::StackDestack(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) != Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());

                let piece = self.get_piece_on(Layer::Upper, *fr).unwrap();

                // Halfmove counter
                if self.get_them().is_set(to) {
                    self.halfmoves = 0;
                } else {
                    self.halfmoves += 1;
                }

                // Remove captured
                self.sides[!self.turn as usize] &= !HexBitboard::from_hex(to);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(to);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(to);

                // Remove piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(fr);

                // Add piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(to);
                self.pieces[piece as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(to);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());
            }
            PijersiMove::StackStack(fr, to) => {
                debug_assert_ne!(fr, to);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_none());

                let piece = self.get_piece_on(Layer::Upper, *fr).unwrap();

                // Halfmove counter
                if self.get_them().is_set(to) {
                    self.halfmoves = 0;
                } else {
                    self.halfmoves += 1;
                }

                // Remove piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(fr);

                // Add piece
                self.pieces[piece as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(to);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*to) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *to).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *to).is_some());
            }
            PijersiMove::StackStackMove(fr, hex1, hex2) => {
                debug_assert_ne!(fr, hex1);
                debug_assert_ne!(hex1, hex2);
                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*hex1) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Lower, *hex1).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *hex1).is_none());

                let piece1 = self.get_piece_on(Layer::Upper, *fr).unwrap();
                let piece2 = self.get_piece_on(Layer::Lower, *hex1).unwrap();

                // Halfmove counter
                if self.get_them().is_set(hex2) {
                    self.halfmoves = 0;
                } else {
                    self.halfmoves += 1;
                }

                // Remove captured
                self.sides[!self.turn as usize] &= !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Rock as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Rock as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Paper as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Paper as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Scissors as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Scissors as usize][Layer::Upper as usize] &=
                    !HexBitboard::from_hex(hex2);
                self.pieces[Piece::Wise as usize][Layer::Lower as usize] &=
                    !HexBitboard::from_hex(hex2);

                // Remove piece
                self.pieces[piece1 as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(fr);

                // Remove piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(hex1);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(hex1);

                // Add piece
                self.sides[self.turn as usize] ^= HexBitboard::from_hex(hex2);
                self.pieces[piece1 as usize][Layer::Upper as usize] ^= HexBitboard::from_hex(hex2);
                self.pieces[piece2 as usize][Layer::Lower as usize] ^= HexBitboard::from_hex(hex2);

                debug_assert!(self.get_side_on(*fr) == Some(self.turn));
                debug_assert!(self.get_side_on(*hex1).is_none());
                debug_assert!(self.get_side_on(*hex2) == Some(self.turn));
                debug_assert!(self.get_piece_on(Layer::Lower, *fr).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *fr).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *hex1).is_none());
                debug_assert!(self.get_piece_on(Layer::Upper, *hex1).is_none());
                debug_assert!(self.get_piece_on(Layer::Lower, *hex2).is_some());
                debug_assert!(self.get_piece_on(Layer::Upper, *hex2).is_some());
            }
        }

        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;

        debug_assert!(self.is_valid().is_ok());
    }

    fn undomove(&mut self, _mv: &Self::MoveType) {
        let data = self
            .stack
            .pop()
            .expect("Can't undo a move that was never made");
        self.sides = data.sides;
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

    fn get_result(&self) -> Option<GameResult> {
        let white_rps = self.get_white() & self.get_rps();
        let black_rps = self.get_black() & self.get_rps();

        if (white_rps & HexBitboard::get_top_edge()).is_occupied() {
            Some(GameResult::Win(Side::Player1))
        } else if (black_rps & HexBitboard::get_bottom_edge()).is_occupied() {
            Some(GameResult::Win(Side::Player2))
        } else if self.get_white().is_empty() {
            Some(GameResult::Win(Side::Player2))
        } else if self.get_black().is_empty() {
            Some(GameResult::Win(Side::Player1))
        } else if self.halfmoves >= 20 {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    fn get_fen(&self) -> String {
        format!(
            "{} {} {} {}",
            self.get_board_fen(),
            match self.turn {
                Side::Player1 => "w",
                Side::Player2 => "b",
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
                "w" => self.turn = Side::Player1,
                "b" => self.turn = Side::Player2,
                _ => panic!("Uh oh"),
            },
            // Halfmoves
            2 => self.halfmoves = part.parse::<i32>().unwrap(),
            // Fullmoves
            3 => self.fullmoves = part.parse::<i32>().unwrap(),
            _ => panic!("Invalid fen part index"),
        }
    }
}

impl std::fmt::Display for PijersiPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..7).rev() {
            let is_long = y % 2 == 1;

            if !is_long {
                write!(f, "  ")?;
            }

            for x in 0..6 + is_long as i32 {
                write!(f, "  ")?;

                let hex = Hex::from_coords(x, y);
                let bottom = self.get_piece_on(Layer::Lower, hex);
                let top = self.get_piece_on(Layer::Upper, hex);
                let side = self.get_side_on(hex);

                match (bottom, side) {
                    (Some(Piece::Rock), Some(Side::Player1)) => write!(f, "R")?,
                    (Some(Piece::Rock), Some(Side::Player2)) => write!(f, "r")?,
                    (Some(Piece::Paper), Some(Side::Player1)) => write!(f, "P")?,
                    (Some(Piece::Paper), Some(Side::Player2)) => write!(f, "p")?,
                    (Some(Piece::Scissors), Some(Side::Player1)) => write!(f, "S")?,
                    (Some(Piece::Scissors), Some(Side::Player2)) => write!(f, "s")?,
                    (Some(Piece::Wise), Some(Side::Player1)) => write!(f, "W")?,
                    (Some(Piece::Wise), Some(Side::Player2)) => write!(f, "w")?,
                    (None, _) => write!(f, ".")?,
                    (_, _) => write!(f, "?")?,
                }

                match (top, side) {
                    (Some(Piece::Rock), Some(Side::Player1)) => write!(f, "R")?,
                    (Some(Piece::Rock), Some(Side::Player2)) => write!(f, "r")?,
                    (Some(Piece::Paper), Some(Side::Player1)) => write!(f, "P")?,
                    (Some(Piece::Paper), Some(Side::Player2)) => write!(f, "p")?,
                    (Some(Piece::Scissors), Some(Side::Player1)) => write!(f, "S")?,
                    (Some(Piece::Scissors), Some(Side::Player2)) => write!(f, "s")?,
                    (Some(Piece::Wise), Some(Side::Player1)) => write!(f, "W")?,
                    (Some(Piece::Wise), Some(Side::Player2)) => write!(f, "w")?,
                    (None, _) => write!(f, ".")?,
                    (_, _) => write!(f, "?")?,
                }
            }

            writeln!(f)?;
        }

        match self.turn {
            Side::Player1 => writeln!(f, "Turn: w")?,
            Side::Player2 => writeln!(f, "Turn: b")?,
        }

        writeln!(f, "Halfmoves: {}", self.halfmoves)?;
        writeln!(f, "Fullmoves: {}", self.fullmoves)?;

        Ok(())
    }
}
