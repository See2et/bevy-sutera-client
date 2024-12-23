use bevy::prelude::*;

use super::{
    cursor::cursor::CursorPlugin, player::player::PlayerPlugin, scene::scene::ScenePlugin,
    window::window::WindowPlugin,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPlugin)
            .add_plugins(ScenePlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(PlayerPlugin);
    }
}
