use super::square::Square;

#[derive(PartialEq, Debug, Clone)]
pub struct Mailbox<T: Copy + Default + PartialEq, const WIDTH: usize, const HEIGHT: usize> {
    data: [[T; HEIGHT]; WIDTH],
}

impl<T: Copy + Default + PartialEq, const WIDTH: usize, const HEIGHT: usize>
    Mailbox<T, WIDTH, HEIGHT>
{
    pub fn set_piece_square(&mut self, sq: Square<WIDTH, HEIGHT>, value: T) {
        self.data[sq.get_file()][sq.get_rank()] = value;
    }

    pub fn set_piece_coords(&mut self, x: i32, y: i32, value: T) {
        self.data[x as usize][y as usize] = value;
    }

    #[must_use]
    pub fn get_width(&self) -> i32 {
        WIDTH as i32
    }

    #[must_use]
    pub fn get_height(&self) -> i32 {
        HEIGHT as i32
    }

    #[must_use]
    pub fn get_piece_square(&self, sq: Square<WIDTH, HEIGHT>) -> T {
        self.data[sq.get_file()][sq.get_rank()]
    }

    #[must_use]
    pub fn get_piece_coords(&self, x: i32, y: i32) -> T {
        self.data[x as usize][y as usize]
    }

    pub fn fill(&mut self, value: T) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.data[x][y] = value;
            }
        }
    }

    #[must_use]
    pub fn on_board(&self, x: i32, y: i32) -> bool {
        0 <= x && x < WIDTH as i32 && 0 <= y && y < HEIGHT as i32
    }

    #[must_use]
    pub fn streak<const MAX_LEN: i32, const DX: i32, const DY: i32>(&self, x: i32, y: i32) -> i32 {
        let piece = self.get_piece_coords(x, y);
        let mut count = 1;

        for i in 1..=MAX_LEN {
            let nx = x + DX * i;
            let ny = y + DY * i;

            if !self.on_board(nx, ny) {
                break;
            } else if self.get_piece_coords(nx, ny) == piece {
                count += 1;
            }
        }

        count
    }

    #[must_use]
    pub fn count_piece(&self, piece: T) -> i32 {
        let mut count = 0;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                count += (self.get_piece_coords(x as i32, y as i32) == piece) as i32;
            }
        }
        count
    }

    pub fn set_piece(&mut self, x: i32, y: i32, piece: T) {
        self.data[x as usize][y as usize] = piece;
    }

    #[must_use]
    pub fn size(&self) -> i32 {
        (WIDTH * HEIGHT) as i32
    }
}

impl<T: Copy + Default + PartialEq, const WIDTH: usize, const HEIGHT: usize> Default
    for Mailbox<T, WIDTH, HEIGHT>
{
    fn default() -> Self {
        Self {
            data: [[T::default(); HEIGHT]; WIDTH],
        }
    }
}

#[cfg(test)]
mod test {
    use super::Mailbox;

    #[test]
    fn basic() {
        let board = Mailbox::<i32, 8, 6>::default();
        assert_eq!(board.get_width(), 8);
        assert_eq!(board.get_height(), 6);
        assert_eq!(board.size(), 8 * 6);
    }

    #[test]
    fn set() {
        let mut board = Mailbox::<i32, 8, 6>::default();

        for y in 0..board.get_height() {
            for x in 0..board.get_width() {
                assert_eq!(board.get_piece_coords(x, y), 0);
                board.set_piece(x, y, 3);
                assert_eq!(board.get_piece_coords(x, y), 3);
            }
        }
    }

    #[test]
    fn piece_type() {
        #[derive(Clone, Copy, Default, PartialEq, Debug)]
        enum PieceType {
            #[default]
            None,
            Pawn,
            King,
        }
        let mut board = Mailbox::<PieceType, 8, 6>::default();

        for y in 0..board.get_height() {
            for x in 0..board.get_width() {
                assert_eq!(board.get_piece_coords(x, y), PieceType::None);

                board.set_piece(x, y, PieceType::Pawn);
                assert_eq!(board.get_piece_coords(x, y), PieceType::Pawn);

                board.set_piece(x, y, PieceType::King);
                assert_eq!(board.get_piece_coords(x, y), PieceType::King);

                board.set_piece(x, y, PieceType::None);
                assert_eq!(board.get_piece_coords(x, y), PieceType::None);
            }
        }
    }

    #[test]
    fn option() {
        type PieceType = Option<i32>;
        let mut board = Mailbox::<PieceType, 8, 6>::default();

        for y in 0..board.get_height() {
            for x in 0..board.get_width() {
                assert_eq!(board.get_piece_coords(x, y), None);

                board.set_piece(x, y, Some(0));
                assert_eq!(board.get_piece_coords(x, y), Some(0));

                board.set_piece(x, y, Some(1));
                assert_eq!(board.get_piece_coords(x, y), Some(1));

                board.set_piece(x, y, None);
                assert_eq!(board.get_piece_coords(x, y), None);
            }
        }
    }
}
