use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode, WindowResolution};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_window);
    }
}

fn init_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.resolution = WindowResolution::new(1920.0, 1080.0);
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
}
