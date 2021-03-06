use noise::{NoiseFn, OpenSimplex, Seedable};

pub struct Noise {
    noise: OpenSimplex,
}
impl Noise {
    pub fn new(seed: u32) -> Self {
        let noise = OpenSimplex::new().set_seed(seed);
        Self { noise }
    }

    pub fn get_2d(&self, coord: [f64; 2], zoom: f64, octaves: u16, offset: f64) -> f64 {
        let mut value: f64 = 0.0;
        for octave in 1..=octaves {
            let c = [
                coord[0] * octave as f64 / zoom + offset,
                coord[1] * octave as f64 / zoom - offset / 3.14,
            ];
            value += self.noise.get(c) / octave as f64;
        }

        value
    }

    pub fn get_3d(&self, coord: [f64; 3], zoom: f64, octaves: u16, offset: f64) -> f64 {
        let mut value: f64 = 0.0;
        for octave in 1..=octaves {
            let c = [
                coord[0] * octave as f64 / zoom + offset,
                coord[1] * octave as f64 / zoom - offset / 3.14,
                coord[2] * octave as f64 / zoom + offset / 5.38,
            ];
            value += self.noise.get(c) / octave as f64;
        }

        value
    }
}
