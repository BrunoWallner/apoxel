use bevy::prelude::*;
use super::camera::{FlyCamera, FlyCameraPlugin};

#[derive(Component)]
pub struct Light;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(update_pos)
            .add_plugin(FlyCameraPlugin);
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
}

use crate::communication::Communicator;
use protocol::event::Event as TcpEvent;

// sends local player pos to server
fn update_pos(
    query: Query<&Transform, With<FlyCamera>>,
    communicator: Res<Communicator>
) {
    for player in query.iter() {
        let pos = player.translation;
        communicator.event_bridge.push_tcp(TcpEvent::MovePlayer( [pos.x as f64, pos.y as f64, pos.z as f64] ));
    }
}