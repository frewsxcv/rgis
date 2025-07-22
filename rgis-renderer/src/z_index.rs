use crate::RenderEntityType;

pub struct ZIndex(pub usize);

impl ZIndex {
    // Example:
    //
    // Layer (index 0, Polygon with border):
    //   z=0: Polygon
    //   z=1: LineString
    //   z=2:
    //   z=3:
    //   z=4:
    //   z=5:
    //   z=6:
    //
    // Layer (index 1, Point):
    //   z=7:
    //   z=8:
    //   z=9: PointStroke
    //   z=10: PointFill
    //   z=11:
    //   z=12:
    //   z=13:
    //
    // Layer (index 2, selected Polygon):
    //   z=14: Polygon
    //   z=15: LineString
    //   z=16:
    //   z=17:
    //   z=18: Selected Polygon
    //   z=19: Selected LineString
    //   z=20:
    pub fn calculate(
        // Lower index is below higher index.
        layer_index: rgis_layers::LayerIndex,
        entity_type: RenderEntityType,
    ) -> Self {
        ZIndex(
            layer_index.0 * 7
                + match entity_type {
                    RenderEntityType::Polygon => 0,
                    RenderEntityType::LineString => 1,
                    RenderEntityType::PointStroke => 2,
                    RenderEntityType::PointFill => 3,
                    RenderEntityType::SelectedPolygon => 4,
                    RenderEntityType::SelectedLineString => 5,
                    RenderEntityType::SelectedPoint => 6,
                    RenderEntityType::Raster => 0,
                },
        )
    }
}
