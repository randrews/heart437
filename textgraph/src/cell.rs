use std::ops::{Add, AddAssign, BitOr, BitOrAssign};
use crate::{CLEAR, Color, WHITE, Grid};

/// A cell's contents.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    /// The character to display
    pub ch: u8,
    /// The foreground, used tor white pixels in the bitmap
    pub fg: Color,
    /// the background, used for black pixels in the bitmap
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Fg(WHITE) + Bg(CLEAR) + Char(' ' as u8)
    }
}

macro_rules! apply_fields {
    ($t:ty { $($tfield:tt => $cfield:ident),+ }) => {
        impl BitOrAssign<$t> for Cell {
            fn bitor_assign(&mut self, rhs: $t) {
                $(
                self.$cfield = rhs.$tfield;
                )+
            }
        }
    };
}

macro_rules! into_properties {
    ($parent:ty, $t:ty => { $field:tt }) => {
        impl From<$parent> for $t {
            fn from(value: $parent) -> Self { Self(value.$field) }
        }
    };

    ($parent:ty, $t:ty => { $( $field:ident ),+ }) => {
        impl From<$parent> for $t {
            fn from(value: $parent) -> Self { Self { $( $field: value.$field, )+ } }
        }
    };
}

macro_rules! property_sum {
    ($a:ty { $($afield:tt => $sumafield:ident),+ }, $b:ty { $($bfield:tt => $sumbfield:ident),+ } => $sum:ty ) => {
        impl Add<$a> for $b {
            type Output = $sum;
            fn add(self, rhs: $a) -> Self::Output {
                Self::Output {
                    $($sumafield: rhs.$afield,)+
                    $($sumbfield: self.$bfield,)+
                }
            }
        }

        impl Add<$b> for $a {
            type Output = $sum;
            fn add(self, rhs: $b) -> Self::Output {
                Self::Output {
                    $($sumafield: self.$afield,)+
                    $($sumbfield: rhs.$bfield,)+
                }
            }
        }
    };
}

/// To create a cell, you can instantiate the struct, but it's more common to construct one
/// from these component structs: `Fg`, `Bg`, and `Char`. These can me added together in various
/// combinations to make a `Cell`:
/// ```
/// # use textgraph::*;
/// let player = Fg(WHITE) + Bg(BLUE) + Char('@' as u8);
/// let inverted = Fg(BLACK) + Bg(RED); // An FgBg
/// let wall = Char('#' as u8) + inverted; // Which can later be used in a Cell
/// ```
/// You can also deconstruct a `Cell` with `From`:
/// ```
/// # use textgraph::*;
/// let player = Fg(WHITE) + Bg(BLUE) + Char('@' as u8);
/// let player_color = FgBg::from(player);
/// ```
/// A `Cell` can be modified with the `|=` operator, to apply changes from component structs:
/// ```
/// # use textgraph::*;
/// let mut player = Fg(WHITE) + Bg(BLUE) + Char('@' as u8);
/// player |= Fg(BLACK) + Bg(RED); // Change its color
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fg(pub Color);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bg(pub Color);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Char(pub u8);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FgBg { fg: Color, bg: Color }
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FgChar { fg: Color, ch: u8 }
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BgChar { bg: Color, ch: u8 }

property_sum!(Fg { 0 => fg }, Bg { 0 => bg } => FgBg);
property_sum!(Fg { 0 => fg }, Char { 0 => ch } => FgChar);
property_sum!(Bg { 0 => bg }, Char { 0 => ch } => BgChar);
property_sum!(Fg { 0 => fg }, BgChar { bg => bg, ch => ch } => Cell);
property_sum!(Bg { 0 => bg }, FgChar { fg => fg, ch => ch } => Cell);
property_sum!(Char { 0 => ch }, FgBg { fg => fg, bg => bg } => Cell);
apply_fields!(Fg { 0 => fg });
apply_fields!(Bg { 0 => bg });
apply_fields!(Char { 0 => ch });
apply_fields!(FgBg { fg => fg, bg => bg });
apply_fields!(FgChar { fg => fg, ch => ch });
apply_fields!(BgChar { bg => bg, ch => ch });
apply_fields!(Cell { fg => fg, bg => bg, ch => ch });
into_properties!(Cell, Fg => { fg });
into_properties!(Cell, Bg => { bg });
into_properties!(Cell, Char => { ch });
into_properties!(Cell, FgBg => { fg, bg });
into_properties!(Cell, FgChar => { fg, ch });
into_properties!(Cell, BgChar => { bg, ch });
into_properties!(FgBg, Fg => { fg });
into_properties!(FgBg, Bg => { bg });
into_properties!(FgChar, Fg => { fg });
into_properties!(FgChar, Char => { ch });
into_properties!(BgChar, Bg => { bg });
into_properties!(BgChar, Char => { ch });

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
        assert_eq!(f + b, FgBg { fg: RED, bg: WHITE });
        assert_eq!(f + b + ch, (f + ch) + b);
        assert_eq!(f + b + ch, Cell { fg: RED, bg: WHITE, ch: 65u8 });
    }

    #[test]
    fn test_bit_assign() {
        let mut a = Fg(RED) + Bg(BLUE) + Char(0x32u8);
        a |= Char(65);
        a |= Bg(YELLOW);
        assert_eq!(a, Fg(RED) + Bg(YELLOW) + Char(65));

        let mut a = Fg(RED) + Bg(BLUE) + Char(0x32u8);
        a |= Char(65) + Bg(YELLOW);
        assert_eq!(a, Fg(RED) + Bg(YELLOW) + Char(65))
    }

    #[test]
    fn test_into() {
        let c = Fg(RED) + Bg(WHITE) + Char(0x65u8);
        assert_eq!(Char::from(c), Char(0x65u8));
        assert_eq!(FgBg::from(c), Fg(RED) + Bg(WHITE));

        let col = Fg(RED) + Bg(WHITE);
        assert_eq!(Bg::from(col), Bg(WHITE));
    }
}
