use std::ops::IndexMut;
use crate::color::{CLEAR, Color};
use crate::font::{Font, Glyph};

/// Represents a rectangular grid of colored glyphs. Has a size (in characters) and a position (in pixels).
/// Borrows a font to know which glyphs to draw.
#[derive(Clone, Debug)]
pub struct Layer<'a> {
    font: &'a Font,
    size: (usize, usize),
    origin: (usize, usize),
    chars: Vec<Char>
}

impl<'a> Layer<'a> {
    /// The normal way to create a Layer.
    /// ```
    /// let font = textgraph::Font::default();
    /// let layer = textgraph::Layer::new(&font, (80, 25), (0, 0));
    /// ```
    pub fn new(font: &'a Font, size: (usize, usize), origin: (usize, usize)) -> Self {
        let default = Char { ch: 0, fg: CLEAR, bg: CLEAR };
        let chars = vec![default; size.0 * size.1];
        Self {
            font,
            size,
            origin,
            chars
        }
    }

    /// Returns the size of this Layer
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    /// Returns an iterator used to iterate over all the cells in the layer:
    ///```
    /// # let font = textgraph::Font::default();
    /// # let layer = textgraph::Layer::new(&font, (10, 10), (0, 0));
    /// for (glyph, fg, bg, x, y) in layer.cells() {
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
    /// # let mut layer = Layer::new(&font, (10, 10), (0, 0));
    /// layer.set((3, 3), Some('A'), Some(WHITE), Some(CLEAR));
    /// ```
    pub fn set(&mut self, at: (usize, usize), ch: Option<char>, fg: Option<Color>, bg: Option<Color>) {
        let current = self.chars.index_mut(at.0 + at.1 * self.size.0);
        ch.map(|ch| current.ch = ch as u8);
        fg.map(|fg| current.fg = fg);
        bg.map(|bg| current.bg = bg);
    }

    fn get_mut(&mut self, at: (usize, usize)) -> Option<&mut Char> {
        self.chars.get_mut(at.0 + at.1 * self.size.0)
    }
}

#[derive(Copy, Clone, Debug)]
struct Char {
    ch: u8,
    fg: Color,
    bg: Color,
}

/// Iterator over the characters of a `Layer`
/// Usually created through `Layer::cells`
pub struct CharIterator<'a> {
    layer: &'a Layer<'a>,
    n: usize
}

impl Iterator for CharIterator<'_> {
    type Item = (Glyph, Color, Color, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        self.n += 1;
        if n >= self.layer.chars.len() {
            None
        } else {
            let Char{ch, fg, bg} = self.layer.chars[n];
            let glyph = self.layer.font[ch];
            let px = n % self.layer.size.0 * 8 + self.layer.origin.0;
            let py = n / self.layer.size.0 * 8 + self.layer.origin.1;
            Some((glyph, fg, bg, px, py))
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
    /// let mut layer = Layer::new(&font, (10, 10), (25, 25));
    /// layer.fill('R', WHITE, BLUE);
    /// let mut buf = [0u8; (640 * 480 * 4)];
    /// layer.draw(&mut buf, 640);
    /// ```
    fn draw(&self, pixels: &mut [u8], width: usize) {
        let height = (pixels.len() / 4) / width;
        for (glyph, fg, bg, x, y) in self.cells() {
            for (color, xo, yo) in &glyph {
                if (x + xo) < width && (y + yo) < height {
                    let start = (x + xo) * 4 + (y + yo) * width * 4;
                    let current = &mut pixels[start .. (start + 4)];
                    let new = (if color { fg } else { bg }).blend_into(current);
                    for n in 0..4 { current[n] = new[n] }
                }
            }
        }
    }
}
