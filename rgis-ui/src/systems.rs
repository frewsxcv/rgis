use crate::window;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts, EguiPrimaryContextPass,
};

fn render_bottom_panel(
    mut bevy_egui_ctx: EguiContexts,
    mouse_pos: Res<rgis_mouse::MousePos>,
    target_crs: Res<rgis_crs::TargetCrs>,
    mut open_change_crs_window_event_writer: bevy::ecs::event::EventWriter<
        rgis_events::OpenChangeCrsWindow,
    >,
    mut bottom_panel_height: ResMut<rgis_units::BottomPanelHeight>,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    crate::bottom_panel::BottomPanel {
        egui_ctx: bevy_egui_ctx_mut,
        mouse_pos: &mouse_pos,
        target_crs: &target_crs,
        open_change_crs_window_event_writer: &mut open_change_crs_window_event_writer,
        bottom_panel_height: &mut bottom_panel_height,
    }
    .render();
}

fn render_side_panel(
    mut bevy_egui_ctx: EguiContexts,
    layers: Res<rgis_layers::Layers>,
    mut events: crate::side_panel::Events,
    mut side_panel_width: ResMut<rgis_units::SidePanelWidth>,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };

    crate::side_panel::SidePanel {
        egui_ctx: bevy_egui_ctx_mut,
        layers: &layers,
        events: &mut events,
        side_panel_width: &mut side_panel_width,
    }
    .render();
}

fn handle_open_file_job(
    mut finished_jobs: bevy_jobs::FinishedJobs,
    mut selected_file: ResMut<crate::add_layer_window::SelectedFile>,
) {
    while let Some(outcome) = finished_jobs
        .take_next::<crate::add_layer_window::OpenFileJob>()
        .flatten()
    {
        selected_file.0 = Some(outcome);
    }
}

fn render_manage_layer_window(
    mut state: Local<crate::ManageLayerWindowState>,
    mut bevy_egui_ctx: EguiContexts,
    layers: Res<rgis_layers::Layers>,
    mut color_events: ResMut<bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>>,
    mut show_manage_layer_window_event_reader: bevy::ecs::event::EventReader<
        rgis_events::ShowManageLayerWindowEvent,
    >,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    if let Some(event) = show_manage_layer_window_event_reader.read().last() {
        state.is_visible = true;
        state.layer_id = Some(event.0);
    }

    crate::manage_layer_window::ManageLayerWindow {
        state: &mut state,
        layers: &layers,
        egui_ctx: bevy_egui_ctx_mut,
        color_events: &mut color_events,
    }
    .render();
}

struct IsVisible(pub bool);

impl Default for IsVisible {
    fn default() -> Self {
        IsVisible(true)
    }
}

fn render_add_layer_window(
    mut is_visible: Local<IsVisible>,
    mut selected_file: ResMut<crate::add_layer_window::SelectedFile>,
    mut bevy_egui_ctx: EguiContexts,
    mut job_spawner: bevy_jobs::JobSpawner,
    mut state: Local<crate::add_layer_window::State>,
    mut events: crate::add_layer_window::Events,
    geodesy_ctx: Res<rgis_geodesy::GeodesyContext>,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    if !events.show_add_layer_window_event_reader.is_empty() {
        (*is_visible).0 = true;
    }

    if !events.hide_add_layer_window_events.is_empty() {
        state.reset();
        (*is_visible).0 = false;
    }

    crate::add_layer_window::AddLayerWindow {
        state: &mut state,
        selected_file: &mut selected_file,
        is_visible: &mut (*is_visible).0,
        egui_ctx: bevy_egui_ctx_mut,
        job_spawner: &mut job_spawner,
        events: &mut events,
        geodesy_ctx: &geodesy_ctx,
    }
    .render();
}

fn render_change_crs_window(
    mut is_visible: Local<bool>,
    mut open_change_crs_window_event_reader: bevy::ecs::event::EventReader<
        rgis_events::OpenChangeCrsWindow,
    >,
    target_crs: Res<rgis_crs::TargetCrs>,
    mut bevy_egui_ctx: EguiContexts,
    mut text_field_value: Local<String>,
    mut change_crs_event_writer: bevy::ecs::event::EventWriter<rgis_events::ChangeCrsEvent>,
    mut crs_input_outcome: Local<Option<crate::widgets::crs_input::Outcome>>,
    geodesy_ctx: Res<rgis_geodesy::GeodesyContext>,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    if open_change_crs_window_event_reader.read().next().is_some() {
        *is_visible = true;
    }

    crate::change_crs_window::ChangeCrsWindow {
        is_visible: &mut is_visible,
        egui_ctx: bevy_egui_ctx_mut,
        text_field_value: &mut text_field_value,
        change_crs_event_writer: &mut change_crs_event_writer,
        target_crs: *target_crs,
        crs_input_outcome: &mut crs_input_outcome,
        geodesy_ctx: &geodesy_ctx,
    }
    .render();
}

fn render_feature_properties_window(
    mut state: Local<crate::FeaturePropertiesWindowState>,
    mut bevy_egui_ctx: EguiContexts,
    layers: Res<rgis_layers::Layers>,
    mut render_message_events: ResMut<
        bevy::ecs::event::Events<rgis_events::RenderFeaturePropertiesEvent>,
    >,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    if let Some(event) = render_message_events.drain().last() {
        state.is_visible = true;
        state.layer_id = Some(event.layer_id);
        state.properties = Some(event.properties);
    }

    let Some(layer) = state.layer_id.and_then(|id| layers.get(id)) else {
        return;
    };

    crate::feature_properties_window::FeaturePropertiesWindow {
        state: &mut state,
        layer,
        egui_ctx: bevy_egui_ctx_mut,
    }
    .render();
}

fn render_message_window(
    mut state: Local<crate::MessageWindowState>,
    mut bevy_egui_ctx: EguiContexts,
    mut render_message_events: ResMut<bevy::ecs::event::Events<rgis_events::RenderMessageEvent>>,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    if let Some(event) = render_message_events.drain().last() {
        state.message = Some(event.0);
        state.is_visible = true;
    }

    crate::message_window::MessageWindow {
        state: &mut state,
        egui_ctx: bevy_egui_ctx_mut,
    }
    .render();
}

fn render_operation_window(
    mut state: Local<crate::OperationWindowState>,
    mut events: ResMut<Events<crate::events::OpenOperationWindowEvent>>,
    mut bevy_egui_ctx: EguiContexts,
    create_layer_event_writer: EventWriter<rgis_events::CreateLayerEvent>,
    render_message_event_writer: EventWriter<rgis_events::RenderMessageEvent>,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };

    if let Some(event) = events.drain().last() {
        state.is_visible = true;
        state.operation = Some(event.operation);
        state.feature_collection = Some(event.feature_collection);
    }

    crate::operation_window::OperationWindow {
        egui_ctx: bevy_egui_ctx_mut,
        state: &mut state,
        create_layer_event_writer,
        render_message_event_writer,
    }
    .render();
}

fn render_in_progress(query: Query<&bevy_jobs::InProgressJob>, mut bevy_egui_ctx: EguiContexts) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    let mut in_progress_job_iter: std::iter::Peekable<
        bevy::ecs::query::QueryIter<'_, '_, &bevy_jobs::InProgressJob, ()>,
    > = query.iter().peekable();

    if in_progress_job_iter.peek().is_none() {
        return;
    }

    egui::Window::new("Running jobs")
        .open(&mut true)
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5., -5.])
        .resizable(false)
        .show(bevy_egui_ctx_mut, |ui| {
            for in_progress_job in in_progress_job_iter {
                ui.add(InProgressJobWidget { in_progress_job });
            }
        });
}

struct InProgressJobWidget<'a> {
    in_progress_job: &'a bevy_jobs::InProgressJob,
}

impl Widget for InProgressJobWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { in_progress_job } = self;

        let name = &in_progress_job.name;
        let progress = in_progress_job.progress;

        ui.horizontal(|ui| {
            ui.add(egui::Spinner::new());
            if progress > 0 {
                egui::ProgressBar::new(f32::from(progress) / 100.)
                    .desired_width(200.)
                    .text(format!("Running '{name}'"))
                    .ui(ui);
            } else {
                ui.label(format!("Running '{name}'"));
            }
        })
        .response
    }
}

fn render_top_panel(
    mut bevy_egui_ctx: EguiContexts,
    mut app_exit_events: ResMut<bevy::ecs::event::Events<bevy::app::AppExit>>,
    mut windows: Query<&mut bevy::window::Window, With<PrimaryWindow>>,
    mut app_settings: ResMut<rgis_settings::RgisSettings>,
    mut top_panel_height: ResMut<rgis_units::TopPanelHeight>,
    mut is_debug_window_open: ResMut<
        window::IsWindowOpen<crate::debug_window::DebugWindow<'static, 'static>>,
    >,
) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    let Ok(mut window) = windows.single_mut() else {
        return;
    };

    crate::top_panel::TopPanel {
        egui_ctx: bevy_egui_ctx_mut,
        app_exit_events: &mut app_exit_events,
        window: &mut window,
        app_settings: &mut app_settings,
        top_panel_height: &mut top_panel_height,
        is_debug_window_open: &mut is_debug_window_open,
    }
    .render();
}

fn set_egui_theme(mut bevy_egui_ctx: EguiContexts, mut clear_color: ResMut<ClearColor>) {
    let bevy_egui_ctx_mut = match bevy_egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(e) => {
            bevy::log::error!("{:?}", e);
            return;
        }
    };
    let egui_visuals = match dark_light::detect() {
        Ok(dark_light::Mode::Dark) => egui::Visuals::dark(),
        Ok(dark_light::Mode::Light | dark_light::Mode::Unspecified) => egui::Visuals::light(),
        Err(e) => {
            bevy::log::error!("Error detecting dark/light mode: {:?}", e);
            egui::Visuals::light()
        }
    };
    // Set the background color of the map
    clear_color.0 = egui_color_to_bevy_color(egui_visuals.extreme_bg_color);
    // Set the egui theme
    bevy_egui_ctx_mut.set_visuals(egui_visuals);
}

fn egui_color_to_bevy_color(egui_color: bevy_egui::egui::Color32) -> bevy::color::Color {
    bevy::color::Color::srgb_u8(egui_color.r(), egui_color.g(), egui_color.b())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum RenderSystemSet {
    RenderingMessageWindow,
    RenderingTopBottomPanels,
    SideBarProgressBar,
    Windows,
}

pub fn configure(app: &mut App) {
    app.add_systems(Startup, set_egui_theme);

    app.configure_sets(
        EguiPrimaryContextPass,
        (
            RenderSystemSet::RenderingMessageWindow,
            RenderSystemSet::RenderingTopBottomPanels,
            RenderSystemSet::SideBarProgressBar,
            RenderSystemSet::Windows,
        )
            .chain(),
    );

    app.add_systems(
        EguiPrimaryContextPass,
        (
            render_message_window.in_set(RenderSystemSet::RenderingMessageWindow),
            render_top_panel.in_set(RenderSystemSet::RenderingTopBottomPanels),
            render_bottom_panel.in_set(RenderSystemSet::RenderingTopBottomPanels),
            render_side_panel.in_set(RenderSystemSet::SideBarProgressBar),
            render_in_progress.in_set(RenderSystemSet::SideBarProgressBar),
            handle_open_file_job,
            render_manage_layer_window.in_set(RenderSystemSet::Windows),
            render_add_layer_window.in_set(RenderSystemSet::Windows),
            render_change_crs_window.in_set(RenderSystemSet::Windows),
            render_feature_properties_window.in_set(RenderSystemSet::Windows),
            render_operation_window.in_set(RenderSystemSet::Windows),
        ),
    );

    app.insert_resource(window::IsWindowOpen::<crate::debug_window::DebugWindow>::closed());
    app.insert_resource(window::IsWindowOpen::<crate::welcome_window::WelcomeWindow>::open());
    app.add_systems(
        EguiPrimaryContextPass,
        window::render_window_system::<crate::debug_window::DebugWindow>
            .run_if(window::run_if_is_window_open::<crate::debug_window::DebugWindow>),
    );
    app.add_systems(
        EguiPrimaryContextPass,
        window::render_window_system::<crate::welcome_window::WelcomeWindow>
            .run_if(window::run_if_is_window_open::<crate::welcome_window::WelcomeWindow>),
    );
}
