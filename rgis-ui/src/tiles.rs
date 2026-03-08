use bevy::prelude::*;
use bevy_egui::egui;

/// The different pane types available in the side panel's tiled layout.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    /// The layers list.
    Layers,
}

/// Bevy resource holding the egui_tiles tree for the side panel.
#[derive(Resource)]
pub struct TilesTree(pub egui_tiles::Tree<Pane>);

impl Default for TilesTree {
    fn default() -> Self {
        let mut tiles = egui_tiles::Tiles::default();
        let layers_pane = tiles.insert_pane(Pane::Layers);
        // For now, a single tab. Future panes (Properties, Attribute Table, etc.)
        // can be added as additional tabs.
        let root = tiles.insert_tab_tile(vec![layers_pane]);
        Self(egui_tiles::Tree::new("rgis_side_tiles", root, tiles))
    }
}

/// Behavior implementation for the side panel tiles.
pub struct SidePanelBehavior<'a> {
    pub snapshots: &'a [crate::panels::side::LayerSnapshot],
    pub events: &'a mut crate::panels::side::Events<'static>,
}

impl egui_tiles::Behavior<Pane> for SidePanelBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::Layers => "Layers".into(),
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
                ui.vertical_centered_justified(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
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
                });
                egui_tiles::UiResponse::None
            }
        }
    }

    fn is_tab_closable(&self, _tiles: &egui_tiles::Tiles<Pane>, _tile_id: egui_tiles::TileId) -> bool {
        false
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }
}
