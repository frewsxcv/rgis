use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use rgis_layer_events::LayerStrokeWidthUpdatedEvent;
use rgis_layers::Layers;

pub fn stroke_width(
    egui_ctx: &mut egui::Context,
    layers: &Layers,
    event_writer: &mut EventWriter<LayerStrokeWidthUpdatedEvent>,
) {
    if let Some(layer) = layers.selected_layer() {
        let mut stroke_width = layer.stroke_width;
        if egui::DragValue::new(&mut stroke_width)
            .speed(0.1)
            .clamp_range(0.1..=100.0)
            .ui(egui_ctx)
            .changed()
        {
            event_writer.send(LayerStrokeWidthUpdatedEvent(layer.id, stroke_width));
        }
    }
}
