use std::ops::{Index};
use crate::coords::{Coord, xy};

/// A trait for operations on a 2d grid of objects
pub trait Grid: Index<Coord> {
    /// Return how large the grid is
    fn size(&self) -> Coord;

    /// The default value to use for unset cells in the grid (or if we look at
    /// a cell outside the grid)
    fn default(&self) -> Self::Output;

    /// Is a given point inside the grid?
    fn contains(&self, point: Coord) -> bool {
        let dims = self.size();
        !(point.0 < 0 || point.1 < 0 ||
            point.1 >= dims.1 ||
            point.0 >= dims.0)
    }

    /// Get an Option<T>. `grid[place]` can panic if you access outside the grid; this won't
    fn get(&self, index: Coord) -> Option<&Self::Output> {
        if self.contains(index) {
            Some(&self[index])
        } else {
            None
        }
    }

    fn iter(&self) -> GridIterator<Self> where Self: Sized {
        GridIterator { grid: &self, n: 0 }
    }

    /// Apply the given predicate to the cell at the given point. If the point is outside
    /// the grid, apply it to the default instead
    fn apply<F: Predicate<Self::Output>>(&self, point: Coord, pred: &F) -> bool
        where <Self as Index<Coord>>::Output: Sized {
        pred.apply(point, self.get(point).unwrap_or(&self.default()))
    }

    /// Runs a lambda on the neighbors of this cell and returns the directions for which it returns
    /// true, as a `(n, s, e, w)` tuple. Uses apply, so "neighbors" outside the grid will get the default value.
    /// ```
    /// # use textgraph::*;
    /// let grid = VecGrid::from("+A\nAB");
    /// let matching = grid.neighbors_matching(xy(0, 0), EqualityPredicate::from('A'));
    /// let how_many = matching.count();
    /// ```
    fn neighbors_matching<T: Predicate<Self::Output>>(&self, point: Coord, pred: T) -> (bool, bool, bool, bool)
        where Self::Output: Sized {
        let (x, y) = (point.0, point.1);
        let n = self.apply(xy(x, y-1), &pred);
        let s = self.apply(xy(x, y+1), &pred);
        let e = self.apply(xy(x+1, y), &pred);
        let w = self.apply(xy(x-1, y), &pred);
        (n, s, e, w)
    }

    /// Just like `neighbors_matching` except it returns a tuple of `(ne, se, sw, nw)`
    fn diagonals_matching<T: Predicate<Self::Output>>(&self, point: Coord, pred: T) -> (bool, bool, bool, bool)
        where Self::Output: Sized {
        let (x, y) = (point.0, point.1);
        let ne = self.apply(xy(x+1, y-1), &pred);
        let se = self.apply(xy(x+1, y+1), &pred);
        let nw = self.apply(xy(x-1, y-1), &pred);
        let sw = self.apply(xy(x-1, y+1), &pred);
        (ne, se, sw, nw)
    }

    /// Runs a given lambda on all orthogonally-adjacent cells, running it on the default
    /// for any cells not in the grid
    /// ```
    /// # use textgraph::*;
    /// let grid = VecGrid::from("+A\nAB");
    /// // Downcase all the neighbors:
    /// let cs = grid.for_neighbors(xy(0, 0), |_c, ch| ch.to_lowercase().next().unwrap());
    /// ```
    fn for_neighbors<T, F: Fn(Coord, &Self::Output) -> T>(&self, point: Coord, func: F) -> (T, T, T, T)
        where Self::Output: Sized {
        let def = self.default();
        let Coord(x, y) = point;
        let n = func(xy(x, y-1), self.get(xy(x, y-1)).unwrap_or(&def));
        let s = func(xy(x, y+1), self.get(xy(x, y+1)).unwrap_or(&def));
        let e = func(xy(x+1, y), self.get(xy(x+1, y)).unwrap_or(&def));
        let w = func(xy(x-1, y), self.get(xy(x-1, y)).unwrap_or(&def));
        (n, s, e, w)
    }

    /// Just like `for_neighbors` except it returns `(ne, se, sw, nw)`
    fn for_diagonals<T, F: Fn(Coord, &Self::Output) -> T>(&self, point: Coord, func: F) -> (T, T, T, T)
        where Self::Output: Sized {
        let def = self.default();
        let Coord(x, y) = point;
        let ne = func(xy(x+1, y-1), self.get(xy(x+1, y-1)).unwrap_or(&def));
        let se = func(xy(x+1, y+1), self.get(xy(x+1, y+1)).unwrap_or(&def));
        let sw = func(xy(x-1, y+1), self.get(xy(x-1, y+1)).unwrap_or(&def));
        let nw = func(xy(x-1, y-1), self.get(xy(x-1, y-1)).unwrap_or(&def));
        (ne, se, sw, nw)
    }

    /// Convenience method for `neighbors_matching` just comparing with ==
    fn neighbors_equal(&self, point: Coord, val: Self::Output) -> (bool, bool, bool, bool)
        where Self::Output: PartialEq + Sized {
        self.neighbors_matching(point, EqualityPredicate::from(val))
    }

    /// Convenience method for `diagonals_matching` just comparing with ==
    fn diagonals_equal(&self, point: Coord, val: Self::Output) -> (bool, bool, bool, bool)
        where Self::Output: PartialEq + Sized {
        self.diagonals_matching(point, EqualityPredicate::from(val))
    }
}

/// Trait impld on `(bool, bool, bool, bool)` to make it easy to count
/// how many neighbors fit some criteria (since neighbors and diagonals fns
/// in Grid return that tuple)
pub trait CountableNeighbors {
    fn count(&self) -> i32;
}

impl CountableNeighbors for (bool, bool, bool, bool) {
    fn count(&self) -> i32 {
        let mut t = 0;
        let (n, s, e, w) = self;
        if *n { t += 1 }
        if *s { t += 1 }
        if *e { t += 1 }
        if *w { t += 1 }
        t
    }
}

pub struct GridIterator<'a, G> where G: Grid + Sized {
    grid: &'a G,
    n: i32,
}

impl<'a, G: Grid> Iterator for GridIterator<'a, G> {
    type Item = (Coord, &'a G::Output);

    fn next(&mut self) -> Option<Self::Item> {
        let width = self.grid.size().0;
        let coord = xy(self.n % width, self.n / width);
        self.n += 1;
        self.grid.get(coord).map(|v| (coord, v))
    }
}

pub trait Predicate<T> {
    fn apply(&self, coord: Coord, value: &T) -> bool;
}

pub struct EqualityPredicate<T: PartialEq>(T);
impl<T: PartialEq> Predicate<T> for EqualityPredicate<T> {
    fn apply(&self, _: Coord, value: &T) -> bool {
        *value == self.0
    }
}

impl<T: PartialEq> From<T> for EqualityPredicate<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, F: Fn(Coord, &T) -> bool> Predicate<T> for F {
    fn apply(&self, coord: Coord, value: &T) -> bool {
        self(coord, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGrid(Vec<char>, i32);
    impl Index<Coord> for TestGrid {
        type Output = char;
        fn index(&self, index: Coord) -> &Self::Output { &self.0[index.0 as usize + (index.1 * self.1) as usize] }
    }

    impl Grid for TestGrid {
        fn size(&self) -> Coord { xy(self.1, self.0.len() as i32 / self.1) }
        fn default(&self) -> Self::Output { ' ' }
    }

    impl From<&str> for TestGrid {
        fn from(value: &str) -> Self {
            let lines: Vec<Vec<_>> = value.lines().map(|line| line.chars().collect()).collect();
            let width = lines[0].len() as i32;
            Self(lines.concat(), width)
        }
    }

    #[test]
    fn test_iter() {
        let grid = TestGrid::from("AB\nCD");
        let mut it = grid.iter();
        assert_eq!(it.next(), Some((xy(0, 0), &'A')));
        assert_eq!(it.next(), Some((xy(1, 0), &'B')));
        assert_eq!(it.next(), Some((xy(0, 1), &'C')));
        assert_eq!(it.next(), Some((xy(1, 1), &'D')));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_neighbors() {
        // ABA
        // BBA
        // AAA
        let grid = TestGrid::from("ABA\nBBA\nAAA");
        // The one in the center:
        assert_eq!(grid.neighbors_matching(xy(1, 1), EqualityPredicate::from('B')), (true, false, false, true));

        // One near the edge:
        assert_eq!(grid.neighbors_matching(xy(1, 0), EqualityPredicate::from('B')), (false, true, false, false))
    }
}