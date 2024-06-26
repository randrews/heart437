mod color;
mod cell;
mod font;
mod layer;
mod drawing;
mod grid;
mod vecgrid;
mod coords;
mod keyboard;
mod sprite;

pub use font::{ Font, Glyph };
pub use color::{ Color, CLEAR, WHITE, BLACK, RED, GREEN, BLUE, YELLOW, PURPLE };
pub use cell::{ Cell, Fg, Bg, Char, FgBg, FgChar, BgChar };
pub use layer::{ Layer };
pub use sprite::Sprite;
pub use drawing::{ Canvas, RectStyle, Wall };
pub use coords::{ Coord, xy, PixelCoord, pxy, Dir };
pub use grid::{ Grid, GridMut, CountableNeighbors };
pub use vecgrid::{VecGrid};
pub use keyboard::ToDirection;

#[cfg(feature="rand")]
mod mapgen;
#[cfg(feature="rand")]
pub use mapgen::CellularMap;

#[cfg(feature = "fov")]
mod fov;
#[cfg(feature = "fov")]
pub use fov::shadowcast;
