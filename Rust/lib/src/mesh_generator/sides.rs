use gdnative::prelude::*;

use super::InternalChunk;
use super::MeshData;

use protocol::chunk::CHUNK_SIZE;

#[derive(Clone)]
pub struct Side {
    pub verts: [Vector3; 4],
    pub normal: Vector3,
    pub indices: PoolArray<i32>
}
impl Side {
    pub fn apply_vertex_position(mut self, position: Vector3) -> Self {
        for mut vert in self.verts.iter_mut() {
            *vert += position;
        }
        self
    }
}

lazy_static!{
    pub static ref LEFT: Side = Side {
        verts: [
            Vector3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: 0.0, y: 0.0, z: 1.0},
            Vector3{x: 0.0, y: 1.0, z: 0.0},
            Vector3{x: 0.0, y: 1.0, z: 1.0},
        ],
        normal: Vector3{x: -1.0, y: 0.0, z: 0.0},
        indices: PoolArray::from_slice(&[0, 2, 1, 1, 2, 3])
    };

    pub static ref RIGHT: Side = Side {
        verts: [
            Vector3{x: 1.0, y: 0.0, z: 1.0},
            Vector3{x: 1.0, y: 0.0, z: 0.0},
            Vector3{x: 1.0, y: 1.0, z: 1.0},
            Vector3{x: 1.0, y: 1.0, z: 0.0},
        ],
        normal: Vector3{x: 1.0, y: 0.0, z: 0.0},
        indices: PoolArray::from_slice(&[0, 2, 1, 1, 2, 3])
    };

    pub static ref BACK: Side = Side {
        verts: [
            Vector3{x: 1.0, y: 0.0, z: 0.0},
            Vector3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: 1.0, y: 1.0, z: 0.0},
            Vector3{x: 0.0, y: 1.0, z: 0.0},
        ],
        normal: Vector3{x: 0.0, y: 0.0, z: -1.0},
        indices: PoolArray::from_slice(&[0, 2, 1, 2, 3, 1])
    };

    pub static ref FRONT: Side = Side {
        verts: [
            Vector3{x: 0.0, y: 0.0, z: 1.0},
            Vector3{x: 1.0, y: 0.0, z: 1.0},
            Vector3{x: 0.0, y: 1.0, z: 1.0},
            Vector3{x: 1.0, y: 1.0, z: 1.0},
        ],
        normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
        indices: PoolArray::from_slice(&[0, 2, 1, 1, 2, 3])
    };

    pub static ref TOP: Side = Side {
        verts: [
            Vector3{x: 0.0, y: 1.0, z: 0.0},
            Vector3{x: 0.0, y: 1.0, z: 1.0},
            Vector3{x: 1.0, y: 1.0, z: 0.0},
            Vector3{x: 1.0, y: 1.0, z: 1.0},
        ],
        normal: Vector3{x: 0.0, y: 1.0, z: 0.0},
        indices: PoolArray::from_slice(&[0, 2, 1, 3, 1, 2])
    };

    pub static ref BOTTOM: Side = Side {
        verts: [
            Vector3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: 0.0, y: 0.0, z: 1.0},
            Vector3{x: 1.0, y: 0.0, z: 0.0},
            Vector3{x: 1.0, y: 0.0, z: 1.0},
        ],
        normal: Vector3{x: 0.0, y: -1.0, z: 0.0},
        indices: PoolArray::from_slice(&[1, 2, 0, 3, 2, 1])
    };
}

pub fn check_left(x: usize, y: usize, z: usize, data: &[[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> bool {
    if x > 0 {
        if data[x - 1][y][z] == 0 {
            return true
        } else {
            return false
        }
    } else {
        return true
    }
}

pub fn check_right(x: usize, y: usize, z: usize, data: &[[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> bool {
    if x < CHUNK_SIZE - 1 {
        if data[x + 1][y][z] == 0 {
            return true
        } else {
            return false
        }
    } else {
        return true
    }
}

pub fn check_front(x: usize, y: usize, z: usize, data: &[[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> bool {
    if z < CHUNK_SIZE - 1 {
        if data[x][y][z + 1] == 0 {
            return true
        } else {
            return false
        }
    } else {
        return true
    }
}

pub fn check_back(x: usize, y: usize, z: usize, data: &[[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> bool {
    if z > 0  {
        if data[x][y][z - 1] == 0 {
            return true
        } else {
            return false
        }
    } else {
        return true
    }
}

pub fn check_top(x: usize, y: usize, z: usize, data: &[[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> bool {
    if y < CHUNK_SIZE - 1  {
        if data[x][y + 1][z] == 0 {
            return true
        } else {
            return false
        }
    } else {
        return true
    }
}

pub fn check_bottom(x: usize, y: usize, z: usize, data: &[[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]) -> bool {
    if y > 0  {
        if data[x][y - 1][z] == 0 {
            return true
        } else {
            return false
        }
    } else {
        return true
    }
}