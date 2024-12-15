use bevy::prelude::*;

pub mod client;

fn main() {
    App::new()
        .add_plugins((client::client::ClientPlugin, DefaultPlugins))
        .run();
}
