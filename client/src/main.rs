mod communication;
mod event_handle;
mod player;
mod material;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(event_handle::EventHandlePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_startup_system(material::add)
        .run();
}

fn setup(
    mut cmds: Commands,
) {
    cmds.insert_resource(WindowDescriptor {
        title: "Broxel".to_string(), 
        width: 1200.0, 
        height: 800.0, 
        ..default()
    });
}
