use std::ops::Index;
use crate::color::{CLEAR, Color};
use crate::font::{Font, Glyph};
use crate::{Coord, Grid, pxy, WHITE, xy};
use crate::coords::PixelCoord;

/// Represents a rectangular grid of colored glyphs.
/// - size (in characters), the character dimensions of the layer
/// - position (in pixels),
/// Borrows a font to know which glyphs to draw.
#[derive(Clone)]
pub struct Layer<'a> {
    /// The font to display the layer in
    pub font: &'a Font,

    /// How much to scale the glyphs, enlarging only. `pxy(1, 2)` will double the height to 8x16
    pub scale: PixelCoord,

    /// Where to place the layer in the target texture
    pub origin: PixelCoord,

    /// The characters to draw. Each of these can be replaced with another grid as long as it's
    /// the same size.
    pub chars: Grid<u8>,

    /// The foreground colors of the cells
    pub fg: Grid<Color>,

    /// The background colors of the cells
    pub bg: Grid<Color>,
}

impl<'a> Layer<'a> {
    /// The normal way to create a Layer.
    /// ```
    /// # use textgraph::*;
    /// let font = Font::default();
    /// let layer = Layer::new(&font, xy(80, 25), pxy(1, 1), pxy(0, 0));
    /// ```
    pub fn new(font: &'a Font, size: Coord, scale: PixelCoord, origin: PixelCoord) -> Self {
        let chars = Grid::new(xy(size.0 as i32, size.1 as i32), ' ' as u8);
        let fg = Grid::new(xy(size.0 as i32, size.1 as i32), WHITE);
        let bg = Grid::new(xy(size.0 as i32, size.1 as i32), CLEAR);
        Self {
            font,
            scale,
            origin,
            chars,
            fg,
            bg
        }
    }

    /// Returns the size of this Layer
    pub fn size(&self) -> Coord {
        self.chars.dimensions().into()
    }

    /// Returns an iterator used to iterate over all the cells in the layer:
    ///```
    /// # use textgraph::*;
    /// # let font = Font::default();
    /// # let layer = Layer::new(&font, xy(10, 10), pxy(1, 1), pxy(0, 0));
    /// for (glyph, fg, bg, PixelCoord(x, y)) in layer.cells() {
    ///   // Draw each glyph in its colors here, at pixel coordinates (x, y)
    /// }
    /// ```
    pub fn cells(&self) -> CharIterator {
        CharIterator {
            layer: &self,
            n: 0
        }
    }

    /// Sets a particular character and color in this grid
    /// ```
    /// # use textgraph::*;
    /// # let font = Font::default();
    /// # let mut layer = Layer::new(&font, xy(10, 10), pxy(1, 1), pxy(0, 0));
    /// layer.set(xy(3, 3), Cell { ch: Some('A' as u8), fg: Some(WHITE), bg: Some(CLEAR) });
    /// ```
    pub fn set(&mut self, at: Coord, cell: Cell) {
        let at: Coord = at.into();
        let Cell { ch, fg, bg} = cell;
        ch.map(|ch| self.chars[at] = ch as u8);
        fg.map(|fg| self.fg[at] = fg);
        bg.map(|bg| self.bg[at] = bg);
    }

    /// Get the contents of a cell as a struct
    /// ```
    /// use textgraph::*;
    /// # let font = Font::default();
    /// # let mut layer = Layer::new(&font, xy(10, 10), pxy(2, 4), pxy(50, 50));
    /// layer.get(xy(2, 3)).ch;
    /// ```
    pub fn get(&self, loc: Coord) -> Cell {
        Cell {
            ch: Some(self.chars[loc]),
            fg: Some(self.fg[loc]),
            bg: Some(self.bg[loc]),
        }
    }

    /// Get the contents of a cell as a struct, mutably
    /// ```
    /// use textgraph::*;
    /// # let font = Font::default();
    /// # let mut layer = Layer::new(&font, xy(10, 10), pxy(2, 4), pxy(50, 50));
    /// *layer.get_mut(xy(2, 3)).fg = RED;
    /// ```
    pub fn get_mut(&mut self, loc: Coord) -> MutCell {
        MutCell {
            ch: &mut self.chars[loc],
            fg: &mut self.fg[loc],
            bg: &mut self.bg[loc],
        }
    }
}

/// A cell's contents, used with `get` and `set`. Each cell contains
/// all three fields, but when passed into `set`, a None will cause that
/// field not to be set.
pub struct Cell {
    pub ch: Option<u8>,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

/// A mutable borrow of the three fields of a cell, gotten with `get_mut`
pub struct MutCell<'a> {
    pub ch: &'a mut u8,
    pub fg: &'a mut Color,
    pub bg: &'a mut Color,
}

impl Into<Cell> for MutCell {
    fn into(self) -> Cell {
        Cell {
            ch: Some(*self.ch),
            fg: Some(*self.fg),
            bg: Some(*self.bg),
        }
    }
}

/// Iterator over the characters of a `Layer`
/// Usually created through `Layer::cells`
pub struct CharIterator<'a> {
    layer: &'a Layer<'a>,
    n: usize
}

impl Iterator for CharIterator<'_> {
    type Item = (Glyph, Color, Color, PixelCoord);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        self.n += 1;
        let coord = self.layer.chars.coord(n);
        if let Some(ch) = self.layer.chars.get(coord) {
            let fg = self.layer.fg[coord];
            let bg = self.layer.bg[coord];
            let glyph = self.layer.font[*ch];
            let (scalex, scaley) = (self.layer.scale.0.max(1), self.layer.scale.1.max(1));
            let n = n as i32;
            let width = self.layer.chars.dimensions().0;
            let px = n % width * 8 * scalex + self.layer.origin.0;
            let py = n / width * 8 * scaley + self.layer.origin.1;
            Some((glyph, fg, bg, pxy(px, py)))
        } else {
            None
        }
    }
}

/// Represents the capability of drawing oneself to an array of RGBA pixels
/// The `pixels` argument is a mutable borrow of pixels (four u8s, RGBA order)
/// in a rectangle `width` pixels wide. Drawing will be clipped to the actual
/// size of the array
pub trait Drawable {
    fn draw(&self, pixels: &mut [u8], width: usize);
}

impl Drawable for Layer<'_> {
    /// Draws the Layer into the frame at its pixel position:
    /// ```
    /// # use textgraph::*;
    /// # let font = Font::default();
    /// let mut layer = Layer::new(&font, Coord(10, 10), PixelCoord(1, 1), PixelCoord(25, 25));
    /// layer.fill(Some('R'), Some(WHITE), Some(BLUE));
    /// let mut buf = [0u8; (640 * 480 * 4)];
    /// layer.draw(&mut buf, 640);
    /// ```
    fn draw(&self, pixels: &mut [u8], width: usize) {
        let (xscale, yscale) = (self.scale.0.max(1), self.scale.1.max(1));
        let height = (pixels.len() / 4) / width; // Height of the pixel buffer in pixels

        for (glyph, fg, bg, PixelCoord(x, y)) in self.cells() {
            if x >= width as i32 || y >= height as i32 { continue }
            let (right, bottom) = (x + xscale * 8, y + yscale * 8);
            if right < 0 || bottom < 0 { continue }

            for (color, xo, yo) in &glyph {
                // Scaling is like drawing a tiny rectangle instead of a single pixel, for each dot:
                for sy in 0..yscale {
                    for sx in 0..xscale {
                        // Pixel coords of the current pixel:
                        let (px, py) = (x + xscale * xo as i32 + sx, y + yscale * yo as i32 + sy);

                        // If in bounds:
                        if px < width as i32 && py < height as i32 && px >= 0 && py >= 0 {
                            let (px, py) = (px as usize, py as usize);
                            let start = px * 4 + py * width * 4; // byte addr of start of pixel
                            let current = &mut pixels[start .. (start + 4)];
                            let new = (if color { fg } else { bg }).blend_into(current);
                            for n in 0..4 { current[n] = new[n] }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::color::YELLOW;
    use crate::RED;
    use super::*;

    #[test]
    fn test_layer_creation() {
        let font = Font::default();
        let layer = Layer::new(&font, xy(10, 10), pxy(1, 2), pxy(0, 0));
        assert_eq!(layer.scale, pxy(1, 2));
        assert_eq!(layer.origin, pxy(0, 0));
        assert_eq!(layer.size(), xy(10, 10));
    }

    #[test]
    fn test_layer_access() {
        let font = Font::default();
        let mut layer = Layer::new(&font, xy(10, 10), pxy(1, 2), pxy(0, 0));

        let at: Coord = xy(3, 2);
        layer.chars[at] = '!' as u8;
        layer.fg[at] = YELLOW;
        layer.bg[at] = RED;

        assert_eq!(layer.chars[at], '!' as u8);
        assert_eq!(layer.fg[at], YELLOW);
        assert_eq!(layer.bg[at], RED);
    }

    #[test]
    fn test_pixel_coords() {
        let font = Font::default();
        let layer = Layer::new(&font, xy(10, 10), pxy(2, 4), pxy(50, 50));

        let mut it = layer.cells();
        // This places us on the 2nd char on the 2nd row
        for _ in 0..11 {
            it.next();
        }
        let (_glyph, _fg, _bg, ps) = it.next().unwrap();

        // That top-left coord should be the offset plus a 2x width and a 4x height:
        assert_eq!(ps, pxy(50 + 16, 50 + 32));
    }

    #[test]
    fn test_get_and_mut() {
        let font = Font::default();
        let mut layer = Layer::new(&font, xy(10, 10), pxy(2, 4), pxy(50, 50));

        *layer.get_mut(xy(2, 3)).ch = 'x' as u8;
        assert_eq!(layer.get(xy(2, 3)).ch.unwrap(), 'x' as u8);
    }

    #[test]
    fn test_set() {
        let font = Font::default();
        let mut layer = Layer::new(&font, xy(10, 10), pxy(2, 4), pxy(50, 50));

        let mut cell = layer.get(xy(0, 0));
        cell.ch = Some('#' as u8);
        cell.fg = None;
        *layer.get_mut(xy(3, 2)).fg = RED;
        layer.set(xy(3, 2), cell);
        assert_eq!(layer.get(xy(3, 2)).ch.unwrap(), '#' as u8);
        assert_eq!(layer.get(xy(3, 2)).fg.unwrap(), RED);
    }
}