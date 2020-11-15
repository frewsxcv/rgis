use geo::algorithm::contains::Contains;
use geo::algorithm::map_coords::MapCoordsInplace;

#[derive(Debug, Copy, Clone)]
pub struct CoordWithSrs<T: num_traits::Float> {
    pub coord: geo::Coordinate<T>,
    pub srs: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct RectWithSrs<T: num_traits::Float> {
    pub rect: geo::Rect<T>,
    pub srs: &'static str,
}

impl<T: num_traits::Float> RectWithSrs<T> {
    pub fn contains(&self, coord: &CoordWithSrs<T>) -> bool {
        assert_eq!(self.srs, coord.srs);
        self.rect.contains(&coord.coord)
    }

    pub fn merge(self, other: RectWithSrs<T>) -> RectWithSrs<T> {
        assert_eq!(self.srs, other.srs);
        RectWithSrs {
            rect: geo::Rect::new(
                geo::Coordinate {
                    x: self.rect.min().x.min(other.rect.min().x),
                    y: self.rect.min().y.min(other.rect.min().y),
                },
                geo::Coordinate {
                    x: self.rect.max().x.max(other.rect.max().x),
                    y: self.rect.max().y.max(other.rect.max().y),
                },
            ),
            srs: self.srs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeometryWithSrs<T: num_traits::Float> {
    pub geometry: geo::Geometry<T>,
    pub srs: &'static str,
}

impl<T: num_traits::Float> GeometryWithSrs<T> {
    // pub fn contains(&self, coord: &CoordWithSrs<T>) -> bool {
    //     assert_eq!(self.srs, coord.srs);
    //     // self.geometry.contains(&coord.coord)
    //     Contains::contains(&self.geometry, &coord.coord)
    // }

    pub fn reproject(&mut self, target_srs: &'static str) {
        let projector =
            geo::algorithm::proj::Proj::new_known_crs(self.srs, target_srs, None).unwrap();

        self.geometry
            .map_coords_inplace(|&(x, y)| projector.convert((x, y)).unwrap().x_y())
    }
}
