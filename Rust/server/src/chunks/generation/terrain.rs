use super::noise::Noise;
use splines::{Interpolation, Key, Spline};

const ZOOM: f64 = 500.0;
const OCTAVES: u16 = 16;
const AMPLITUDE: f64 = 80.0;

pub enum Biome {
    Planes,
}
// TODO: 2d vec of biomes, biome determined by both humidity and temp

pub struct TerrainGen {
    noise: Noise,
}
impl TerrainGen {
    pub fn new(seed: u32) -> Self {
        let noise = Noise::new(seed);

        Self { noise }
    }

    pub fn get(&self, coord: [i64; 2]) -> (i64, Biome) {
        let mountain_spline = get_mountain_spline();

        // range 0..1
        let mountain = (self
            .noise
            .get_2d([coord[0] as f64, coord[1] as f64], ZOOM, OCTAVES, 0.0)
            + 1.0)
            / 2.0;

        let height = mountain_spline.clamped_sample(mountain).unwrap() * AMPLITUDE;

        (height as i64, Biome::Planes)
    }
}

fn get_mountain_spline() -> Spline<f64, f64> {
    Spline::from_vec(vec![
        Key::new(0.0, 0.1, Interpolation::Linear),
        Key::new(0.25, 0.4, Interpolation::Linear),
        Key::new(0.5, 0.5, Interpolation::Linear),
        Key::new(0.75, 0.7, Interpolation::Linear),
        Key::new(1.0, 0.75, Interpolation::Linear),
    ])
}
