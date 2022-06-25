use super::noise::Noise;
use protocol::blocks::Block;

const ZOOM: f64 = 150.0;
const OCTAVES: u16 = 8;

pub struct CaveGen {
    noise: Noise,
}
impl CaveGen {
    pub fn new(seed: u32) -> Self {
        let noise = Noise::new(seed);

        Self { noise }
    }

    pub fn get(&self, coord: [i64; 3]) -> Block {
        // range 0..1
        let noise = (self
            .noise
            .get_3d([coord[0] as f64, coord[1] as f64, coord[2] as f64], ZOOM, OCTAVES, 0.0)
            + 1.0)
            / 2.0;

        let barrier: f64 = 0.55 + ((coord[1] as f64 - 32.0) / 32.0 / 2.0).clamp(0.0, 0.45);

        match noise {
            _ if noise > barrier => Block::Air,
            _ => Block::None,
        }
    }
}