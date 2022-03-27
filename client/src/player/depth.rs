use bevy::prelude::*;
use super::camera::FlyCamera;


pub fn update_lighting(
    camera: Query<(&Transform, &FlyCamera)>,
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
) {
    for (_transform, _) in camera.iter().next() {
        //let mut y = transform.translation.y;
        //if y < 0.0 {y = 0.0}

        let sky = [0.0, 0.6, 0.82];
        //let mut sky_mul: f32 = y / 60.0;
        //if sky_mul > 1.0 {sky_mul = 1.0}
        let sky_mul = 1.0;

        commands.insert_resource(ClearColor(
            Color::Rgba {
                red: sky[0] * sky_mul,
                green: sky[1] * sky_mul,
                blue: sky[2] * sky_mul,
                alpha: 1.0,
            }
        ));
        ambient_light.brightness = 1.0;
    }
}