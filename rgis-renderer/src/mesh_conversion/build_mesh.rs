use geo_traits::{CoordTrait, GeometryType};
use std::iter;

pub trait BuildMesh {
    fn build(self) -> Result<crate::mesh_conversion::GeometryMesh, crate::mesh_conversion::Error>;
}

pub struct BuildBevyMeshesContext<Scalar: geo_types::CoordFloat> {
    pub point_mesh_builder: crate::mesh_conversion::point::PointMeshBuilder,
    pub line_string_mesh_builder: crate::mesh_conversion::line_string::LineStringMeshBuilder,
    pub polygon_mesh_builder: crate::mesh_conversion::polygon::PolygonMeshBuilder<Scalar>,
}

impl<Scalar: geo_types::CoordFloat> Default for BuildBevyMeshesContext<Scalar> {
    fn default() -> Self {
        Self {
            point_mesh_builder: Default::default(),
            line_string_mesh_builder: Default::default(),
            polygon_mesh_builder: Default::default(),
        }
    }
}

fn populate_point_mesh_builders<Scalar: geo_types::CoordFloat>(
    point: &impl geo_traits::PointTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    if let Some(coord) = point.coord() {
        ctx.point_mesh_builder.add_coord(coord);
    }
    Ok(())
}

fn populate_line_string_mesh_builders<Scalar: geo_types::CoordFloat>(
    line_string: &impl geo_traits::LineStringTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    ctx.line_string_mesh_builder
        .add_coords(line_string.coords())
}

fn populate_polygon_mesh_builders<Scalar: geo_types::CoordFloat>(
    polygon: &impl geo_traits::PolygonTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    ctx.polygon_mesh_builder.add_polygon(polygon)
}

fn populate_multi_point_mesh_builders<Scalar: geo_types::CoordFloat>(
    multi_point: &impl geo_traits::MultiPointTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    for point in multi_point.points() {
        populate_point_mesh_builders(&point, ctx)?;
    }
    Ok(())
}

fn populate_multi_line_string_mesh_builders<Scalar: geo_types::CoordFloat>(
    multi_line_string: &impl geo_traits::MultiLineStringTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    for line_string in multi_line_string.line_strings() {
        populate_line_string_mesh_builders(&line_string, ctx)?;
    }
    Ok(())
}

fn populate_multi_polygon_mesh_builders<Scalar: geo_types::CoordFloat>(
    multi_polygon: &impl geo_traits::MultiPolygonTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    for polygon in multi_polygon.polygons() {
        populate_polygon_mesh_builders(&polygon, ctx)?;
    }
    Ok(())
}

fn populate_geometry_collection_mesh_builders<Scalar: geo_types::CoordFloat>(
    geometry_collection: &impl geo_traits::GeometryCollectionTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    for g in geometry_collection.geometries() {
        populate_geometry_mesh_builders(&g, ctx)?;
    }
    Ok(())
}

fn populate_triangle_mesh_builders<Scalar: geo_types::CoordFloat>(
    triangle: &impl geo_traits::TriangleTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    // TODO: build with earcutr directly
    let polygon = geo_types::Polygon::new(
        geo_types::LineString::new(
            triangle
                .coords()
                .into_iter()
                .map(|c| geo_types::Coord { x: c.x(), y: c.y() })
                .collect(),
        ),
        vec![],
    );
    ctx.polygon_mesh_builder.add_polygon(&polygon)
}

fn populate_rect_mesh_builders<Scalar: geo_types::CoordFloat>(
    rect: &impl geo_traits::RectTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    ctx.polygon_mesh_builder
        .add_polygon_from_exterior_coords(rect_coords_iter(rect))
}

fn rect_coords_iter<
    'a,
    Scalar: geo_types::CoordFloat + 'a,
    PointType: geo_traits::CoordTrait<T = Scalar> + 'a,
>(
    rect: &'a impl geo_traits::RectTrait<T = Scalar, CoordType<'a> = PointType>,
) -> impl Iterator<Item = (Scalar, Scalar)> + Clone {
    let (min, max) = (rect.min(), rect.max());
    [
        (min.x(), min.y()),
        (min.x(), max.y()),
        (max.x(), max.y()),
        (max.x(), min.y()),
    ]
    .into_iter()
}

fn populate_line_mesh_builders<Scalar: geo_types::CoordFloat>(
    line: &impl geo_traits::LineTrait<T = Scalar>,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    let iter = iter::once(line.start()).chain(iter::once(line.end()));
    ctx.line_string_mesh_builder.add_coords(iter)
}

pub fn populate_geometry_mesh_builders<
    Scalar: geo_types::CoordFloat,
    G: geo_traits::GeometryTrait<T = Scalar>,
>(
    geometry: &G,
    ctx: &mut BuildBevyMeshesContext<Scalar>,
) -> Result<(), crate::mesh_conversion::Error> {
    match geometry.as_type() {
        GeometryType::Point(g) => populate_point_mesh_builders(g, ctx)?,
        GeometryType::LineString(g) => populate_line_string_mesh_builders(g, ctx)?,
        GeometryType::Polygon(g) => populate_polygon_mesh_builders(g, ctx)?,
        GeometryType::MultiPoint(g) => populate_multi_point_mesh_builders(g, ctx)?,
        GeometryType::MultiLineString(g) => populate_multi_line_string_mesh_builders(g, ctx)?,
        GeometryType::MultiPolygon(g) => populate_multi_polygon_mesh_builders(g, ctx)?,
        GeometryType::GeometryCollection(g) => populate_geometry_collection_mesh_builders(g, ctx)?,
        GeometryType::Triangle(g) => populate_triangle_mesh_builders(g, ctx)?,
        GeometryType::Rect(g) => populate_rect_mesh_builders(g, ctx)?,
        GeometryType::Line(g) => populate_line_mesh_builders(g, ctx)?,
    }
    Ok(())
}
