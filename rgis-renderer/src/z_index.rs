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
    //
    // Layer (index 1, Point):
    //   z=6:
    //   z=7:
    //   z=8: Point
    //   z=9:
    //   z=10:
    //   z=11:
    //
    // Layer (index 2, selected Polygon):
    //   z=12: Polygon
    //   z=13: LineString
    //   z=14:
    //   z=15: Selected Polygon
    //   z=16: Selected LineString
    //   z=17:
    pub fn calculate(
        // Lower index is below higher index.
        layer_index: rgis_layers::LayerIndex,
        entity_type: RenderEntityType,
    ) -> Self {
        ZIndex(
            layer_index.0 * 6
                + match entity_type {
                    RenderEntityType::Polygon => 0,
                    RenderEntityType::LineString => 1,
                    RenderEntityType::Point => 2,
                    RenderEntityType::SelectedPolygon => 3,
                    RenderEntityType::SelectedLineString => 4,
                    RenderEntityType::SelectedPoint => 5,
                },
        )
    }
}
