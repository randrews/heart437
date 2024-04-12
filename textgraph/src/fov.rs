use crate::{Coord, Grid, VecGrid, xy};
use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, MapData};

pub fn shadowcast<G: Grid<Output=bool>>(grid: G, loc: Coord, radius: u32) -> VecGrid<bool> {
    let mut map_data = grid.mapdata();
    FovRecursiveShadowCasting::new().compute_fov(&mut map_data,
                                                 loc.0 as usize,
                                                 loc.1 as usize,
                                                 radius as usize,
                                                 true);
    map_data.into()
}

trait Doryenable {
    fn mapdata(&self) -> MapData;
}

impl<G: Grid<Output=bool>> Doryenable for G {
    fn mapdata(&self) -> MapData {
        let mut map_data = MapData::new(self.size().0 as usize, self.size().1 as usize);
        for pt in self.size() {
            map_data.set_transparent(pt.0 as usize, pt.1 as usize, self[pt])
        }
        map_data
    }
}

impl From<MapData> for VecGrid<bool> {
    fn from(mapdata: MapData) -> Self {
        let size = xy(mapdata.width as i32, mapdata.height as i32);
        let mut grid = Self::new(size, false);
        for pt in size {
            grid[pt] = mapdata.is_in_fov(pt.0 as usize, pt.1 as usize)
        }
        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;
}