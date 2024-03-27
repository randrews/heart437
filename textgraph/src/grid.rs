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

    /// Returns the `Coord` representing the nth cell in the grid, in reading order:
    /// left-to-right, top-to-bottom. This is useful because this is also the order that
    /// an `iter()` traverses the grid:
    /// ```
    /// # use textgraph::*;
    /// let grid = VecGrid::from("ABC\nDEF");
    /// for (n, cell) in grid.iter().enumerate() {
    ///     let pt = grid.coord(n);
    /// }
    /// ```
    fn coord(&self, n: usize) -> Coord {
        let Coord(width, _) = self.size();
        xy(n as i32 % width, n as i32 / width)
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

    /// Convenience method for `for_neighbors` just comparing with ==
    fn neighbors_equal(&self, point: Coord, val: Self::Output) -> (bool, bool, bool, bool)
        where Self::Output: PartialEq + Sized {
        self.for_neighbors(point, |_, cell| *cell == val)
    }

    /// Convenience method for `for_diagonals` just comparing with ==
    fn diagonals_equal(&self, point: Coord, val: Self::Output) -> (bool, bool, bool, bool)
        where Self::Output: PartialEq + Sized {
        self.for_diagonals(point, |_, cell| *cell == val)
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
    type Item = &'a G::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let coord = self.grid.coord(self.n as usize);
        self.n += 1;
        self.grid.get(coord)
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
        assert_eq!(it.next(), Some(&'A'));
        assert_eq!(it.next(), Some(&'B'));
        assert_eq!(it.next(), Some(&'C'));
        assert_eq!(it.next(), Some(&'D'));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_neighbors() {
        // ABA
        // BBA
        // AAA
        let grid = TestGrid::from("ABA\nBBA\nAAA");
        // The one in the center:
        assert_eq!(grid.neighbors_equal(xy(1, 1), 'B'), (true, false, false, true));

        // One near the edge:
        assert_eq!(grid.neighbors_equal(xy(1, 0), 'B'), (false, true, false, false))
    }
}