use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use protocol::chunk::CHUNK_SIZE;

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
	fn build(&self, app: &mut App) {
		app
            .add_system(cursor_grab_system)
			.add_system(camera_movement_system)
			.add_system(mouse_motion_system);
	}
}

#[derive(Component)]
pub struct FlyCamera {
	pub accel: f32,
	pub max_speed: f32,
	pub sensitivity: f32,
	pub friction: f32,
	pub pitch: f32,
	pub yaw: f32,
	pub velocity: Vec3,
	pub key_forward: KeyCode,
	pub key_backward: KeyCode,
	pub key_left: KeyCode,
	pub key_right: KeyCode,
	pub key_up: KeyCode,
	pub key_down: KeyCode,
	pub enabled: bool,
}
impl Default for FlyCamera {
	fn default() -> Self {
		Self {
			accel: 2.0,
			max_speed: 0.5,
			sensitivity: 1.0,
			friction: 1.0,
			pitch: 1.0,
			yaw: 1.0,
			velocity: Vec3::ZERO,
			key_forward: KeyCode::W,
			key_backward: KeyCode::R,
			key_left: KeyCode::A,
			key_right: KeyCode::S,
			key_up: KeyCode::Space,
			key_down: KeyCode::LShift,
			enabled: false,
		}
	}
}

#[derive(PartialEq)]
enum CameraLock {
    Locked,
    Unlocked,
    None,
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut query: Query<&mut FlyCamera>
) {
    let window = windows.get_primary_mut().unwrap();
    let mut locked = CameraLock::None;

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);

        locked = CameraLock::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);

        locked = CameraLock::Unlocked;
    }

    for mut flycam in query.iter_mut() {
        if locked == CameraLock::Locked {
            flycam.enabled = true
        }
        if locked == CameraLock::Unlocked {
            flycam.enabled = false
        }
    }
}

pub fn movement_axis(
	input: &Res<Input<KeyCode>>,
	plus: KeyCode,
	minus: KeyCode,
) -> f32 {
	let mut axis = 0.0;
	if input.pressed(plus) {
		axis += 1.0;
	}
	if input.pressed(minus) {
		axis -= 1.0;
	}
	axis
}

fn forward_vector(rotation: &Quat) -> Vec3 {
	rotation.mul_vec3(Vec3::Z).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
	let f = forward_vector(rotation);
	let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
	f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
	// Rotate it 90 degrees to get the strafe direction
	Quat::from_rotation_y(90.0f32.to_radians())
		.mul_vec3(forward_walk_vector(rotation))
		.normalize()
}

use crate::player::Player;

fn camera_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	//mut query: Query<(&mut FlyCamera, &mut Transform)>,
    mut query: ParamSet<(
        Query<(&mut FlyCamera, &mut Transform)>,
        Query<(&Transform, &mut Player)>
    )>
) {

    for (mut options, mut transform) in query.p0().iter_mut() {
        let (axis_h, axis_v, axis_float) = if options.enabled {
            (
                movement_axis(&keyboard_input, options.key_right, options.key_left),
                movement_axis(
                    &keyboard_input,
                    options.key_backward,
                    options.key_forward,
                ),
                movement_axis(&keyboard_input, options.key_up, options.key_down),
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        let rotation = transform.rotation;
        let accel: Vec3 = (strafe_vector(&rotation) * axis_h)
            + (forward_walk_vector(&rotation) * axis_v)
            + (Vec3::Y * axis_float);
        let accel: Vec3 = if accel.length() != 0.0 {
            accel.normalize() * options.accel
        } else {
            Vec3::ZERO
        };

        let friction: Vec3 = if options.velocity.length() != 0.0 {
            options.velocity.normalize() * -1.0 * options.friction
        } else {
            Vec3::ZERO
        };

        options.velocity += accel * time.delta_seconds();

        // clamp within max speed
        if options.velocity.length() > options.max_speed {
            options.velocity = options.velocity.normalize() * options.max_speed;
        }

        let delta_friction = friction * time.delta_seconds();

        options.velocity = if (options.velocity + delta_friction).signum()
            != options.velocity.signum()
        {
            Vec3::ZERO
        } else {
            options.velocity + delta_friction
        };

        transform.translation += options.velocity;
    }


    // determines player chunk pos necessary for chunk loading
    for (transform, mut player) in query.p1().iter_mut() {
        let player_pos = transform.translation;
        let chunk_x = (player_pos.x / CHUNK_SIZE as f32).round() as i64;
        let chunk_y = (player_pos.y / CHUNK_SIZE as f32).round() as i64;
        let chunk_z = (player_pos.z / CHUNK_SIZE as f32).round() as i64;

        let last = player.last_chunk;

        player.last_chunk = [chunk_x, chunk_y, chunk_z];

        let diff =
            (last[0] - chunk_x).abs() + 
            (last[1] - chunk_y).abs() + 
            (last[2] - chunk_z).abs();

        player.chunks_moved_since_load += diff as u64;
        player.chunks_moved_since_unload += diff as u64;
    }
}

fn mouse_motion_system(
	//time: Res<Time>,
	mut mouse_motion_event_reader: EventReader<MouseMotion>,
	mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut options, mut transform) in query.iter_mut() {
        if !options.enabled {
            continue;
        }
        options.yaw -= delta.x * options.sensitivity * 0.1;
        options.pitch += delta.y * options.sensitivity * 0.1;

        options.pitch = options.pitch.clamp(-89.0, 89.9);
        // println!("pitch: {}, yaw: {}", options.pitch, options.yaw);

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();

        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians)
            * Quat::from_axis_angle(-Vec3::X, pitch_radians);
    }
}