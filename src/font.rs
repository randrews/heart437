use image::{DynamicImage, GenericImageView};

/// A set of 256 glyphs, 8x8 pixels in size, which can be rendered to a `Layer` in a foreground
/// and background color.
#[derive(Copy, Clone, Debug)]
pub struct Font {
    glyphs: [Glyph; 256]
}

/// A single symbol in a `Font`, 8x8 pixels in size
#[derive(Copy, Clone, Debug)]
pub struct Glyph([u8; 8]);

impl Default for Glyph {
    /// Returns a `Glyph` that is entirely blank (when rendered, every pixel will be the background
    /// color)
    fn default() -> Self {
        Self([0; 8])
    }
}

impl Glyph {
    fn from_image_slice(image: &DynamicImage, x: u32, y: u32) -> Self {
        let mut bytes = [0u8; 8];

        for yo in 0..8 {
            for xo in 0..8 {
                if image.get_pixel(x * 8 + xo, y * 8 + yo).0[3] != 0 {
                    bytes[yo as usize] |= 1 << (7 - xo)
                }
            }
        }
        Self(bytes)
    }
}

/// An iterator over each pixel in a `Glyph`
pub struct GlyphIterator<'a> (&'a Glyph, usize);
impl Iterator for GlyphIterator<'_> {
    type Item = (bool, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.1;
        self.1 += 1;
        if n >= 64 {
            None
        } else {
            let (x, y) = (n % 8, n / 8);
            let b = self.0.0[y];
            let color = (b & (1 << (7 - x))) != 0;
            Some((color, x, y))
        }
    }
}

impl<'a> IntoIterator for &'a Glyph {
    type Item = (bool, usize, usize);
    type IntoIter = GlyphIterator<'a>;

    /// Convert a `&Glyph` into an iterator over each pixel
    /// ```
    /// # use heart437::*;
    /// # let font = Font::default();
    /// let glyph = font[65];
    /// for (on, x, y) in &glyph {
    ///   // do something with each pixel, like draw a color depending on whether `on` is true
    /// }
    /// ```
    /// The iterator yields `(bool, usize, usize)`. If the bool is true, the glyph expects that
    /// pixel to be the foreground color; otherwise background color. The x and y coordinates range
    /// from 0..7, with (0, 0) being the top left.
    fn into_iter(self) -> Self::IntoIter {
        GlyphIterator(self, 0)
    }
}

impl From<[u8; 8]> for Glyph {
    /// Create a glyph from an 8x8 bitmap. Each byte is a row, low-order bit is the right edge
    /// ```
    /// let glyph = heart437::Glyph::from([
    ///   0b00000000,
    ///   0b00011000,
    ///   0b10011000,
    ///   0b01111110,
    ///   0b00011001,
    ///   0b00100100,
    ///   0b01000010,
    ///   0b11000011,
    /// ]);
    /// ```
    fn from(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
}

impl Default for Font {
    /// Builds a `Font` from an 8x8 bitmap found here: https://int10h.org/oldschool-pc-fonts/readme/
    /// US law does not consider typefaces copyrightable; the bitmap representation of a font (which
    /// this is) should be free to redistribute and use. (TrueType / vector fonts are a different
    /// story though)
    fn default() -> Self {
        Self::from_png(include_bytes!("font.png"))
    }
}

impl Font {
    /// Takes the bytes of a PNG image of 256 8x8 glyphs and turns them into a `Font`.
    /// Glyphs are read left-to-right, top-to-bottom, but the actual dimensions of the image don't
    /// matter as long as it's large enough.
    /// The image must be a transparent PNG; any pixel with 0 for alpha is taken to be background,
    /// anything non-zero alpha is foreground.
    /// ```
    /// let font = heart437::Font::from_png(include_bytes!("font.png"));
    /// ```
    pub fn from_png(image_data: &[u8]) -> Self {
        let image = image::load_from_memory_with_format(image_data, image::ImageFormat::Png).unwrap();
        let w = image.width() / 8;
        let mut glyphs = [Glyph::default(); 256];
        for n in 0..256 {
            let (x, y) = (n % w, n / w);
            glyphs[n as usize] = Glyph::from_image_slice(&image, x as u32, y as u32);
        }

        Self { glyphs }
    }
}

impl std::ops::IndexMut<u8> for Font {
    /// Fetch the `Glyph` corresponding to a given u8 in this font
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.glyphs[index as usize]
    }
}

impl std::ops::Index<u8> for Font {
    type Output = Glyph;

    /// Fetch the `Glyph` corresponding to a given u8 in this font
    fn index(&self, index: u8) -> &Self::Output {
        &self.glyphs[index as usize]
    }
}

impl std::ops::IndexMut<char> for Font {
    /// Fetch the `Glyph` corresponding to a given char in this font. Fonts are only defined for
    /// ASCII chars; will panic if passed a non-ASCII char!
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        assert!(index.is_ascii(), "Fonts are only defined for ASCII chars!");
        &mut self.glyphs[index as usize]
    }
}

impl std::ops::Index<char> for Font {
    type Output = Glyph;

    /// Fetch the `Glyph` corresponding to a given char in this font. Fonts are only defined for
    /// ASCII chars; will panic if passed a non-ASCII char!
    fn index(&self, index: char) -> &Self::Output {
        assert!(index.is_ascii(), "Fonts are only defined for ASCII chars!");
        &self.glyphs[index as usize]
    }
}