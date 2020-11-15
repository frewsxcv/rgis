use geo::algorithm::map_coords::MapCoordsInplace;
// use geo::algorithm::contains::Contains;

#[derive(Debug, Clone)]
pub struct CoordWithSrs<T: num_traits::Float> {
    pub coord: geo::Coordinate<T>,
    pub srs: &'static str,
}

#[derive(Debug, Clone)]
pub struct RectWithSrs<T: num_traits::Float> {
    pub rect: geo::Rect<T>,
    pub srs: &'static str,
}

#[derive(Debug, Clone)]
pub struct GeometryWithSrs<T: num_traits::Float> {
    pub geometry: geo::Geometry<T>,
    pub srs: &'static str,
}

impl<T: num_traits::Float> GeometryWithSrs<T> {
    // pub fn contains(&self, coord: &CoordWithSrs<f64>) -> bool {
    //     assert_eq!(self.srs, coord.srs);
    //     self.geometry.contains(&coord.coord)
    // }

    pub fn reproject(&mut self, target_srs: &'static str) {
        let projector = geo::algorithm::proj::Proj::new_known_crs(
            self.srs,
            target_srs,
            None,
        )
        .unwrap();

        self.geometry.map_coords_inplace(|&(x, y)| {
            projector.convert((x, y)).unwrap().x_y()
        })
    }
}
