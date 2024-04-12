use crate::{Coord, Grid, VecGrid, xy};
use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, MapData};

pub fn shadowcast<G: Grid<CellType=bool>>(grid: G, loc: Coord, radius: u32) -> VecGrid<bool> {
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

impl<G: Grid<CellType=bool>> Doryenable for G {
    fn mapdata(&self) -> MapData {
        let mut map_data = MapData::new(self.size().0 as usize, self.size().1 as usize);
        for pt in self.size() {
            map_data.set_transparent(pt.0 as usize, pt.1 as usize, *self.get(pt).unwrap())
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

    #[test]
    fn test_fov() {
        // This will involve some setup. First we build a map:
        let str = "....+.\n\
                         ....+.\n\
                         +.+++.\n\
                         ......\n\
                         .+....\n\
                         ......";
        let map = VecGrid::from(str);
        // Now we need to turn this into a map of what's transparent:
        let transparent = map.map_grid(|_, ch| *ch == '.', false);
        // Then actually shadowcast:
        let visible = shadowcast(transparent, xy(1, 1), 20);
        // Then turn that back into a map of chars:
        let visible = visible.map_grid(|c, v| if *v { map[c] } else { '#' }, '.');
        // Now we compare that to the correct answer:
        let expected = "....+#\n\
                              ....+#\n\
                              +.+++#\n\
                              ...###\n\
                              .+.###\n\
                              .#..##";
        let actual: String = visible.into();
        assert_eq!(actual.as_str(), expected)
    }
}