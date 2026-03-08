use bevy::prelude::*;
use bevy_egui::egui;

/// The different pane types in the tiled layout.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    /// The layers panel (list of loaded layers).
    Layers,
    /// The map viewport (transparent pass-through to Bevy renderer).
    Map,
}

/// Bevy resource holding the egui_tiles tree.
#[derive(Resource)]
pub struct TilesTree(pub egui_tiles::Tree<Pane>);

impl Default for TilesTree {
    fn default() -> Self {
        let mut tiles = egui_tiles::Tiles::default();
        let layers_pane = tiles.insert_pane(Pane::Layers);
        let map_pane = tiles.insert_pane(Pane::Map);
        // Horizontal split: layers on left, map on right
        let root = tiles.insert_horizontal_tile(vec![layers_pane, map_pane]);
        // Set the layers pane to take ~20% of width
        if let Some(egui_tiles::Tile::Container(egui_tiles::Container::Linear(linear))) =
            tiles.get_mut(root)
        {
            linear.shares.set_share(layers_pane, 0.2);
            linear.shares.set_share(map_pane, 0.8);
        }
        Self(egui_tiles::Tree::new("rgis_tiles", root, tiles))
    }
}

/// Behavior implementation that bridges egui_tiles with the rgis UI.
pub struct RgisBehavior<'a> {
    pub snapshots: &'a [crate::panels::side::LayerSnapshot],
    pub events: &'a mut crate::panels::side::Events<'static>,
    pub map_pane_rect: &'a mut rgis_units::MapPaneRect,
}

impl egui_tiles::Behavior<Pane> for RgisBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::Layers => "Layers".into(),
            Pane::Map => "Map".into(),
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::Layers => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Layers");
                    ui.add(crate::widgets::add_layer::AddLayer {
                        events: self.events,
                    });
                    let count = self.snapshots.len();
                    for i in 0..count {
                        let snap = &self.snapshots[i];
                        ui.add(crate::panels::side::Layer {
                            is_move_down_enabled: i < count - 1,
                            is_move_up_enabled: i > 0,
                            layer_id: snap.layer_id,
                            name: &snap.name,
                            visible: snap.visible,
                            color: &snap.color,
                            is_vector: snap.is_vector,
                            is_active: snap.is_active,
                            geom_type: snap.geom_type,
                            crs: &snap.crs,
                            unprojected_fc: snap.unprojected_fc.as_deref(),
                            events: self.events,
                        });
                    }
                });
                egui_tiles::UiResponse::None
            }
            Pane::Map => {
                // Record the map pane rect for camera viewport and input passthrough
                let rect = ui.max_rect();
                *self.map_pane_rect = rgis_units::MapPaneRect {
                    min_x: rect.min.x,
                    min_y: rect.min.y,
                    max_x: rect.max.x,
                    max_y: rect.max.y,
                };
                egui_tiles::UiResponse::None
            }
        }
    }

    fn tab_bar_color(&self, _visuals: &egui::Visuals) -> egui::Color32 {
        egui::Color32::TRANSPARENT
    }

    fn is_tab_closable(&self, _tiles: &egui_tiles::Tiles<Pane>, _tile_id: egui_tiles::TileId) -> bool {
        false
    }

    fn gap_width(&self, _style: &egui::Style) -> f32 {
        2.0
    }
}
