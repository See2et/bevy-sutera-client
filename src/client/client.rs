use bevy::prelude::*;

use super::{scene::scene::ScenePlugin, window::window::WindowPlugin};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPlugin).add_plugins(ScenePlugin);
    }
}
