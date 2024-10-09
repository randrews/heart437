use image::Rgba;

/// A simple Color struct
/// ```
/// let slashdot = heart437::Color::rgba(0, 102, 102, 255);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Self { r, g, b, a }
    }

    /// Return the RGBA bytes of this color laid over an opaque background of another color.
    /// The bg arg is a [u8; 4] but only the first three bytes (r, g, b) matter.
    /// ```
    /// heart437::Color::rgba(0, 0, 0, 127).blend_into(&[0, 120, 160, 255]);
    /// ```
    pub fn blend_into(&self, bg: &[u8]) -> [u8; 4] {
        let a = (self.a as f32) / 255.0;
        if let [bgr, bgg, bgb, _] = bg {
            let mut out = [0, 0, 0, 255];
            out[0] = ((self.r as f32 * a) + (*bgr as f32 * (1.0 - a))) as u8;
            out[1] = ((self.g as f32 * a) + (*bgg as f32 * (1.0 - a))) as u8;
            out[2] = ((self.b as f32 * a) + (*bgb as f32 * (1.0 - a))) as u8;
            out
        } else {
            panic!("Sir this is a Wendy's.")
        }
    }
}

pub const CLEAR: Color = Color { r: 0, g: 0, b: 0, a: 0 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 255 };
pub const PURPLE: Color = Color { r: 255, g: 0, b: 255, a: 255 };

impl Into<Rgba<u8>> for Color {
    fn into(self) -> Rgba<u8> {
        Rgba::from([self.r, self.g, self.b, self.a])
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}