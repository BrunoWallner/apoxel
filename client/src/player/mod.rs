mod camera;
mod plugin;
mod depth;

pub use plugin::PlayerPlugin;


use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    pub last_chunk: [i64; 3],
    pub chunks_moved_since_load: u64,
    pub chunks_moved_since_unload: u64,
}
impl Player {
    pub fn new() -> Self {
        Self {
            last_chunk: [0, 0, 0],
            chunks_moved_since_load: 1,
            chunks_moved_since_unload: 0,
        }
    }
}