use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, rank::Rank, side::Side, square::Square},
};
use std::fmt;

#[derive(Clone, Copy)]
struct Irrecoverable {
    pub captured: Option<ChessPiece>,
    pub halfmoves: i32,
    pub ep: Option<Square<8, 8>>,
    pub us_ksc: bool,
    pub us_qsc: bool,
    pub them_ksc: bool,
    pub them_qsc: bool,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Board {
    colour: [Bitboard<8, 8>; 2],
    pieces: [Bitboard<8, 8>; 6],
    ep: Option<Square<8, 8>>,
    pub flipped: bool,
}

#[derive(Default, Clone)]
pub struct ChessPosition {
    pub board: Board,
    pub halfmoves: i32,
    pub fullmoves: i32,
    pub us_ksc: bool,
    pub us_qsc: bool,
    pub them_ksc: bool,
    pub them_qsc: bool,
    stack: Vec<Irrecoverable>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChessMove {
    pub from: Square<8, 8>,
    pub to: Square<8, 8>,
    pub promo: Option<ChessPiece>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Castling {
    WKS,
    WQS,
    BKS,
    BQS,
}

impl ChessMove {
    #[must_use]
    pub fn from_string(word: &str) -> Result<Self, &'static str> {
        if word.len() < 4 || word.len() > 5 {
            Err("movestr wrong length")
        } else {
            let fr = Square::<8, 8>::from_string(&word[0..2]);
            let to = Square::<8, 8>::from_string(&word[2..4]);

            if word.len() == 4 {
                Ok(ChessMove {
                    from: fr.unwrap(),
                    to: to.unwrap(),
                    promo: None,
                })
            } else {
                let promo = match word.chars().nth(4) {
                    Some('q') => Some(ChessPiece::Queen),
                    Some('r') => Some(ChessPiece::Rook),
                    Some('b') => Some(ChessPiece::Bishop),
                    Some('n') => Some(ChessPiece::Knight),
                    Some(_) => return Err("invalid promotion character"),
                    _ => return Err("missing promotion character"),
                };
                Ok(ChessMove {
                    from: fr.unwrap(),
                    to: to.unwrap(),
                    promo,
                })
            }
        }
    }

    #[must_use]
    pub fn flip(&self) -> Self {
        Self {
            from: self.from.flipped(),
            to: self.to.flipped(),
            promo: self.promo,
        }
    }
}

impl Board {
    #[must_use]
    pub fn get_us(&self) -> Bitboard<8, 8> {
        self.colour[0]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard<8, 8> {
        self.colour[1]
    }

    #[must_use]
    pub fn get_occupied(&self) -> Bitboard<8, 8> {
        self.colour[0] | self.colour[1]
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard<8, 8> {
        !self.get_occupied()
    }

    #[must_use]
    pub fn get_pawns(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Pawn as usize]
    }

    #[must_use]
    pub fn get_knights(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Knight as usize]
    }

    #[must_use]
    pub fn get_bishops(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Bishop as usize]
    }

    #[must_use]
    pub fn get_rooks(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Rook as usize]
    }

    #[must_use]
    pub fn get_queens(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Queen as usize]
    }

    #[must_use]
    pub fn get_kings(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::King as usize]
    }

    #[must_use]
    pub fn get_bq(&self) -> Bitboard<8, 8> {
        self.get_bishops() | self.get_queens()
    }

    #[must_use]
    pub fn get_rq(&self) -> Bitboard<8, 8> {
        self.get_rooks() | self.get_queens()
    }

    #[must_use]
    pub fn piece_on(&self, x: i32, y: i32) -> Option<ChessPiece> {
        let sq = Square::<8, 8>::from_coords(x, y);
        self.piece_on_sq(sq)
    }

    #[must_use]
    pub fn piece_on_sq(&self, sq: Square<8, 8>) -> Option<ChessPiece> {
        if self.pieces[ChessPiece::Pawn as usize].is_square_set(sq) {
            Some(ChessPiece::Pawn)
        } else if self.pieces[ChessPiece::Knight as usize].is_square_set(sq) {
            Some(ChessPiece::Knight)
        } else if self.pieces[ChessPiece::Bishop as usize].is_square_set(sq) {
            Some(ChessPiece::Bishop)
        } else if self.pieces[ChessPiece::Rook as usize].is_square_set(sq) {
            Some(ChessPiece::Rook)
        } else if self.pieces[ChessPiece::Queen as usize].is_square_set(sq) {
            Some(ChessPiece::Queen)
        } else if self.pieces[ChessPiece::King as usize].is_square_set(sq) {
            Some(ChessPiece::King)
        } else {
            None
        }
    }

    #[must_use]
    pub fn side_on(&self, x: i32, y: i32) -> Option<Side> {
        let sq = Square::<8, 8>::from_coords(x, y);
        if self.colour[Side::Player1].is_square_set(sq) {
            Some(Side::Player1)
        } else if self.colour[Side::Player2].is_square_set(sq) {
            Some(Side::Player2)
        } else {
            None
        }
    }

    pub fn toggle(&mut self, sq: Square<8, 8>, piece: ChessPiece, side: Side) {
        self.colour[side] ^= Bitboard::from_square(sq);
        self.pieces[piece as usize] ^= Bitboard::from_square(sq);
    }

    pub fn set(&mut self, sq: Square<8, 8>, piece: ChessPiece, side: Side) {
        self.colour[side] |= Bitboard::from_square(sq);
        self.pieces[piece as usize] |= Bitboard::from_square(sq);
    }

    pub fn remove(&mut self, sq: Square<8, 8>, piece: ChessPiece, side: Side) {
        self.colour[side] &= !Bitboard::from_square(sq);
        self.pieces[piece as usize] &= !Bitboard::from_square(sq);
    }

    #[must_use]
    fn is_empty(&self, sq: Square<8, 8>) -> bool {
        !self.get_occupied().is_square_set(sq)
    }

    #[must_use]
    fn is_attacked(&self, sq: Square<8, 8>) -> bool {
        let bb = Bitboard::from_square(sq);

        let pawn_attacks = bb.ne() | bb.nw();
        let knight_attacks = bb.north().north().east()
            | bb.north().north().west()
            | bb.south().south().east()
            | bb.south().south().west()
            | bb.east().east().north()
            | bb.east().east().south()
            | bb.west().west().north()
            | bb.west().west().south();
        let bishop_attacks = bb.ray_ne(self.get_occupied())
            | bb.ray_nw(self.get_occupied())
            | bb.ray_se(self.get_occupied())
            | bb.ray_sw(self.get_occupied());
        let rook_attacks = bb.ray_north(self.get_occupied())
            | bb.ray_south(self.get_occupied())
            | bb.ray_east(self.get_occupied())
            | bb.ray_west(self.get_occupied());
        let king_attacks = bb.adjacent();

        (pawn_attacks & self.get_pawns() & self.get_them()).is_occupied()
            | (knight_attacks & self.get_knights() & self.get_them()).is_occupied()
            | (bishop_attacks & (self.get_bishops() | self.get_queens()) & self.get_them())
                .is_occupied()
            | (rook_attacks & (self.get_rooks() | self.get_queens()) & self.get_them())
                .is_occupied()
            | (king_attacks & self.get_kings() & self.get_them()).is_occupied()
    }

    #[must_use]
    fn get_attackers(&self, sq: Square<8, 8>) -> Bitboard<8, 8> {
        let bb = Bitboard::from_square(sq);
        let blockers = self.get_occupied();
        let mut attackers = Bitboard::default();

        // Pawns
        attackers |= (bb.ne() | bb.nw()) & self.get_pawns();

        // Knights
        attackers |= bb.knights() & self.get_knights();

        // Bishops & Queens
        attackers |= bb.rays_diagonal(blockers) & (self.get_bishops() | self.get_queens());

        // Rooks & Queens
        attackers |= bb.rays_hor_ver(blockers) & (self.get_rooks() | self.get_queens());

        // King
        attackers |= bb.adjacent() & self.get_kings();

        attackers & self.get_them()
    }

    pub fn swap(&mut self) {
        // Swap colours
        self.colour[0] = self.colour[0].swap();
        self.colour[1] = self.colour[1].swap();
        (self.colour[0], self.colour[1]) = (self.colour[1], self.colour[0]);

        // Swap pieces
        self.pieces[ChessPiece::Pawn as usize] = self.pieces[ChessPiece::Pawn as usize].swap();
        self.pieces[ChessPiece::Knight as usize] = self.pieces[ChessPiece::Knight as usize].swap();
        self.pieces[ChessPiece::Bishop as usize] = self.pieces[ChessPiece::Bishop as usize].swap();
        self.pieces[ChessPiece::Rook as usize] = self.pieces[ChessPiece::Rook as usize].swap();
        self.pieces[ChessPiece::Queen as usize] = self.pieces[ChessPiece::Queen as usize].swap();
        self.pieces[ChessPiece::King as usize] = self.pieces[ChessPiece::King as usize].swap();

        // Swap ep
        if let Some(sq) = self.ep {
            self.ep = Some(sq.flipped());
        }

        self.flipped = !self.flipped;
    }
}

impl ChessPosition {
    #[must_use]
    pub fn can_castle(&self, perm: Castling) -> bool {
        match perm {
            Castling::WKS => self.us_ksc,
            Castling::WQS => self.us_qsc,
            Castling::BKS => self.them_ksc,
            Castling::BQS => self.them_qsc,
        }
    }

    #[must_use]
    pub fn in_check(&self) -> bool {
        let ksq = (self.board.get_kings() & self.board.get_us()).lsb();
        self.board.is_attacked(ksq)
    }

    #[must_use]
    fn can_move(&self) -> bool {
        let mut found = false;
        self.move_generator(|_| {
            found = true;
            true
        });
        found
    }

    #[must_use]
    pub fn parse_movestr(&self, word: &str) -> Result<ChessMove, &'static str> {
        if word.len() < 4 || word.len() > 5 {
            Err("movestr wrong length")
        } else {
            let mut from = Square::<8, 8>::from_string(&word[0..2]).unwrap();
            let mut to = Square::<8, 8>::from_string(&word[2..4]).unwrap();
            let promo = match word.chars().nth(4) {
                Some('q') => Some(ChessPiece::Queen),
                Some('r') => Some(ChessPiece::Rook),
                Some('b') => Some(ChessPiece::Bishop),
                Some('n') => Some(ChessPiece::Knight),
                Some(_) => return Err("invalid promotion character"),
                _ => None,
            };

            if self.board.flipped {
                from = from.flipped();
                to = to.flipped();
            }

            Ok(ChessMove { from, to, promo })
        }
    }

    pub fn move_to_string(&self, mv: &ChessMove) -> String {
        format!(
            "{}{}{}",
            if self.board.flipped {
                mv.from.flipped()
            } else {
                mv.from
            },
            if self.board.flipped {
                mv.to.flipped()
            } else {
                mv.to
            },
            match mv.promo {
                Some(ChessPiece::Queen) => "q",
                Some(ChessPiece::Rook) => "r",
                Some(ChessPiece::Bishop) => "b",
                Some(ChessPiece::Knight) => "n",
                Some(_) => panic!("Invalid promotion piece"),
                None => "",
            }
        )
    }

    fn flip(&mut self) {
        self.board.swap();
        (self.us_ksc, self.them_ksc) = (self.them_ksc, self.us_ksc);
        (self.us_qsc, self.them_qsc) = (self.them_qsc, self.us_qsc);
    }
}

impl GameRules for ChessPosition {
    type MoveType = ChessMove;
    const WIDTH: i32 = 8;
    const HEIGHT: i32 = 8;

    fn startpos() -> Self {
        Self {
            board: Board {
                colour: [Bitboard(0x000000000000FFFF), Bitboard(0xFFFF000000000000)],
                pieces: [
                    Bitboard(0x00FF00000000FF00),
                    Bitboard(0x4200000000000042),
                    Bitboard(0x2400000000000024),
                    Bitboard(0x8100000000000081),
                    Bitboard(0x0800000000000008),
                    Bitboard(0x1000000000000010),
                ],
                ep: None,
                flipped: false,
            },
            us_ksc: true,
            us_qsc: true,
            them_ksc: true,
            them_qsc: true,
            halfmoves: 0,
            fullmoves: 1,
            stack: vec![],
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, _yee: usize) -> bool {
        let sq = Square::from_coords(x, y);
        match c {
            'P' => self.board.set(sq, ChessPiece::Pawn, Side::Player1),
            'N' => self.board.set(sq, ChessPiece::Knight, Side::Player1),
            'B' => self.board.set(sq, ChessPiece::Bishop, Side::Player1),
            'R' => self.board.set(sq, ChessPiece::Rook, Side::Player1),
            'Q' => self.board.set(sq, ChessPiece::Queen, Side::Player1),
            'K' => self.board.set(sq, ChessPiece::King, Side::Player1),
            'p' => self.board.set(sq, ChessPiece::Pawn, Side::Player2),
            'n' => self.board.set(sq, ChessPiece::Knight, Side::Player2),
            'b' => self.board.set(sq, ChessPiece::Bishop, Side::Player2),
            'r' => self.board.set(sq, ChessPiece::Rook, Side::Player2),
            'q' => self.board.set(sq, ChessPiece::Queen, Side::Player2),
            'k' => self.board.set(sq, ChessPiece::King, Side::Player2),
            _ => panic!("Unrecognised piece char {}", c),
        }
        true
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let ny = if self.board.flipped { 7 - y } else { y };

        let flip_side = if self.board.flipped {
            Side::Player2
        } else {
            Side::Player1
        };

        let is_uppercase = self.board.side_on(x, ny) == Some(flip_side);

        match (is_uppercase, self.board.piece_on(x, ny)) {
            (true, Some(ChessPiece::Pawn)) => Some("P".to_owned()),
            (true, Some(ChessPiece::Knight)) => Some("N".to_owned()),
            (true, Some(ChessPiece::Bishop)) => Some("B".to_owned()),
            (true, Some(ChessPiece::Rook)) => Some("R".to_owned()),
            (true, Some(ChessPiece::Queen)) => Some("Q".to_owned()),
            (true, Some(ChessPiece::King)) => Some("K".to_owned()),
            (false, Some(ChessPiece::Pawn)) => Some("p".to_owned()),
            (false, Some(ChessPiece::Knight)) => Some("n".to_owned()),
            (false, Some(ChessPiece::Bishop)) => Some("b".to_owned()),
            (false, Some(ChessPiece::Rook)) => Some("r".to_owned()),
            (false, Some(ChessPiece::Queen)) => Some("q".to_owned()),
            (false, Some(ChessPiece::King)) => Some("k".to_owned()),
            (_, _) => None,
        }
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        let ksq = (self.board.get_kings() & self.board.get_us()).lsb();
        let kbb = Bitboard::from_square(ksq);
        let us = self.board.get_us();
        let attackers = self.board.get_attackers(ksq);
        let in_check = attackers.is_occupied();
        let (ray_n, xray_n) = kbb.xray_north(self.board.get_occupied());
        let (ray_e, xray_e) = kbb.xray_east(self.board.get_occupied());
        let (ray_s, xray_s) = kbb.xray_south(self.board.get_occupied());
        let (ray_w, xray_w) = kbb.xray_west(self.board.get_occupied());
        let (ray_ne, xray_ne) = kbb.xray_ne(self.board.get_occupied());
        let (ray_nw, xray_nw) = kbb.xray_nw(self.board.get_occupied());
        let (ray_se, xray_se) = kbb.xray_se(self.board.get_occupied());
        let (ray_sw, xray_sw) = kbb.xray_sw(self.board.get_occupied());

        let pinned_north = if (xray_n & self.board.get_them() & self.board.get_rq()).is_occupied() {
            ray_n & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_south = if (xray_s & self.board.get_them() & self.board.get_rq()).is_occupied() {
            ray_s & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_east = if (xray_e & self.board.get_them() & self.board.get_rq()).is_occupied() {
            ray_e & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_west = if (xray_w & self.board.get_them() & self.board.get_rq()).is_occupied() {
            ray_w & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_ne = if (xray_ne & self.board.get_them() & self.board.get_bq()).is_occupied() {
            ray_ne & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_nw = if (xray_nw & self.board.get_them() & self.board.get_bq()).is_occupied() {
            ray_nw & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_se = if (xray_se & self.board.get_them() & self.board.get_bq()).is_occupied() {
            ray_se & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_sw = if (xray_sw & self.board.get_them() & self.board.get_bq()).is_occupied() {
            ray_sw & self.board.get_us()
        } else {
            Bitboard::empty()
        };
        let pinned_ne_sw = pinned_ne | pinned_sw;
        let pinned_nw_se = pinned_nw | pinned_se;
        let pinned_diagonal = pinned_ne_sw | pinned_nw_se;
        let pinned_horizontal = pinned_east | pinned_west;
        let pinned_vertical = pinned_north | pinned_south;
        let pinned = pinned_diagonal | pinned_horizontal | pinned_vertical;

        let allowed = match attackers.count() {
            0 => Bitboard::all(),
            1 => {
                let mut mask = Bitboard::empty();

                mask |= kbb.knights() & self.board.get_knights() & self.board.get_them();

                if (ray_n & attackers).is_occupied() {
                    mask |= ray_n;
                }
                if (ray_s & attackers).is_occupied() {
                    mask |= ray_s;
                }
                if (ray_e & attackers).is_occupied() {
                    mask |= ray_e;
                }
                if (ray_w & attackers).is_occupied() {
                    mask |= ray_w;
                }
                if (ray_nw & attackers).is_occupied() {
                    mask |= ray_nw;
                }
                if (ray_ne & attackers).is_occupied() {
                    mask |= ray_ne;
                }
                if (ray_sw & attackers).is_occupied() {
                    mask |= ray_sw;
                }
                if (ray_se & attackers).is_occupied() {
                    mask |= ray_se;
                }

                mask
            }
            _ => Bitboard::empty(),
        };

        // Single pawn moves
        for to in (self.board.get_pawns() & us & !pinned_diagonal & !pinned_horizontal).north()
            & self.board.get_empty()
            & allowed
        {
            let from = Square::from_coords(to.get_file().0 as i32, to.get_rank().0 as i32 - 1);

            if to.get_rank() == Rank(7) {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Queen),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Rook),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Bishop),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Knight),
                }) {
                    return;
                }
            } else {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Pawn captures NE
        for from in self.board.get_pawns()
            & !pinned_horizontal
            & !pinned_vertical
            & !pinned_nw_se
            & self.board.get_us()
            & self.board.get_them().south().west()
            & allowed.south().west()
        {
            let to =
                Square::from_coords(from.get_file().0 as i32 + 1, from.get_rank().0 as i32 + 1);

            if to.get_rank() == Rank(7) {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Queen),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Rook),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Bishop),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Knight),
                }) {
                    return;
                }
            } else {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Pawn captures NW
        for from in self.board.get_pawns()
            & !pinned_horizontal
            & !pinned_vertical
            & !pinned_ne_sw
            & self.board.get_us()
            & self.board.get_them().south().east()
            & allowed.south().east()
        {
            let to =
                Square::from_coords(from.get_file().0 as i32 - 1, from.get_rank().0 as i32 + 1);

            if to.get_rank() == Rank(7) {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Queen),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Rook),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Bishop),
                }) {
                    return;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: Some(ChessPiece::Knight),
                }) {
                    return;
                }
            } else {
                func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                });
            }
        }

        // Double pawn moves
        for from in self.board.get_pawns()
            & self.board.get_us()
            & self.board.get_empty().south()
            & self.board.get_empty().south().south()
            & Bitboard::from_rank(Rank(1))
            & !pinned_diagonal
            & !pinned_horizontal
            & allowed.south().south()
        {
            let to = Square::from_coords(from.get_file().0 as i32, from.get_rank().0 as i32 + 2);
            if func(Self::MoveType {
                from,
                to,
                promo: None,
            }) {
                return;
            }
        }

        if let Some(ep) = self.board.ep {
            let bb = Bitboard::from_square(ep);

            if attackers.is_empty() || (attackers == bb.south()) || (allowed & bb).is_occupied() {
                // En passant NW
                if (bb.se()
                    & self.board.get_us()
                    & self.board.get_pawns()
                    & !pinned_horizontal
                    & !pinned_vertical
                    & !pinned_ne_sw)
                    .is_occupied()
                {
                    let occupied_after = self.board.get_occupied() ^ bb ^ bb.south() ^ bb.se();
                    let bad = self.board.get_rq() & self.board.get_them();
                    let is_pinned = ksq.get_rank() == Rank(4)
                        && ((kbb.ray_east(occupied_after) & bad).is_occupied()
                            || (kbb.ray_west(occupied_after) & bad).is_occupied());

                    if !is_pinned {
                        if func(Self::MoveType {
                            from: bb.se().lsb(),
                            to: ep,
                            promo: None,
                        }) {
                            return;
                        }
                    }
                }

                // En passant NE
                if (bb.sw()
                    & self.board.get_us()
                    & self.board.get_pawns()
                    & !pinned_horizontal
                    & !pinned_vertical
                    & !pinned_nw_se)
                    .is_occupied()
                {
                    let occupied_after = self.board.get_occupied() ^ bb ^ bb.south() ^ bb.sw();
                    let bad = self.board.get_rq() & self.board.get_them();
                    let is_pinned = ksq.get_rank() == Rank(4)
                        && ((kbb.ray_east(occupied_after) & bad).is_occupied()
                            || (kbb.ray_west(occupied_after) & bad).is_occupied());

                    if !is_pinned {
                        if func(Self::MoveType {
                            from: bb.sw().lsb(),
                            to: ep,
                            promo: None,
                        }) {
                            return;
                        }
                    }
                }
            }
        }

        // Knight moves
        for from in self.board.get_knights()
            & self.board.get_us()
            & !pinned_diagonal
            & !pinned_horizontal
            & !pinned_vertical
        {
            let bb = Bitboard::from_square(from);
            let moves = bb.knights();
            for to in moves & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Bishop moves
        for from in self.board.get_bishops() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_diagonal(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_bishops() & self.board.get_us() & pinned_ne_sw {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ne_sw(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_bishops() & self.board.get_us() & pinned_nw_se {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_nw_se(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Rook moves
        for from in self.board.get_rooks() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_rooks() & self.board.get_us() & pinned_vertical {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_rooks() & self.board.get_us() & pinned_horizontal {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Queen moves - bishop
        for from in self.board.get_queens() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_diagonal(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_ne_sw {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ne_sw(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_nw_se {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_nw_se(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Queen moves - rook
        for from in self.board.get_queens() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_vertical {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_horizontal {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // King moves
        for from in self.board.get_kings() & self.board.get_us() {
            let bb_from = Bitboard::from_square(from);
            let bb_other = self.board.get_occupied() ^ bb_from;

            for to in bb_from.adjacent() & !self.board.get_us() {
                let bb_to = Bitboard::from_square(to);
                let pawn_rays = bb_to.ne() | bb_to.nw();
                let knight_rays = bb_to.knights();
                let bishop_rays = bb_to.rays_diagonal(bb_other);
                let rook_rays = bb_to.rays_hor_ver(bb_other);
                let king_rays = bb_to.adjacent();
                let gives_check = (pawn_rays & self.board.get_them() & self.board.get_pawns())
                    .is_occupied()
                    | (knight_rays & self.board.get_them() & self.board.get_knights())
                        .is_occupied()
                    | (bishop_rays & self.board.get_them() & self.board.get_bq()).is_occupied()
                    | (rook_rays & self.board.get_them() & self.board.get_rq()).is_occupied()
                    | (king_rays & self.board.get_them() & self.board.get_kings()).is_occupied();

                if gives_check {
                    continue;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Castling - King side
        if self.can_castle(Castling::WKS)
            && self.board.is_empty(Square::from_coords(5, 0))
            && self.board.is_empty(Square::from_coords(6, 0))
            && !in_check
            && !self.board.is_attacked(Square::from_coords(5, 0))
            && !self.board.is_attacked(Square::from_coords(6, 0))
        {
            if func(Self::MoveType {
                from: Square::from_coords(4, 0),
                to: Square::from_coords(6, 0),
                promo: None,
            }) {
                return;
            }
        }

        // Castling - Queen side
        if self.can_castle(Castling::WQS)
            && self.board.is_empty(Square::from_coords(1, 0))
            && self.board.is_empty(Square::from_coords(2, 0))
            && self.board.is_empty(Square::from_coords(3, 0))
            && !in_check
            && !self.board.is_attacked(Square::from_coords(3, 0))
            && !self.board.is_attacked(Square::from_coords(2, 0))
        {
            if func(Self::MoveType {
                from: Square::from_coords(4, 0),
                to: Square::from_coords(2, 0),
                promo: None,
            }) {
                return;
            }
        }
    }

    fn get_turn(&self) -> Side {
        match self.board.flipped {
            true => Side::Player2,
            false => Side::Player1,
        }
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        debug_assert!(mv.from != mv.to);
        debug_assert!(mv.promo != Some(ChessPiece::Pawn));
        debug_assert!(mv.promo != Some(ChessPiece::King));

        let piece = self.board.piece_on_sq(mv.from);
        let captured = self.board.piece_on_sq(mv.to);
        debug_assert!(piece.is_some());
        debug_assert!(captured != Some(ChessPiece::King));

        // Push irrecoverable to stack
        self.stack.push(Irrecoverable {
            captured,
            halfmoves: self.halfmoves,
            ep: self.board.ep,
            us_ksc: self.us_ksc,
            us_qsc: self.us_qsc,
            them_ksc: self.them_ksc,
            them_qsc: self.them_qsc,
        });

        self.board.ep = None;
        self.halfmoves += 1;

        // Remove piece
        if let Some(p) = piece {
            self.board.remove(mv.from, p, Side::Player1);

            if p == ChessPiece::Pawn {
                self.halfmoves = 0;
            }
        }

        // Remove captured
        if let Some(p) = captured {
            self.board.remove(mv.to, p, Side::Player2);
            self.halfmoves = 0;
        }

        // Add piece
        if let Some(p) = piece {
            self.board.set(mv.to, p, Side::Player1);
        }

        debug_assert_eq!(self.board.piece_on_sq(mv.from), None);
        debug_assert_ne!(self.board.piece_on_sq(mv.to), None);

        // En passant
        if piece == Some(ChessPiece::Pawn)
            && mv.from.get_file() != mv.to.get_file()
            && captured == None
        {
            let sq = Square::from_coords(mv.to.get_file().0 as i32, 4);
            debug_assert_eq!(self.board.piece_on_sq(sq), Some(ChessPiece::Pawn));
            self.board.remove(sq, ChessPiece::Pawn, Side::Player2);
            debug_assert_eq!(self.board.piece_on_sq(sq), None);
        }

        // Double pawn move
        if piece == Some(ChessPiece::Pawn)
            && mv.from.get_rank() == Rank(1)
            && mv.to.get_rank() == Rank(3)
        {
            self.board.ep = Some(Square::<8, 8>::from_coords(mv.from.get_file().0 as i32, 2));
        }

        // Promotions
        if let Some(p) = mv.promo {
            // Remove pawn
            self.board.remove(mv.to, ChessPiece::Pawn, Side::Player1);

            // Add piece
            self.board.set(mv.to, p, Side::Player1);
        }

        // Castling - kingside
        if piece == Some(ChessPiece::King)
            && mv.from == Square::from_coords(4, 0)
            && mv.to == Square::from_coords(6, 0)
        {
            // Remove rook
            self.board
                .remove(Square::from_coords(7, 0), ChessPiece::Rook, Side::Player1);

            // Add rook
            self.board
                .set(Square::from_coords(5, 0), ChessPiece::Rook, Side::Player1);
        }

        // Castling - queenside
        if piece == Some(ChessPiece::King)
            && mv.from == Square::from_coords(4, 0)
            && mv.to == Square::from_coords(2, 0)
        {
            // Remove rook
            self.board
                .remove(Square::from_coords(0, 0), ChessPiece::Rook, Side::Player1);

            // Add rook
            self.board
                .set(Square::from_coords(3, 0), ChessPiece::Rook, Side::Player1);
        }

        // Castling permissions
        if mv.from == Square::from_coords(4, 0) {
            self.us_ksc = false;
            self.us_qsc = false;
        }
        if mv.from == Square::from_coords(0, 0) {
            self.us_qsc = false;
        }
        if mv.from == Square::from_coords(7, 0) {
            self.us_ksc = false;
        }
        if mv.to == Square::from_coords(0, 7) {
            self.them_qsc = false;
        }
        if mv.to == Square::from_coords(7, 7) {
            self.them_ksc = false;
        }

        self.flip();
        self.fullmoves += (self.board.flipped == false) as i32;
    }

    fn undomove(&mut self, mv: &Self::MoveType) {
        debug_assert!(!self.stack.is_empty());
        debug_assert!(mv.from != mv.to);
        debug_assert!(mv.promo != Some(ChessPiece::Pawn));
        debug_assert!(mv.promo != Some(ChessPiece::King));

        self.flip();

        let irr = self.stack.pop();
        let piece = self.board.piece_on_sq(mv.to);

        debug_assert!(piece.is_some(), "{} {:#?}", self, mv);
        debug_assert!(irr.unwrap().captured != Some(ChessPiece::King));

        // Remove piece
        if let Some(p) = piece {
            self.board.remove(mv.to, p, Side::Player1)
        }

        // Place piece
        if let Some(p) = piece {
            self.board.set(mv.from, p, Side::Player1);
        }

        // Promotions
        if let Some(p) = mv.promo {
            // Remove piece
            self.board.remove(mv.from, p, Side::Player1);

            // Add pawn
            self.board.set(mv.from, ChessPiece::Pawn, Side::Player1);
        }

        if let Some(old) = irr {
            // Replace captured
            if let Some(captured) = old.captured {
                self.board.set(mv.to, captured, Side::Player2);
            }

            // En passant
            if piece == Some(ChessPiece::Pawn) && Some(mv.to) == old.ep {
                let sq = Square::from_coords(mv.to.get_file().0 as i32, 4);
                self.board.set(sq, ChessPiece::Pawn, Side::Player2);
            }

            // Restore irrecoverable to stack
            self.halfmoves = old.halfmoves;
            self.board.ep = old.ep;
            self.us_ksc = old.us_ksc;
            self.us_qsc = old.us_qsc;
            self.them_ksc = old.them_ksc;
            self.them_qsc = old.them_qsc;
        }

        // Castling - kingside
        if piece == Some(ChessPiece::King)
            && mv.from == Square::from_coords(4, 0)
            && mv.to == Square::from_coords(6, 0)
        {
            // Remove rook
            self.board
                .remove(Square::from_coords(5, 0), ChessPiece::Rook, Side::Player1);

            // Add rook
            self.board
                .set(Square::from_coords(7, 0), ChessPiece::Rook, Side::Player1);
        }

        // Castling - queenside
        if piece == Some(ChessPiece::King)
            && mv.from == Square::from_coords(4, 0)
            && mv.to == Square::from_coords(2, 0)
        {
            // Remove rook
            self.board
                .remove(Square::from_coords(3, 0), ChessPiece::Rook, Side::Player1);

            // Add rook
            self.board
                .set(Square::from_coords(0, 0), ChessPiece::Rook, Side::Player1);
        }

        self.fullmoves -= (self.board.flipped == true) as i32;
    }

    fn makenull(&mut self) {
        todo!()
    }

    fn undonull(&mut self) {
        todo!()
    }

    fn get_result(&self) -> Option<GameResult> {
        let in_check = self.in_check();
        let them = !self.get_turn();

        // 50 move rule
        if self.fullmoves >= 100 {
            return Some(GameResult::Draw);
        }

        // Checkmate
        if in_check && !self.can_move() {
            return Some(GameResult::Win(them));
        }

        // Threefold

        None
    }

    fn get_fen(&self) -> String {
        let wksc = if self.board.flipped {
            self.them_ksc
        } else {
            self.us_ksc
        };
        let wqsc = if self.board.flipped {
            self.them_qsc
        } else {
            self.us_qsc
        };
        let bksc = if self.board.flipped {
            self.us_ksc
        } else {
            self.them_ksc
        };
        let bqsc = if self.board.flipped {
            self.us_qsc
        } else {
            self.them_qsc
        };

        format!(
            "{} {} {}{}{}{}{} {} {} {}",
            self.get_board_fen(),
            match self.get_turn() {
                Side::Player1 => "w",
                Side::Player2 => "b",
            },
            if wksc { "K" } else { "" },
            if wqsc { "Q" } else { "" },
            if bksc { "k" } else { "" },
            if bqsc { "q" } else { "" },
            if !(self.us_ksc | self.us_qsc | self.them_ksc | self.them_qsc) {
                "-"
            } else {
                ""
            },
            if let Some(sq) = self.board.ep {
                sq.to_string()
            } else {
                "-".to_string()
            },
            self.halfmoves,
            self.fullmoves
        )
    }

    fn parse_fen_part(&mut self, idx: usize, part: &str) {
        match idx {
            0 => {}
            1 => match part {
                "w" => self.board.flipped = false,
                "b" => self.board.flipped = true,
                _ => panic!("Uh oh {}", part),
            },
            // castling
            2 => {
                self.us_ksc = part.contains("K");
                self.us_qsc = part.contains("Q");
                self.them_ksc = part.contains("k");
                self.them_qsc = part.contains("q");
            }
            // en passant
            3 => self.board.ep = Square::from_string(part).ok(),
            4 => self.halfmoves = part.parse::<i32>().unwrap(),
            5 => {
                self.fullmoves = part.parse::<i32>().unwrap();
                if self.board.flipped {
                    self.flip();
                    self.board.flipped = true;
                }
            }
            _ => panic!("Invalid fen part index"),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nboard = if self.flipped {
            let mut nboard = self.clone();
            nboard.swap();
            nboard
        } else {
            self.clone()
        };

        for y in (0..8).rev() {
            for x in 0..8 {
                match (nboard.side_on(x, y), nboard.piece_on(x, y)) {
                    (Some(Side::Player1), Some(ChessPiece::Pawn)) => write!(f, "P")?,
                    (Some(Side::Player1), Some(ChessPiece::Knight)) => write!(f, "N")?,
                    (Some(Side::Player1), Some(ChessPiece::Bishop)) => write!(f, "B")?,
                    (Some(Side::Player1), Some(ChessPiece::Rook)) => write!(f, "R")?,
                    (Some(Side::Player1), Some(ChessPiece::Queen)) => write!(f, "Q")?,
                    (Some(Side::Player1), Some(ChessPiece::King)) => write!(f, "K")?,
                    (Some(Side::Player2), Some(ChessPiece::Pawn)) => write!(f, "p")?,
                    (Some(Side::Player2), Some(ChessPiece::Knight)) => write!(f, "n")?,
                    (Some(Side::Player2), Some(ChessPiece::Bishop)) => write!(f, "b")?,
                    (Some(Side::Player2), Some(ChessPiece::Rook)) => write!(f, "r")?,
                    (Some(Side::Player2), Some(ChessPiece::Queen)) => write!(f, "q")?,
                    (Some(Side::Player2), Some(ChessPiece::King)) => write!(f, "k")?,
                    (_, _) => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for ChessPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)?;

        match self.get_turn() {
            Side::Player1 => writeln!(f, "Turn: w")?,
            Side::Player2 => writeln!(f, "Turn: b")?,
        }

        write!(f, "Castling: ")?;
        if self.us_ksc {
            write!(f, "K")?;
        }
        if self.us_qsc {
            write!(f, "Q")?;
        }
        if self.them_ksc {
            write!(f, "k")?;
        }
        if self.them_qsc {
            write!(f, "q")?;
        }
        writeln!(f, "")?;

        writeln!(f, "Halfmoves: {}", self.halfmoves)?;
        writeln!(f, "Fullmoves: {}", self.fullmoves)?;

        Ok(())
    }
}
