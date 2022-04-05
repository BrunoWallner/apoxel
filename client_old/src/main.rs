mod communication;
mod event_handle;
mod player;
mod material;
mod meshing;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(event_handle::EventHandlePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(meshing::MeshPlugin)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_startup_system(material::add)
        .run();
}

fn setup(
    mut cmds: Commands,
    mut ambient_light: ResMut<AmbientLight>,
) {
    cmds.insert_resource(WindowDescriptor {
        title: "Broxel".to_string(), 
        width: 1200.0, 
        height: 800.0, 
        ..default()
    });

    ambient_light.brightness = 1.0;

    cmds.insert_resource(ClearColor(
        Color::Rgba {
            red: 0.5,
            green: 0.7,
            blue: 1.0,
            alpha: 1.0,
        }
    ));
}
