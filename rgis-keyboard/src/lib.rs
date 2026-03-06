mod systems;

/// Configuration for the keyboard input plugin.
pub struct Plugin {
    /// How many units the camera pans per arrow key press.
    pub pan_amount: f32,
}

impl Default for Plugin {
    fn default() -> Self {
        Self { pan_amount: 15. }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(systems::PanAmount(self.pan_amount));
        systems::configure(app);
    }
}
