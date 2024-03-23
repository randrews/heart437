use std::ops::{Index, IndexMut};
use crate::{Color, Coord, Layer};

macro_rules! layer_proxy {
    ($field:ident, $name:ident, $tp:ty) => {
        pub struct $name<'a>(pub &'a mut Layer<'a>);

        impl Index<Coord> for $name<'_> {
            type Output = $tp;
            fn index(&self, index: Coord) -> &Self::Output {
                &(self.0.grid()[index].$field)
            }
        }

        impl IndexMut<Coord> for $name<'_> {
            fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
                &mut self.0.grid_mut()[index].$field
            }
        }
    }
}

layer_proxy!(fg, FgProxy, Color);
layer_proxy!(bg, BgProxy, Color);
layer_proxy!(ch, ChProxy, u8);