use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseMotion},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
struct Player;

fn init_player(mut commands: Commands) {
    let fov = 60.0_f32.to_radians();
    commands
        .spawn((
            Player,
            Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Projection::from(PerspectiveProjection { fov, ..default() }),
            ));
        });
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let camera_sensitivity = Vec2::new(0.003, 0.002);
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
