use geo::prelude::*;

#[derive(Debug, Clone)]
pub struct CoordWithSrs<T: geo::CoordFloat> {
    pub coord: geo::Coordinate<T>,
    pub srs: String,
}

impl<T: geo::CoordFloat> CoordWithSrs<T> {
    pub fn reproject(&mut self, target_srs: &str) {
        self.coord = geo::Point(self.coord).transform_crs_to_crs(&self.srs, target_srs).unwrap().0;
        self.srs = target_srs.to_owned();
    }
}

#[derive(Debug, Clone)]
pub struct RectWithSrs<T: geo::CoordFloat> {
    pub rect: geo::Rect<T>,
    pub srs: String,
}

impl<T: geo::CoordFloat> RectWithSrs<T> {
    pub fn contains(&self, coord: &CoordWithSrs<T>) -> bool {
        assert_eq!(self.srs, coord.srs);
        self.rect.contains(&coord.coord)
    }

    pub fn merge(self, other: &RectWithSrs<T>) -> RectWithSrs<T> {
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
pub struct GeometryWithSrs<T: geo::CoordFloat> {
    pub geometry: geo::Geometry<T>,
    pub srs: String,
}

impl<T: geo::CoordFloat> GeometryWithSrs<T> {
    // pub fn contains(&self, coord: &CoordWithSrs<T>) -> bool {
    //     assert_eq!(self.srs, coord.srs);
    //     // self.geometry.contains(&coord.coord)
    //     Contains::contains(&self.geometry, &coord.coord)
    // }

    pub fn reproject(&mut self, target_srs: &str) {
        self.geometry = self.geometry.transform_crs_to_crs(&self.srs, target_srs).unwrap();
        self.srs = target_srs.to_owned();
    }
}
