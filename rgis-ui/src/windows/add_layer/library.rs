use bevy_egui::egui;
use rgis_geodesy::GeodesyContext;

use super::AddLayerOutput;

pub struct LibraryWidget<'a> {
    pub geodesy_ctx: &'a GeodesyContext,
}

impl LibraryWidget<'_> {
    pub fn show(self, ui: &mut egui::Ui) -> Option<AddLayerOutput> {
        let mut output = None;
        ui.vertical(|ui| {
            ui.heading("Library");
            for folder in rgis_library::get() {
                if let Some(new_output) = show_folder(ui, folder, folder.name, self.geodesy_ctx) {
                    output = Some(new_output);
                }
            }
        });
        output
    }
}

fn show_folder(
    ui: &mut egui::Ui,
    folder: &rgis_library::Folder,
    name_prefix: &str,
    geodesy_ctx: &GeodesyContext,
) -> Option<AddLayerOutput> {
    let mut output = None;
    let folder_header = ui.collapsing(folder.name, |ui| {
        for entry in &folder.entries {
            if let Some(new_output) = show_entry(ui, entry, name_prefix, geodesy_ctx) {
                output = Some(new_output);
            }
        }
        for sub_folder in &folder.sub_folders {
            let sub_prefix = format!("{}: {}", name_prefix, sub_folder.name);
            if let Some(new_output) = show_folder(ui, sub_folder, &sub_prefix, geodesy_ctx) {
                output = Some(new_output);
            }
        }
    });
    crate::widget_registry::register(folder.name, folder_header.header_response.rect);
    output
}

fn show_entry(
    ui: &mut egui::Ui,
    entry: &rgis_library::Entry,
    name_prefix: &str,
    geodesy_ctx: &GeodesyContext,
) -> Option<AddLayerOutput> {
    let mut output = None;
    ui.horizontal(|ui| {
        let add_btn = ui.button("Add");
        crate::widget_registry::register(&format!("Add:{}", entry.name), add_btn.rect);
        if add_btn.clicked() {
            let mut geodesy_ctx = geodesy_ctx.write().unwrap();
            let op_handle =
                rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, entry.crs)
                    .unwrap();
            output = Some(AddLayerOutput::LoadFromLibrary {
                name: format!("{}: {}", name_prefix, entry.name),
                url: entry.url.into(),
                source_crs: rgis_primitives::Crs {
                    epsg_code: Some(entry.crs),
                    proj_string: None,
                    op_handle,
                },
            });
        }
        ui.label(entry.name);
    });
    output
}
