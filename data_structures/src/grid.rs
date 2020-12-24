//! A flexible rectangular grid with various accessing and storage policies.
use std::ops::Bound;

use crate::{vec2, vector::*};

pub struct Grid<S: Storage> {
    data: S,
}

pub trait Storage {
    type Cell;

    fn get(&self, at: i64x2) -> Option<&Self::Cell>;
    fn get_mut(&mut self, at: i64x2) -> Option<&mut Self::Cell>;
}

pub trait BoundedStorage: Storage {
    // Should return the minimum and maximum bound (inclusive)
    fn bounds(&self) -> (i64x2, i64x2);
}

pub struct DenseStorage<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Storage for DenseStorage<T> {
    type Cell = T;

    fn get(&self, at: i64x2) -> Option<&Self::Cell> {
        if at.x < 0 || at.x as usize >= self.width || at.y < 0 {
            None
        } else {
            let index = at.x as usize + (at.y as usize * self.width);
            self.data.get(index)
        }
    }

    fn get_mut(&mut self, at: i64x2) -> Option<&mut Self::Cell> {
        if at.x < 0 || at.x as usize >= self.width || at.y < 0 {
            None
        } else {
            let index = at.x as usize + (at.y as usize * self.width);
            self.data.get_mut(index)
        }
    }
}

impl<T> BoundedStorage for DenseStorage<T> {
    #[inline]
    fn bounds(&self) -> (i64x2, i64x2) {
        (
            i64x2::default(),
            vec2![self.width as i64, (self.data.len() / self.width) as i64],
        )
    }
}

impl<S> Grid<S>
where
    S: BoundedStorage,
{
    pub fn rows(&self) -> GridRows<S> {
        let (min, _max) = self.data.bounds();
        GridRows {
            grid: self,
            row_index: min.x,
        }
    }
}

pub struct GridRows<'g, S: BoundedStorage> {
    grid: &'g Grid<S>,
    row_index: i64,
}
