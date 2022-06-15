use protocol::chunk::ChunkData;
use protocol::blocks::Block;

const LIGHT_REDUX: f32 = 1.0 / 4.0;

fn get_block(x: usize, y: usize, z: usize, data: &ChunkData) -> Block {
    if let Some(x) = data.get(x) {
        if let Some(y) = x.get(y) {
            if let Some(block) = y.get(z) {
                *block
            } else {
                Block::None
            }
        } else {
            Block::None
        }
    } else {
        Block::None
    }
}

pub(super) fn left(x: usize, y: usize, z: usize, lights: &mut Vec<f32>, data: &ChunkData) {
    let mut light: f32 = 1.0;
    if get_block(x - 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x - 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x - 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x - 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);
}

pub(super) fn right(x: usize, y: usize, z: usize, lights: &mut Vec<f32>, data: &ChunkData) {
    let mut light: f32 = 1.0;
    if get_block(x + 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x + 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x + 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x + 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);
}

pub(super) fn top(x: usize, y: usize, z: usize, lights: &mut Vec<f32>, data: &ChunkData) {
    let mut light: f32 = 1.0;
    if get_block(x, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x - 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x + 1, y + 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);
}

pub(super) fn bottom(x: usize, y: usize, z: usize, lights: &mut Vec<f32>, data: &ChunkData) {
    let mut light: f32 = 1.0;
    if get_block(x, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x - 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x + 1, y - 1, z, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);
}

pub(super) fn front(x: usize, y: usize, z: usize, lights: &mut Vec<f32>, data: &ChunkData) {
    let mut light: f32 = 1.0;
    if get_block(x, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y + 1, z + 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);
}

pub(super) fn back(x: usize, y: usize, z: usize, lights: &mut Vec<f32>, data: &ChunkData) {
    let mut light: f32 = 1.0;
    if get_block(x, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y - 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x - 1, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);

    let mut light: f32 = 1.0;
    if get_block(x, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    if get_block(x + 1, y + 1, z - 1, data).to_category().0 != 0 {light -= LIGHT_REDUX}
    lights.push(light);
}