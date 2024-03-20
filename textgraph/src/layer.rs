use std::ops::IndexMut;
use crate::color::{CLEAR, Color};
use crate::font::{Font, Glyph};

/// A dimension in pixel terms. This is "pixel" in the sense of whatever
/// unspecified thing you're drawing to, meaning, this might get scaled
/// for a `pixels` scaling factor and a hidpi scaling factor
#[derive(Copy, Clone, Debug)]
pub struct PixelSize(pub i32, pub i32);

/// A dimension in character terms.
#[derive(Copy, Clone, Debug)]
pub struct CharSize(pub usize, pub usize);

/// Represents a rectangular grid of colored glyphs.
/// - size (in characters), the character dimensions of the layer
/// - position (in pixels),
/// Borrows a font to know which glyphs to draw.
#[derive(Clone, Debug)]
pub struct Layer<'a> {
    font: &'a Font,
    size: CharSize,
    scale: PixelSize,
    origin: PixelSize,
    chars: Vec<Char>
}

impl<'a> Layer<'a> {
    /// The normal way to create a Layer.
    /// ```
    /// let font = textgraph::Font::default();
    /// let layer = textgraph::Layer::new(&font, textgraph::CharSize(80, 25), textgraph::PixelSize(1, 1), textgraph::PixelSize(0, 0));
    /// ```
    pub fn new(font: &'a Font, size: CharSize, scale: PixelSize, origin: PixelSize) -> Self {
        let default = Char { ch: 0, fg: CLEAR, bg: CLEAR };
        let chars = vec![default; size.0 * size.1];
        Self {
            font,
            size,
            scale,
            origin,
            chars
        }
    }

    /// Returns the size of this Layer
    pub fn size(&self) -> CharSize {
        self.size
    }

    /// Returns an iterator used to iterate over all the cells in the layer:
    ///```
    /// # let font = textgraph::Font::default();
    /// # let layer = textgraph::Layer::new(&font, textgraph::CharSize(10, 10), textgraph::PixelSize(1, 1), textgraph::PixelSize(0, 0));
    /// for (glyph, fg, bg, textgraph::PixelSize(x, y)) in layer.cells() {
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
    /// # let mut layer = Layer::new(&font, CharSize(10, 10), PixelSize(1, 1), PixelSize(0, 0));
    /// layer.set(CharSize(3, 3), Some('A'), Some(WHITE), Some(CLEAR));
    /// ```
    pub fn set(&mut self, at: CharSize, ch: Option<char>, fg: Option<Color>, bg: Option<Color>) {
        let current = self.chars.index_mut(at.0 + at.1 * self.size.0);
        ch.map(|ch| current.ch = ch as u8);
        fg.map(|fg| current.fg = fg);
        bg.map(|bg| current.bg = bg);
    }

    fn get_mut(&mut self, at: CharSize) -> Option<&mut Char> {
        self.chars.get_mut(at.0 + at.1 * self.size.0)
    }

    pub fn set_origin(&mut self, origin: PixelSize) {
        self.origin = origin
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
    type Item = (Glyph, Color, Color, PixelSize);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        self.n += 1;
        if n >= self.layer.chars.len() {
            None
        } else {
            let Char{ch, fg, bg} = self.layer.chars[n];
            let glyph = self.layer.font[ch];
            let (scalex, scaley) = (self.layer.scale.0.max(1), self.layer.scale.1.max(1));
            let n = n as i32;
            let width = self.layer.size.0 as i32;
            let px = n % width * 8 * scalex + self.layer.origin.0;
            let py = n / width * 8 * scaley + self.layer.origin.1;
            Some((glyph, fg, bg, PixelSize(px, py)))
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
    /// let mut layer = Layer::new(&font, CharSize(10, 10), PixelSize(1, 1), PixelSize(25, 25));
    /// layer.fill(Some('R'), Some(WHITE), Some(BLUE));
    /// let mut buf = [0u8; (640 * 480 * 4)];
    /// layer.draw(&mut buf, 640);
    /// ```
    fn draw(&self, pixels: &mut [u8], width: usize) {
        let (xscale, yscale) = (self.scale.0.max(1), self.scale.1.max(1));
        let height = (pixels.len() / 4) / width; // Height of the pixel buffer in pixels

        for (glyph, fg, bg, PixelSize(x, y)) in self.cells() {
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
