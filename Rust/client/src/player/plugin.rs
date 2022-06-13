use bevy::prelude::*;
use super::camera::{FlyCamera, FlyCameraPlugin};

#[derive(Component)]
pub struct Light;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(light_system)
            .add_plugin(FlyCameraPlugin);
    }
}

use super::Player;

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
        ..default()
    })
    .insert(Player::new())
    .insert(FlyCamera::default());

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::hex("FFFFFF").unwrap(),
            range: 500000.0,
            radius: 1.0,
            intensity: 10000.0,
            ..default()
        },
        ..default()
    })
    .insert(Light);
}

fn light_system(
    mut query: ParamSet<(
        Query<(&Transform, &FlyCamera)>,
        Query<(&mut Transform, &Light)>
    )>
) {
    let mut player = Transform::from_xyz(0.0, 0.0, 0.0);
    // only one player supported
    for (t, _) in query.p0().iter().next() {
        player = *t;
    }

    for (mut t, _) in query.p1().iter_mut() {
        *t = player
    }
}