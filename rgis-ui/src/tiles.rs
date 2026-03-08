use bevy::prelude::*;
use bevy_egui::egui;

/// The different pane types available in the tiled layout.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    /// The layers list.
    Layers,
    /// The map view (transparent — Bevy renders behind it).
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
        // Wrap each pane in a tab container so the tab bar is always visible.
        let layers_tab = tiles.insert_tab_tile(vec![layers_pane]);
        let map_tab = tiles.insert_tab_tile(vec![map_pane]);
        let mut linear = egui_tiles::Linear::new(
            egui_tiles::LinearDir::Horizontal,
            vec![layers_tab, map_tab],
        );
        linear.shares.set_share(layers_tab, 0.15);
        linear.shares.set_share(map_tab, 0.85);
        let root = tiles.insert_container(egui_tiles::Container::Linear(linear));
        Self(egui_tiles::Tree::new("rgis_tiles", root, tiles))
    }
}

/// Behavior implementation for the tiles.
pub struct TilesBehavior<'a> {
    pub snapshots: &'a [crate::panels::side::LayerSnapshot],
    pub events: &'a mut crate::panels::side::Events<'static>,
    /// Set by the Map pane during rendering, read afterward to update resources.
    pub map_pane_rect: Option<egui::Rect>,
}

impl egui_tiles::Behavior<Pane> for TilesBehavior<'_> {
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
                // Paint opaque background so the map doesn't show through.
                let bg = ui.visuals().window_fill();
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, bg);

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
            Pane::Map => {
                // Transparent — Bevy renders behind this area.
                self.map_pane_rect = Some(ui.available_rect_before_wrap());
                ui.allocate_space(ui.available_size());
                egui_tiles::UiResponse::None
            }
        }
    }

    fn tab_bar_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        visuals.window_fill()
    }

    fn is_tab_closable(
        &self,
        _tiles: &egui_tiles::Tiles<Pane>,
        _tile_id: egui_tiles::TileId,
    ) -> bool {
        false
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            prune_single_child_tabs: false,
            prune_empty_tabs: false,
            prune_empty_containers: false,
            prune_single_child_containers: false,
            join_nested_linear_containers: false,
        }
    }
}
