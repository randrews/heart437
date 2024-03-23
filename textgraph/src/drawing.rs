use crate::{Color, Coord, OptionCell, xy};
use crate::layer::*;

/// A Canvas is anything that we can set a cell on: anything that lets us put an optionally-colored
/// ASCII char into a spot on a grid. `Layer` is a canvas.
pub trait Canvas {
    /// Set a cell, optionally set the color as well
    fn set(&mut self, at: Coord, ch: Option<char>, fg: Option<Color>, bg: Option<Color>);

    /// Return the size of the grid
    fn size(&self) -> Coord;

    /// Return whether a given point is within a grid: uses `size` to determine
    fn within(&self, point: Coord) -> bool {
        let Coord(x, y) = point;
        let Coord(xmax, ymax) = self.size();
        x < xmax && y < ymax
    }

    /// Fill a rectangle with a single character. The rectangle is clipped to the region of the canvas.
    /// ```
    /// # use textgraph::*;
    /// # let font = Font::default();
    /// let mut layer = Layer::new(&font, xy(10, 10), pxy(0, 0), pxy(0, 0));
    /// layer.fill_rect(Some('.'), Some(WHITE), None, xy(1, 1), xy(3, 3));
    /// ```
    fn fill_rect(&mut self, ch: Option<char>, fg: Option<Color>, bg: Option<Color>, pos: Coord, size: Coord) {
        for y in pos.1 .. (pos.1 + size.1) {
            for x in pos.0 .. (pos.0 + size.0) {
                if self.within(xy(x, y)) {
                    self.set(xy(x, y), ch, fg, bg)
                }
            }
        }
    }

    /// Fill with a given char / color
    fn fill(&mut self, ch: Option<char>, fg: Option<Color>, bg: Option<Color>) {
        self.fill_rect(ch, fg, bg, xy(0, 0), self.size())
    }

    /// Draw the outline of a rectangle, clipped to the region of the canvas
    /// Rectangles can be drawn in several styles, see `RectStyle`.
    fn rect(&mut self, wall: Wall, fg: Option<Color>, bg: Option<Color>, pos: Coord, size: Coord) {
        self.set(pos, Some(wall.nw as char), fg, bg);
        self.set(xy(pos.0 + size.0 - 1, pos.1), Some(wall.ne as char), fg, bg);
        self.set(xy(pos.0, pos.1 + size.1 - 1), Some(wall.sw as char), fg, bg);
        self.set(xy(pos.0 + size.0 - 1, pos.1 + size.1 - 1), Some(wall.se as char), fg, bg);
        for x in (pos.0 + 1) .. (pos.0 + size.0 - 1) {
            self.set(xy(x, pos.1), Some(wall.n as char), fg, bg);
            self.set(xy(x, pos.1 + size.1 - 1), Some(wall.s as char), fg, bg);
        }

        for y in (pos.1 + 1) .. (pos.1 + size.1 - 1) {
            self.set(xy(pos.0, y), Some(wall.w as char), fg, bg);
            self.set(xy(pos.0 + size.0 - 1, y), Some(wall.e as char), fg, bg);
        }
    }
}

impl Canvas for Layer<'_> {
    fn set(&mut self, at: Coord, ch: Option<char>, fg: Option<Color>, bg: Option<Color>) {
        self.set(at, OptionCell { ch: ch.map(|c| c as u8), fg, bg })
    }

    fn size(&self) -> Coord {
        self.size()
    }
}

/// Styles of ASCII rectangles:
pub enum RectStyle {
    /// Normal rectangles use the +, -, and | characters:
    /// ```text
    ///   +-----+
    ///   |     |
    ///   |     |
    ///   +-----+
    /// ```
    NORMAL,

    /// Rounded rectangles use slashes to "round" the corners:
    /// ```text
    ///   /----\
    ///   |    |
    ///   \----/
    /// ```
    ROUNDED,

    /// Single-bar rectangles use extended ASCII chars 0xbf, 0xc0, 0xd9, etc (which are single-line
    /// walls in code page 437)
    SINGLE,

    /// Double-bar rectangles use extended ASCII chars 0xbb, 0xbc, 0xc8, etc (which are double-line
    /// walls in code page 437)
    DOUBLE,
}

/// The 8 characters (4 sides and 4 corners) making up a rectangle border
pub struct Wall {
    nw: u8, n: u8, ne: u8,
    w: u8, e: u8,
    sw: u8, s: u8, se: u8
}

impl RectStyle {
    /// Return the characters needed for a given RectStyle to draw a rectangular border in that
    /// style
    /// ```
    /// # use textgraph::*;
    /// RectStyle::DOUBLE.wall();
    pub fn wall(self) -> Wall {
        match self {
            RectStyle::NORMAL => Wall {
                nw: '+' as u8,
                n: '-' as u8,
                ne: '+' as u8,
                w: '|' as u8,
                e: '|' as u8,
                sw: '+' as u8,
                s: '-' as u8,
                se: '+' as u8,
            },
            RectStyle::ROUNDED => Wall {
                nw: '/' as u8,
                n: '-' as u8,
                ne: '\\' as u8,
                w: '|' as u8,
                e: '|' as u8,
                sw: '\\' as u8,
                s: '-' as u8,
                se: '/' as u8,
            },
            RectStyle::SINGLE => Wall {
                nw: 0xda,
                n: 0xc4,
                ne: 0xbf,
                w: 0xb3,
                e: 0xb3,
                sw: 0xc0,
                s: 0xc4,
                se: 0xd9,
            },
            RectStyle::DOUBLE => Wall {
                nw: 0xc9,
                n: 0xcd,
                ne: 0xbb,
                w: 0xba,
                e: 0xba,
                sw: 0xc8,
                s: 0xcd,
                se: 0xbc,
            },
        }
    }
}