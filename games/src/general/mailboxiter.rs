use super::{mailbox::Mailbox, square::Square};

#[derive(Debug, Clone)]
pub struct MailboxIter<const WIDTH: usize, const HEIGHT: usize>(usize);

impl<const WIDTH: usize, const HEIGHT: usize> Iterator for MailboxIter<WIDTH, HEIGHT> {
    type Item = Square<WIDTH, HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == WIDTH * HEIGHT {
            None
        } else {
            let f = (self.0 % WIDTH) as i32;
            let r = (self.0 / WIDTH) as i32;
            let sq = Square::<WIDTH, HEIGHT>::from_coords(f, r);
            self.0 += 1;
            Some(sq)
        }
    }
}

impl<T: Copy + Default + PartialEq, const WIDTH: usize, const HEIGHT: usize> IntoIterator
    for &Mailbox<T, WIDTH, HEIGHT>
{
    type Item = Square<WIDTH, HEIGHT>;
    type IntoIter = MailboxIter<WIDTH, HEIGHT>;

    fn into_iter(self) -> Self::IntoIter {
        MailboxIter(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::general::{mailbox::Mailbox, square::Square};

    #[test]
    fn iter() {
        let mailbox = Mailbox::<i32, 8, 8>::default();
        let mut iter = mailbox.into_iter();
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(0)));
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(1)));
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(2)));
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(3)));
    }
}
