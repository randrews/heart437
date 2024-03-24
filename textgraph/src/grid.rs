use std::ops;
use std::slice::Iter;
use crate::coords::{Coord, xy};

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    default: T
}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        let lines: Vec<Vec<_>> = value.lines().map(|line| line.chars().collect()).collect();
        let width = lines[0].len();
        let cells = lines.concat();
        Self {
            cells,
            width,
            default: ' '
        }
    }
}

impl<T> ops::Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.cells[index.1 as usize * self.width + index.0 as usize]
    }
}

impl<T> ops::IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.cells[index.1 as usize * self.width + index.0 as usize]
    }
}

impl<T: Clone + Copy> Grid<T> {
    pub fn new(size: Coord, default: T) -> Grid<T> {
        let (width, height) = (size.0 as usize, size.1 as usize);
        let cells = vec![default; width * height];
        Self { cells, width, default }
    }

    pub fn from_vec(cells: Vec<T>, width: usize, default: T) -> Self {
        Self { cells, width, default }
    }

    pub fn as_blank(&self) -> Self {
        Self::new(self.dimensions(), self.default)
    }

    pub fn contains(&self, point: Coord) -> bool {
        let dims = self.dimensions();
        !(point.0 < 0 || point.1 < 0 ||
            point.1 >= dims.1 ||
            point.0 >= dims.0)
    }

    pub fn dimensions(&self) -> Coord {
        xy(
            self.width as i32,
            (self.cells.len() / self.width) as i32
        )
    }

    pub fn get(&self, index: Coord) -> Option<&T> {
        if self.contains(index) {
            Some(&self[index])
        } else {
            None
        }
    }

    pub fn get_or_default(&self, index: Coord) -> &T {
        if self.contains(index) {
            &self[index]
        } else {
            &self.default
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.cells.iter()
    }

    pub fn coord(&self, idx: usize) -> Coord {
        xy((idx % self.width) as i32, (idx / self.width) as i32)
    }

    pub fn copy_region(&self, to: &mut Grid<T>, at: Coord) {
        for (i, cell) in self.iter().enumerate() {
            let c = self.coord(i);
            to[c + at] = *cell
        }
    }

    pub fn fill(&mut self, val: T) {
        self.cells.fill(val)
    }
}

impl<T: PartialEq + Clone + Copy> Grid<T> {
    /// Return a 4-tuple of (n, s, e, w) for whether those neighbors are equal to the
    /// given value
    pub fn neighbors(&self, point: Coord, val: T) -> (bool, bool, bool, bool) {
        let (x, y) = (point.0, point.1);
        let n = *self.get_or_default(xy(x, y-1)) == val;
        let s = *self.get_or_default(xy(x, y+1)) == val;
        let e = *self.get_or_default(xy(x+1, y)) == val;
        let w = *self.get_or_default(xy(x-1, y)) == val;
        (n, s, e, w)
    }

    /// Return a 4-tuple of (ne, se, sw, nw) for whether those neighbors are equal to the
    /// given value
    pub fn diagonal_neighbors(&self, point: Coord, val: T) -> (bool, bool, bool, bool) {
        let (x, y) = (point.0, point.1);
        let ne = *self.get_or_default(xy(x+1, y-1)) == val;
        let se = *self.get_or_default(xy(x+1, y+1)) == val;
        let sw = *self.get_or_default(xy(x-1, y+1)) == val;
        let nw = *self.get_or_default(xy(x-1, y-1)) == val;
        (ne, se, sw, nw)
    }

    pub fn count_neighbors(&self, point: Coord, val: T, diag: bool) -> i32 {
        let mut t = 0;
        let (n, s, e, w) = self.neighbors(point, val);
        if n { t += 1 }
        if s { t += 1 }
        if e { t += 1 }
        if w { t += 1 }

        if diag {
            let (ne, se, sw, nw) = self.diagonal_neighbors(point, val);
            if ne { t += 1 }
            if se { t += 1 }
            if sw { t += 1 }
            if nw { t += 1 }
        }

        t
    }

    pub fn any_neighbors(&self, point: Coord, val: T) -> bool {
        let (n, s, e, w) = self.neighbors(point, val);
        n || s || e || w
    }

    pub fn neighbors_with<F: Fn(T, Coord) -> bool>(&self, point: Coord, pred: F) -> (bool, bool, bool, bool) {
        let (x, y) = (point.0, point.1);
        let n = pred(*self.get_or_default(xy(x, y-1)), xy(x, y-1));
        let s = pred(*self.get_or_default(xy(x, y+1)), xy(x, y+1));
        let e = pred(*self.get_or_default(xy(x+1, y)), xy(x+1, y));
        let w = pred(*self.get_or_default(xy(x-1, y)), xy(x-1, y));
        (n, s, e, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        // ABA
        // BBA
        // AAA
        let grid = Grid::from("ABA\nBBA\nAAA");
        // The one in the center:
        assert_eq!(grid.neighbors(xy(1, 1), 'B'), (true, false, false, true));

        // One near the edge:
        assert_eq!(grid.neighbors(xy(1, 0), 'B'), (false, true, false, false))
    }

    #[test]
    fn test_neighbors_with() {
        // ABA
        // CBA
        // AAA
        let grid = Grid::from("aba\ncba\naaa");
        let nbrs = grid.neighbors_with(xy(1, 1), |c, _p| c == 'b' || c == 'c');
        // It finds all b's and c's, so north and west
        assert_eq!(nbrs, (true, false, false, true))
    }
}