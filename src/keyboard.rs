use crate::coords::Dir;

/// Convert a keyboard scancode to a direction, up / down / left / right
pub trait ToDirection: Sized {
    /// Return the keyboard scan code for `self`
    fn as_scancode(self) -> u32;

    /// Map from scancode to which arrow key it is
    fn to_direction(self) -> Option<Dir> {
        match self.as_scancode() {
            123 => Some(Dir::West),
            124 => Some(Dir::East),
            125 => Some(Dir::South),
            126 => Some(Dir::North),
            _ => None
        }
    }
}

impl ToDirection for Option<u32> {
    fn as_scancode(self) -> u32 {
        self.unwrap_or(0)
    }
}