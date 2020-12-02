pub use itertools;
pub use itertools::Itertools;

pub trait IterExt: Iterator + Sized {
    fn count_if<P>(self, mut predicate: P) -> usize
    where
        P: FnMut(Self::Item) -> bool,
    {
        let mut count = 0usize;
        for item in self {
            if predicate(item) {
                count += 1;
            }
        }
        count
    }

    fn count_if_res<P, E>(self, mut predicate: P) -> Result<usize, E>
    where
        P: FnMut(Self::Item) -> Result<bool, E>,
    {
        let mut count = 0usize;
        for item in self {
            if predicate(item)? {
                count += 1;
            }
        }
        Ok(count)
    }
}

impl<I: Iterator> IterExt for I {}
