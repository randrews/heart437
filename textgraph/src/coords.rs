use std::fmt::Formatter;
use std::ops::{Add, Div, Mul};

#[derive(Copy, Clone, PartialEq)]
pub enum Dir { North, South, East, West }

/// A tile coordinate: tiles are 8x8 pixels
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord(pub i32, pub i32);
pub fn xy(x: i32, y: i32) -> Coord {
    Coord(x, y)
}

impl Coord {
    pub fn north(&self) -> Coord { Coord(self.0, self.1 - 1) }
    pub fn south(&self) -> Coord { Coord(self.0, self.1 + 1) }
    pub fn east(&self) -> Coord { Coord(self.0 + 1, self.1) }
    pub fn west(&self) -> Coord { Coord(self.0 - 1, self.1) }

    pub fn northwest(&self) -> Coord { Coord(self.0 - 1, self.1 - 1) }
    pub fn southwest(&self) -> Coord { Coord(self.0 - 1, self.1 + 1) }
    pub fn northeast(&self) -> Coord { Coord(self.0 + 1, self.1 - 1) }
    pub fn southeast(&self) -> Coord { Coord(self.0 + 1, self.1 + 1) }

    pub fn translate(&self, dir: Dir) -> Coord {
        match dir {
            Dir::North => self.north(),
            Dir::South => self.south(),
            Dir::East => self.east(),
            Dir::West => self.west()
        }
    }

    pub fn within(&self, other: Coord) -> bool {
        self.0 >= 0 && self.1 >= 0 &&
            self.0 < other.0 && self.1 < other.1
    }

    pub fn dist_to(&self, other: Coord) -> f32 {
        let (dx, dy) = (self.0 - other.0, self.1 - other.1);
        ((dx * dx) as f32 + (dy * dy) as f32).sqrt()
    }

    pub fn adjacent(&self, other: Coord) -> bool {
        other == self.north() || other == self.south() ||
            other == self.east() || other == self.west()
    }

    pub fn diagonal(&self, other: Coord) -> bool {
        other == self.northeast() || other == self.northwest() ||
            other == self.southeast() || other == self.southwest()
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.0, self.1)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        xy(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for Coord {
    type Output = Coord;

    fn mul(self, rhs: Self) -> Self::Output {
        xy(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Coord {
    type Output = Coord;

    fn div(self, rhs: Self) -> Self::Output {
        xy(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        xy(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<i32> for Coord {
    type Output = Coord;

    fn div(self, rhs: i32) -> Self::Output {
        xy(self.0 / rhs, self.1 / rhs)
    }
}

// /// A pixel coordinate: only makes sense on the screen buffer
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct PixelCoord(pub u32, pub u32);
// pub fn pc(x: u32, y: u32) -> PixelCoord {
//     PixelCoord(x, y)
// }
//
// impl From<&Coord> for PixelCoord {
//     fn from(value: &Coord) -> Self {
//         Self(value.0 as u32 * 24, value.1 as u32 * 24)
//     }
// }
//
// impl From<Coord> for PixelCoord {
//     fn from(value: Coord) -> Self {
//         Self(value.0 as u32 * 24, value.1 as u32 * 24)
//     }
// }
//
// impl Add for PixelCoord {
//     type Output = PixelCoord;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         pc(self.0 + rhs.0, self.1 + rhs.1)
//     }
// }

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct PixelRect {
//     pub pos: PixelCoord,
//     pub size: PixelCoord
// }
//
// impl PixelRect {
//     pub fn right(&self) -> u32 {
//         self.pos.0 + self.size.0
//     }
//
//     pub fn bottom(&self) -> u32 {
//         self.pos.1 + self.size.1
//     }
//
//     pub fn contains(&self, pos: PixelCoord) -> bool {
//         pos.0 >= self.pos.0 && pos.1 >= self.pos.1 &&
//             pos.0 < self.pos.0 + self.size.0 &&
//             pos.1 < self.pos.1 + self.size.1
//     }
//
//     pub fn translate(self, offset: PixelCoord) -> Self {
//         Self {
//             pos: self.pos + offset,
//             size: self.size
//         }
//     }
// }