use noise::{NoiseFn};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

#[derive(Clone, Copy, Default)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Opts {
    pub inverted: bool,
    pub output: String,
    pub seed: i64,
    pub res: Resolution,
    pub res_str: String,
    pub threshold_cutoff: f64,
    pub tiled: bool,
    pub turbulence: bool,
}

pub struct Builder<'a> {
    pub noise_fn: &'a dyn NoiseFn<[f64; 3]>,
    pub opts: &'a Opts,
}

impl <'a> Builder<'a> {
    pub fn new(noise_fn: &'a dyn NoiseFn<[f64; 3]>, opts: &'a Opts) -> Self {
        Self {
            noise_fn,
            opts,
        }
    }

    pub fn generate(&self) {
        PlaneMapBuilder::new(self.noise_fn)
            .set_size(self.opts.res.x, self.opts.res.y)
            .set_is_seamless(self.opts.tiled)
            .build()
            .write_to_file(&self.opts.output);
    }
}
