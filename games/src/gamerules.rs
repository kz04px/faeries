use crate::general::side::Side;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Win(Side),
    Draw,
}

pub trait GameRules: Sized + Copy + Default {
    type MoveType: Copy;
    const WIDTH: i32;
    const HEIGHT: i32;

    fn startpos() -> Self;

    fn set_piece(&mut self, x: i32, y: i32, c: char);

    fn set_fen(&mut self, fen: &str) {
        if fen == "startpos" {
            *self = Self::startpos();
            return;
        }

        *self = Self::default();
        let parts: Vec<&str> = fen.split(' ').collect();

        let mut x: i32 = 0;
        let mut y: i32 = Self::HEIGHT - 1;
        for c in parts[0].chars() {
            match c {
                '1'..='9' => x += (c as u8 - b'0') as i32,
                '/' => {
                    x = 0;
                    y -= 1;
                }
                _ => {
                    self.set_piece(x, y, c);
                    x += 1;
                }
            }
        }

        for (idx, word) in parts[1..].iter().enumerate() {
            self.parse_fen_part(idx + 1, word);
        }
    }

    #[must_use]
    fn get_square_string(&self, x: i32, y: i32) -> Option<String>;

    #[must_use]
    fn get_board_fen(&self) -> String {
        let mut fen = String::from("");

        for y in (0..Self::HEIGHT).rev() {
            let mut spaces = 0;

            for x in 0..Self::WIDTH {
                if let Some(a) = self.get_square_string(x, y) {
                    if spaces > 0 {
                        fen += &spaces.to_string();
                        spaces = 0;
                    }
                    fen += &a;
                } else {
                    spaces += 1;
                }
            }

            if spaces > 0 {
                fen += &spaces.to_string();
            }

            if y > 0 {
                fen += "/";
            }
        }

        fen
    }

    fn move_generator(&self, func: impl FnMut(Self::MoveType));

    #[must_use]
    fn from_fen(fen: &str) -> Self {
        let mut pos = Self::default();
        pos.set_fen(fen);
        pos
    }

    #[must_use]
    fn legal_moves(&self) -> Vec<Self::MoveType> {
        let mut moves = vec![];

        self.move_generator(|mv| {
            moves.push(mv);
        });

        moves
    }

    #[must_use]
    fn get_turn(&self) -> Side;

    fn makemove(&mut self, mv: &Self::MoveType);

    #[must_use]
    fn after_move(&self, mv: &Self::MoveType) -> Self {
        let mut npos = *self;
        npos.makemove(mv);
        npos
    }

    #[must_use]
    fn is_gameover(&self) -> bool {
        self.get_result().is_some()
    }

    #[must_use]
    fn get_result(&self) -> Option<GameResult>;

    #[must_use]
    fn count_moves(&self) -> u64 {
        let mut nodes = 0;
        self.move_generator(|_| {
            nodes += 1;
        });
        nodes
    }

    #[must_use]
    fn get_fen(&self) -> String;

    fn parse_fen_part(&mut self, idx: usize, part: &str);
}
