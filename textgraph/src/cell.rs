use std::ops::{Add, AddAssign, BitOr, BitOrAssign};
use crate::{CLEAR, Color, WHITE};

/// A cell's contents, used with `get` and `set`. Each cell contains
/// all three fields, but when passed into `set`, a None will cause that
/// field not to be set.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OptionCell {
    pub ch: Option<u8>,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl From<Cell> for OptionCell {
    fn from(value: Cell) -> Self {
        Self {
            ch: Some(value.ch),
            fg: Some(value.fg),
            bg: Some(value.bg),
        }
    }
}

impl From<&Cell> for OptionCell {
    fn from(value: &Cell) -> Self {
        Self {
            ch: Some(value.ch),
            fg: Some(value.fg),
            bg: Some(value.bg)
        }
    }
}

impl Default for OptionCell {
    fn default() -> Self {
        Self {
            ch: None,
            fg: None,
            bg: None,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

/// A cell's contents
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub ch: u8,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ' as u8,
            fg: WHITE,
            bg: CLEAR
        }
    }
}

// Single tuples
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fg(Color);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bg(Color);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Char(u8);

macro_rules! reverse_add {
    ($lhs:ty, $rhs:ty) => {
        impl Add<$rhs> for $lhs {
            type Output = OptionCell;
            fn add(self, rhs: $rhs) -> Self::Output { rhs + self }
        }
    };
}

macro_rules! addish_or {
    ($lhs:ty, $rhs:ty) => {
        impl BitOr<$rhs> for $lhs {
            type Output = OptionCell;
            fn bitor(self, rhs: $rhs) -> Self::Output { self + rhs }
        }

        impl BitOr<$lhs> for $rhs {
            type Output = OptionCell;
            fn bitor(self, rhs: $lhs) -> Self::Output { self + rhs }
        }
    };
}

macro_rules! reflexive_or {
    ($lhs:ty) => {
        impl BitOr<$lhs> for $lhs {
            type Output = $lhs;
            fn bitor(self, rhs: $lhs) -> Self::Output { rhs }
        }
    };
}

// Adding any two (different) tuples makes an optioncell:
impl Add<Bg> for Fg {
    type Output = OptionCell;
    fn add(self, rhs: Bg) -> Self::Output { OptionCell { fg: Some(self.0), bg: Some(rhs.0), ch: None } }
}

impl Add<Char> for Fg {
    type Output = OptionCell;
    fn add(self, rhs: Char) -> Self::Output { OptionCell { fg: Some(self.0), bg: None, ch: Some(rhs.0) } }
}

impl Add<Char> for Bg {
    type Output = OptionCell;
    fn add(self, rhs: Char) -> Self::Output { OptionCell { fg: None, bg: Some(self.0), ch: Some(rhs.0) } }
}

// Adding things, the left hand side takes precedence
impl Add<Fg> for OptionCell {
    type Output = OptionCell;
    fn add(self, rhs: Fg) -> Self::Output { OptionCell { fg: self.fg.or(Some(rhs.0)), bg: self.bg, ch: self.ch } }
}
impl Add<Bg> for OptionCell {
    type Output = OptionCell;
    fn add(self, rhs: Bg) -> Self::Output { OptionCell { fg: self.fg, bg: self.bg.or(Some(rhs.0)), ch: self.ch } }
}
impl Add<Char> for OptionCell {
    type Output = OptionCell;
    fn add(self, rhs: Char) -> Self::Output { OptionCell { fg: self.fg, bg: self.bg, ch: self.ch.or(Some(rhs.0)) } }
}

// OR-ing things, the right hand side takes precedence
impl BitOr<Fg> for OptionCell {
    type Output = OptionCell;
    fn bitor(self, rhs: Fg) -> Self::Output { OptionCell { fg: Some(rhs.0), bg: self.bg, ch: self.ch } }
}
impl BitOr<Bg> for OptionCell {
    type Output = OptionCell;
    fn bitor(self, rhs: Bg) -> Self::Output { OptionCell { fg: self.fg, bg: Some(rhs.0), ch: self.ch } }
}
impl BitOr<Char> for OptionCell {
    type Output = OptionCell;
    fn bitor(self, rhs: Char) -> Self::Output { OptionCell { fg: self.fg, bg: self.bg, ch: Some(rhs.0) } }
}

// Adding two optioncells has the lhs take precedence:
impl Add<OptionCell> for OptionCell {
    type Output = OptionCell;
    fn add(self, rhs: OptionCell) -> Self::Output {
        OptionCell {
            fg: self.fg.or(rhs.fg),
            bg: self.bg.or(rhs.bg),
            ch: self.ch.or(rhs.ch)
        }
    }
}

// OR-ing two optioncells has the lhs take precedence:
impl BitOr<OptionCell> for OptionCell {
    type Output = OptionCell;
    fn bitor(self, rhs: OptionCell) -> Self::Output {
        OptionCell {
            fg: rhs.fg.or(self.fg),
            bg: rhs.bg.or(self.bg),
            ch: rhs.ch.or(self.ch)
        }
    }
}

impl BitOrAssign<OptionCell> for OptionCell {
    fn bitor_assign(&mut self, rhs: OptionCell) {
        self.fg = rhs.fg.or(self.fg);
        self.bg = rhs.bg.or(self.bg);
        self.ch = rhs.ch.or(self.ch);
    }
}

impl BitOrAssign<Fg> for OptionCell {
    fn bitor_assign(&mut self, rhs: Fg) { self.fg = Some(rhs.0) }
}
impl BitOrAssign<Bg> for OptionCell {
    fn bitor_assign(&mut self, rhs: Bg) { self.bg = Some(rhs.0) }
}
impl BitOrAssign<Char> for OptionCell {
    fn bitor_assign(&mut self, rhs: Char) { self.ch = Some(rhs.0) }
}

impl AddAssign<OptionCell> for OptionCell {
    fn add_assign(&mut self, rhs: OptionCell) {
        self.fg = self.fg.or(rhs.fg);
        self.bg = self.bg.or(rhs.bg);
        self.ch = self.ch.or(rhs.ch);
    }
}

impl AddAssign<Fg> for OptionCell {
    fn add_assign(&mut self, rhs: Fg) { self.fg = self.fg.or(Some(rhs.0)) }
}
impl AddAssign<Bg> for OptionCell {
    fn add_assign(&mut self, rhs: Bg) { self.bg = self.bg.or(Some(rhs.0)) }
}
impl AddAssign<Char> for OptionCell {
    fn add_assign(&mut self, rhs: Char) { self.ch = self.ch.or(Some(rhs.0)) }
}

// The single-tuples, adding or or-ing them together, the order doesn't matter, so
// provide reversed impls that just call the originals:
reverse_add!(Char, Fg);
reverse_add!(Char, Bg);
reverse_add!(Bg, Fg);

// Single tuples can't overlap, so or-ing them is the same as adding them
addish_or!(Char, Fg);
addish_or!(Char, Bg);
addish_or!(Fg, Bg);

// Adding two single-tuples isn't allowed but OR-ing them is, and makes the rhs take precedence:
reflexive_or!(Char);
reflexive_or!(Fg);
reflexive_or!(Bg);

#[cfg(test)]
mod test {
    use crate::{BLUE, RED, YELLOW};
    use super::*;

    #[test]
    fn test_adds() {
        let f = Fg(RED);
        let b = Bg(WHITE);
        let ch = Char(65);

        assert_eq!(f + b, b + f);
        assert_eq!(f + b, OptionCell{ fg: Some(RED), bg: Some(WHITE), ch: None });
        assert_eq!(f + b + ch, (f + ch) + b);
        assert_eq!(f + b + ch, OptionCell { fg: Some(RED), bg: Some(WHITE), ch: Some(65u8) });
    }

    #[test]
    fn test_ors() {
        let f = Fg(RED);
        let f2 = Fg(BLUE);
        let b = Bg(WHITE);
        let ch = Char(65);

        assert_eq!(f | b, f + b);
        assert_eq!(f | f2, f2);
        assert_eq!((f + b) | f2, f2 + b);
    }

    #[test]
    fn test_bit_assign() {
        let mut a = Fg(RED) + Bg(BLUE);
        a |= Char(65);
        a |= Bg(YELLOW);
        assert_eq!(a, Fg(RED) + Bg(YELLOW) + Char(65))
    }

    #[test]
    fn test_add_assign() {
        let mut a = Fg(RED) + Bg(BLUE);
        a += Char(65);
        a += Bg(YELLOW);
        assert_eq!(a, Fg(RED) + Bg(BLUE) + Char(65))
    }
}

// Pairs
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct FgBg{ fg: Color, bg: Color }
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct FgChar { fg: Color, ch: u8 }
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct BgChar { bg: Color, ch: u8 }

// macro_rules! combine_add {
//     ($lhs:ty, $rhs:ty, $sum:ty, $body:expr) => {
//         impl Add<$rhs> for $lhs {
//             type Output = $sum;
//             fn add(self, rhs: $rhs) -> Self::Output {
//                 let body = $body;
//                 body(self, rhs)
//             }
//         }
//
//         impl Add<$lhs> for $rhs {
//             type Output = $sum;
//             fn add(self, rhs: $lhs) -> Self::Output { rhs + self }
//         }
//     };
// }
//
// // Adding singles together, each orientation:
// combine_add!(Bg, Fg, FgBg, |lhs: Bg, rhs: Fg| -> FgBg { FgBg { fg: rhs.0, bg: lhs.0 } });
// combine_add!(Char, Fg, FgChar, |lhs: Char, rhs: Fg| -> FgChar { FgChar { fg: rhs.0, ch: lhs.0 } });
// combine_add!(Char, Bg, BgChar, |lhs: Char, rhs: Bg| -> BgChar { BgChar { bg: rhs.0, ch: lhs.0 } });
//
// // Adding the third thing to a pair, each orientation
// combine_add!(Char, FgBg, Cell, |lhs: Char, rhs: FgBg| -> Cell { Cell { fg: rhs.fg, bg: rhs.bg, ch: lhs.0 } });
// combine_add!(Fg, BgChar, Cell, |lhs: Fg, rhs: BgChar| -> Cell { Cell { fg: lhs.0, bg: rhs.bg, ch: rhs.ch } });
// combine_add!(Bg, FgChar, Cell, |lhs: Bg, rhs: FgChar| -> Cell { Cell { fg: rhs.fg, bg: lhs.0, ch: rhs.ch } });
//
// macro_rules! combine_or {
//     ($lhs:ty, $rhs:ty, $sum:ty, $body:expr) => {
//         impl BitOr<$rhs> for $lhs {
//             type Output = $sum;
//             fn bitor(self, rhs: $rhs) -> Self::Output {
//                 $body(self, rhs)
//             }
//         }
//
//         impl BitOr<$lhs> for $rhs {
//             type Output = $sum;
//             fn bitor(self, rhs: $lhs) -> Self::Output { rhs | self }
//         }
//     };
// }
//
// // OR-ing singles into cells
// impl BitOr<Fg> for Cell {
//     type Output = Cell;
//     fn bitor(self, rhs: Fg) -> Self::Output { Cell { fg: rhs.0, bg: self.bg, ch: self.ch } }
// }
// impl BitOr<Bg> for Cell {
//     type Output = Cell;
//     fn bitor(self, rhs: Bg) -> Self::Output { Cell { fg: self.fg, bg: rhs.0, ch: self.ch } }
// }
// impl BitOr<Char> for Cell {
//     type Output = Cell;
//     fn bitor(self, rhs: Char) -> Self::Output { Cell { fg: self.fg, bg: self.bg, ch: rhs.0 } }
// }
//
// // OR-ing pairs into cells
// impl BitOr<FgBg> for Cell {
//     type Output = Cell;
//     fn bitor(self, rhs: FgBg) -> Self::Output { Cell { fg: rhs.fg, bg: rhs.bg, ch: self.ch } }
// }
// impl BitOr<FgChar> for Cell {
//     type Output = Cell;
//     fn bitor(self, rhs: FgChar) -> Self::Output { Cell { fg: rhs.fg, bg: self.bg, ch: rhs.ch } }
// }
// impl BitOr<BgChar> for Cell {
//     type Output = Cell;
//     fn bitor(self, rhs: BgChar) -> Self::Output { Cell { fg: self.fg, bg: rhs.bg, ch: rhs.ch } }
// }
//
// // OR-ing singles into pairs
// impl BitOr<Fg> for FgBg {
//     type Output = FgBg;
//     fn bitor(self, rhs: Fg) -> Self::Output { FgBg { fg: rhs.fg, bg: self.bg } }
// }
//
//
// macro_rules! overwrite_or {
//     ($lhs:ty, $rhs:ty) => {
//         impl BitOr<$rhs> for $lhs {
//             type Output = $rhs;
//             fn bitor(self, rhs: $rhs) -> Self::Output { rhs }
//         }
//     };
// }
//
// // OR-ing overwrites the left hand stuff with the right hand stuff. In cases where the
// // lhs is a subset of the rhs...
// overwrite_or!(Fg, FgBg);
// overwrite_or!(Fg, FgChar);
// overwrite_or!(Fg, Cell);
// overwrite_or!(Bg, Cell);
// overwrite_or!(Bg, BgChar);
// overwrite_or!(Bg, FgBg);
// overwrite_or!(Char, FgChar);
// overwrite_or!(Char, BgChar);
// overwrite_or!(Char, Cell);
// overwrite_or!(FgChar, Cell);
// overwrite_or!(BgChar, Cell);
// overwrite_or!(FgBg, Cell);
//
// // OR-ing pairs into cells
// //combine_or!(Cell, FgChar, Cell, |lhs: Cell, rhs: FgChar| -> Cell { Cell { fg: rhs.fg, ch: rhs.ch, bg: lhs.bg } });
// //combine_or!(Cell, BgChar, Cell, |lhs: Cell, rhs: BgChar| -> Cell { Cell { fg: lhs.fg, bg: rhs.bg, ch: rhs.ch } });
// //combine_or!(Cell, FgBg, Cell, |lhs: Cell, rhs: FgBg| -> Cell { Cell { fg: rhs.fg, bg: rhs.bg, ch: lhs.ch } });
//
// macro_rules! addish_or {
//     ($lhs:ty, $rhs:ty, $sum:ty) => {
//         impl BitOr<$rhs> for $lhs {
//             type Output = $sum;
//             fn bitor(self, rhs: $rhs) -> Self::Output { self + rhs }
//         }
//
//         impl BitOr<$lhs> for $rhs {
//             type Output = $sum;
//             fn bitor(self, rhs: $lhs) -> Self::Output { self + rhs }
//         }
//     };
// }
//
// // OR-ing complementary things is just an add:
// addish_or!(Fg, Bg, FgBg);
// addish_or!(Fg, Char, FgChar);
// addish_or!(Bg, Char, BgChar);
// addish_or!(Bg, FgChar, Cell);
// addish_or!(Fg, BgChar, Cell);
// addish_or!(Char, FgBg, Cell);
//
// // OR-ing pairs together:
// impl BitOr<FgChar> for FgBg {
//     type Output = Cell;
//     fn bitor(self, rhs: FgChar) -> Self::Output { Cell { fg: rhs.fg, ch: rhs.ch, bg: self.bg } }
// }
// impl BitOr<BgChar> for FgBg {
//     type Output = Cell;
//     fn bitor(self, rhs: BgChar) -> Cell { Cell { bg: rhs.bg, ch: rhs.ch, fg: self.fg } }
// }
// impl BitOr<FgBg> for FgChar {
//     type Output = Cell;
//     fn bitor(self, rhs: FgBg) -> Cell { Cell { fg: rhs.fg, bg: rhs.bg, ch: self.ch } }
// }
// impl BitOr<BgChar> for FgChar {
//     type Output = Cell;
//     fn bitor(self, rhs: BgChar) -> Cell { Cell { bg: rhs.bg, ch: rhs.ch, fg: self.fg } }
// }
// impl BitOr<FgBg> for BgChar {
//     type Output = Cell;
//     fn bitor(self, rhs: FgBg) -> Cell { Cell { fg: rhs.fg, bg: rhs.bg, ch: self.ch } }
// }
// impl BitOr<FgChar> for BgChar {
//     type Output = Cell;
//     fn bitor(self, rhs: FgChar) -> Cell { Cell { fg: rhs.fg, ch: rhs.ch, bg: self.bg } }
// }
//
// //OR-ing singles with themselves (not macroable, obviously)
// impl BitOr<Fg> for Fg {
//     type Output = Fg; fn bitor(self, _rhs: Fg) -> Self::Output { self }
// }
// impl BitOr<Bg> for Bg {
//     type Output = Bg; fn bitor(self, _rhs: Bg) -> Self::Output { self }
// }
// impl BitOr<Char> for Char {
//     type Output = Char; fn bitor(self, _rhs: Char) -> Self::Output { self }
// }
//
// // OR-ing pairs with themselves
// impl BitOr<FgBg> for FgBg {
//     type Output = FgBg; fn bitor(self, _rhs: FgBg) -> Self::Output { self }
// }
// impl BitOr<BgChar> for BgChar {
//     type Output = BgChar; fn bitor(self, _rhs: BgChar) -> Self::Output { self }
// }
// impl BitOr<FgChar> for FgChar {
//     type Output = FgChar; fn bitor(self, _rhs: FgChar) -> Self::Output { self }
// }
//
// // OR-ing two cells together
// impl BitOr<Cell> for Cell {
//     type Output = Cell;
//     fn bitor(self, rhs: Cell) -> Self::Output { rhs }
// }
