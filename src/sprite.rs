use crate::{Cell, PixelCoord};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sprite {
    pub cell: Cell,
    pub position: PixelCoord,
    pub scale: PixelCoord,
}
