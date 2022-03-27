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
            .add_plugin(FlyCameraPlugin)
            .add_system(super::depth::update_lighting);
    }
}

use super::Player;

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
        perspective_projection: PerspectiveProjection {
            fov: 1.5,
            ..default()
        },
        ..PerspectiveCameraBundle::new_3d()
    })
    .insert(Player::new())
    .insert(FlyCamera::default());

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::hex("efb615").unwrap(),
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
    mut query: QuerySet<(
        QueryState<(&Transform, &FlyCamera)>,
        QueryState<(&mut Transform, &Light)>
    )>
) {
    let mut player = Transform::from_xyz(0.0, 0.0, 0.0);
    // only one player supported
    for (t, _) in query.q0().iter().next() {
        player = *t;
    }

    for (mut t, _) in query.q1().iter_mut() {
        *t = player
    }
}