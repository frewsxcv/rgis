use bevy_prototype_lyon::prelude::*;

// TODO: interiors
pub fn convert(g: &geo_types::Polygon<f64>) -> Path {
    let mut path_builder = PathBuilder::new();

    let mut exterior_iter = g.exterior().0.iter();

    let first = exterior_iter.next().unwrap();
    path_builder.move_to(point(first.x as f32, first.y as f32));

    for coord in exterior_iter {
        path_builder.line_to(point(coord.x as f32, coord.y as f32))
    }

    path_builder.close();

    path_builder.build()
}
