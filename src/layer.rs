use std::ops::{Index, IndexMut};
use crate::color::{Color};
use crate::font::{Font, Glyph};
use crate::{Cell, Char, Coord, pxy, Sprite, VecGrid, xy};
use crate::coords::PixelCoord;
use crate::grid::{Grid, GridMut};

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

    width: i32,
    data: Vec<Cell>
}

impl<'a> Layer<'a> {
    /// The normal way to create a Layer.
    /// ```
    /// # use textgraph::*;
    /// let font = Font::default();
    /// let layer = Layer::new(&font, xy(80, 25), pxy(1, 1), pxy(0, 0));
    /// ```
    pub fn new(font: &'a Font, size: Coord, scale: PixelCoord, origin: PixelCoord) -> Self {
        let len = (size.0 * size.1) as usize;
        let data = vec![Cell::default(); len];
        Self {
            font,
            scale,
            origin,
            data,
            width: size.0
        }
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

    /// Returns the `PixelCoord` corresponding to a given `Coord` in this layer, taking into account
    /// the scale factor and origin.
    pub fn pixel_coord(&self, coord: Coord) -> PixelCoord {
        let (scalex, scaley) = (self.scale.0.max(1), self.scale.1.max(1));
        let px = coord.0 * 8 * scalex + self.origin.0;
        let py = coord.1 * 8 * scaley + self.origin.1;
        pxy(px, py)
    }

    /// Create a new VecGrid<Char> with the characters (uncolored) from this Layer
    pub fn chars(&self) -> VecGrid<Char> {
        let v = self.data.iter().map(|c| Char::from(*c));
        VecGrid::from_vec(v.collect(), self.width as usize, Char(' ' as u8))
    }

    fn blit(&self, pixels: &mut [u8], width: usize, glyph: Glyph, fg: Color, bg: Color, pc: PixelCoord, scale: PixelCoord) {
        let PixelCoord(x, y) = pc;
        let PixelCoord(xscale, yscale) = scale;
        let height = (pixels.len() / 4) / width; // Height of the pixel buffer in pixels

        if x >= width as i32 || y >= height as i32 { return }
        let (right, bottom) = (x + xscale * 8, y + yscale * 8);
        if right < 0 || bottom < 0 { return }

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

    /// Draws the Layer into the frame at its pixel position:
    /// ```
    /// # use textgraph::*;
    /// # let font = Font::default();
    /// let mut layer = Layer::new(&font, Coord(10, 10), PixelCoord(1, 1), PixelCoord(25, 25));
    /// layer.fill(Some('R'), Some(WHITE), Some(BLUE));
    /// let mut buf = [0u8; (640 * 480 * 4)];
    /// layer.draw(&mut buf, 640);
    /// ```
    pub fn draw(&self, pixels: &mut [u8], width: usize) {
        let scale = PixelCoord(self.scale.0.max(1), self.scale.1.max(1));

        for (glyph, fg, bg, pc) in self.cells() {
            self.blit(pixels, width, glyph, fg, bg, pc, scale)
        }
    }

    /// Draw a list of sprites to the pixel buffer. Sprites aren't stored as part of the layer,
    /// you can manage them separately (like in an ECS), but it's often useful to draw them with
    /// the same layout as the layer they're on top of.
    /// Sprites store a cell, their own scale (which still treats 1x as a minimum), and a position,
    /// which is treeated as an offset from the layer's position: if the layer has position (50, 50)
    /// and a sprite has position (25, 25), the sprite will actually be drawn at (75, 75). In other
    /// words, moving the layer also moves the sprites it draws.
    pub fn draw_sprites<'b, I: Iterator<Item=&'b Sprite>, II: IntoIterator<IntoIter=I>>(&self, sprites: II, pixels: &mut [u8], width: usize) {
        for sprite in sprites {
            let Cell { ch, fg, bg} = sprite.cell;
            let glyph = self.font[ch];
            let scale = PixelCoord(sprite.scale.0.max(1), sprite.scale.1.max(1));
            self.blit(pixels, width, glyph, fg, bg, sprite.position + self.origin, scale)
        }
    }
}

impl Grid for Layer<'_> {
    type CellType = Cell;
    fn size(&self) -> Coord {
        xy(self.width, self.data.len() as i32 / self.width)
    }
    fn default(&self) -> Cell {
        Cell::default()
    }
    fn get(&self, index: Coord) -> Option<&Cell> {
        if self.contains(index) {
            Some(&self.data[(index.0 + self.width * index.1) as usize])
        } else {
            None
        }
    }
}

impl GridMut for Layer<'_> {
    fn get_mut(&mut self, index: Coord) -> Option<&mut Cell> {
        if self.contains(index) {
            Some(&mut self.data[(index.0 + self.width * index.1) as usize])
        } else {
            None
        }
    }
}

impl Index<Coord> for Layer<'_> {
    type Output = Cell;
    fn index(&self, index: Coord) -> &Self::Output { self.get(index).unwrap() }
}

impl IndexMut<Coord> for Layer<'_> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output { self.get_mut(index).unwrap() }
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
        let coord = self.layer.coord(n);
        if let Some(c) = self.layer.get(coord) {
            let Cell { ch, fg, bg } = *c;
            let glyph = self.layer.font[ch];
            Some((glyph, fg, bg, self.layer.pixel_coord(coord)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Bg, Char, Fg, RED, YELLOW};
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
        layer[at] |= Char('!' as u8) + Fg(YELLOW) + Bg(RED);

        assert_eq!(Char::from(layer[at]), Char('!' as u8));
        assert_eq!(Fg::from(layer[at]), Fg(YELLOW));
        assert_eq!(Bg::from(layer[at]), Bg(RED));
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
    fn test_chars() {
        let font = Font::default();
        let mut layer = Layer::new(&font, xy(10, 10), pxy(2, 4), pxy(50, 50));
        layer[xy(3, 5)] |= Char('a' as u8);
        assert_eq!(layer.chars()[xy(3, 5)], Char('a' as u8));
    }
}