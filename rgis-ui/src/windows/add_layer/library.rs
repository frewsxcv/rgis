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
                let folder_header = ui.collapsing(format!("📁 {}", folder.name), |ui| {
                    for entry in &folder.entries {
                        if let Some(new_output) = (LibraryEntryWidget {
                            folder,
                            entry,
                            geodesy_ctx: self.geodesy_ctx,
                        })
                        .show(ui)
                        {
                            output = Some(new_output);
                        }
                    }
                });
                crate::widget_registry::register(folder.name, folder_header.header_response.rect);
            }
        });
        output
    }
}

struct LibraryEntryWidget<'a> {
    entry: &'a rgis_library::Entry,
    folder: &'a rgis_library::Folder,
    geodesy_ctx: &'a GeodesyContext,
}

impl LibraryEntryWidget<'_> {
    fn show(self, ui: &mut egui::Ui) -> Option<AddLayerOutput> {
        let mut output = None;
        ui.horizontal(|ui| {
            let add_btn = ui.button("➕ Add");
            crate::widget_registry::register(&format!("Add:{}", self.entry.name), add_btn.rect);
            if add_btn.clicked() {
                let mut geodesy_ctx = self.geodesy_ctx.0.write().unwrap();
                let op_handle =
                    rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, self.entry.crs)
                        .unwrap();
                output = Some(AddLayerOutput::LoadFromLibrary {
                    name: format!("{}: {}", self.folder.name, self.entry.name),
                    url: self.entry.url.into(),
                    source_crs: rgis_primitives::Crs {
                        epsg_code: self.entry.crs,
                        op_handle,
                    },
                });
            }
            ui.label(self.entry.name);
        });
        output
    }
}
