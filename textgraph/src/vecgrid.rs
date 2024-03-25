use std::ops::{Index, IndexMut};
use crate::{Coord, Grid, xy};

/// An implementation of Grid backed by a Vec
#[derive(Clone)]
pub struct VecGrid<T> {
    cells: Vec<T>,
    width: usize,
    default: T
}

impl<T: Clone> Index<Coord> for VecGrid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.cells[index.1 as usize * self.width + index.0 as usize]
    }
}

impl<T: Clone> IndexMut<Coord> for VecGrid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.cells[index.1 as usize * self.width + index.0 as usize]
    }
}

impl<T: Clone> Grid for VecGrid<T> {
    fn size(&self) -> Coord {
        xy(self.width as i32, (self.cells.len() / self.width) as i32)
    }

    fn default(&self) -> Self::Output {
        self.default.clone()
    }
}

impl<T: Clone + Copy> VecGrid<T> {
    pub fn new(size: Coord, default: T) -> VecGrid<T> {
        let (width, height) = (size.0 as usize, size.1 as usize);
        let cells = vec![default; width * height];
        Self { cells, width, default }
    }

    pub fn from_vec(cells: Vec<T>, width: usize, default: T) -> Self {
        Self { cells, width, default }
    }
}

impl From<&str> for VecGrid<char> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vecgrid() {
        let mut grid = VecGrid::from("AB\nCD");
        grid[xy(0, 0)] = 'z';
        assert_eq!(grid[xy(0, 0)], 'z');
        assert_eq!(grid[xy(0, 1)], 'C');
        assert_eq!(grid.get(xy(2, 2)), None);
    }
}