use bevy::prelude::*;

pub struct Materials {
    pub chunk: Handle<StandardMaterial>,
}

pub fn add(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let block_texture_handle = asset_server.load("textures/blocks.png");

    let chunk = materials.add(StandardMaterial { 
        base_color: Color::rgba(1.0, 1.0, 1.0, 1.0), 
        base_color_texture: Some(block_texture_handle.clone()),
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Mask(0.1),
        ..default() });

    cmds.insert_resource(Materials {
        chunk: chunk,
    }); 
}