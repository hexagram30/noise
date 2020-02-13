use crate::modifiers::{Invert, Threshold};
use log;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::NoiseFn;

#[derive(Clone, Copy, Default)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Opts<'a> {
    pub inverted: bool,
    pub is_cave: bool,
    pub noise_type: &'a str,
    pub output: &'a str,
    pub seed: u32,
    pub res: Resolution,
    pub res_str: &'a str,
    pub threshold_cutoff: f64,
    pub threshold_enabled: bool,
    pub tiled: bool,
    pub turbulence: bool,
}

pub struct Builder<'a> {
    pub noise_fn: &'a dyn NoiseFn<[f64; 3]>,
    pub opts: &'a Opts<'a>,
}

impl<'a> Builder<'a> {
    pub fn new(noise_fn: &'a dyn NoiseFn<[f64; 3]>, opts: &'a Opts) -> Self {
        Self { noise_fn, opts }
    }
    fn build(&self, noise_fn: &'a dyn NoiseFn<[f64; 3]>) -> NoiseMap {
        let noise_map = PlaneMapBuilder::new(noise_fn)
            .set_size(self.opts.res.x, self.opts.res.y)
            .set_is_seamless(self.opts.tiled)
            .build();
        return noise_map;
    }
    pub fn generate(&self) {
        log::debug!("Generating ...");
        // Permute options
        if self.opts.threshold_enabled && self.opts.inverted {
            let thresh = Threshold::new(self.noise_fn).set_cutoff(self.opts.threshold_cutoff);
            let invert = Invert::new(&thresh);
            let noise_map = self.build(&invert);
            noise_map.write_to_file(&self.opts.output);
        } else if self.opts.inverted {
            let invert = Invert::new(self.noise_fn);
            let noise_map = self.build(&invert);
            noise_map.write_to_file(&self.opts.output);
        } else if self.opts.threshold_enabled {
            let thresh = Threshold::new(self.noise_fn).set_cutoff(self.opts.threshold_cutoff);
            let noise_map = self.build(&thresh);
            noise_map.write_to_file(&self.opts.output);
        } else {
            let noise_map = self.build(self.noise_fn);
            noise_map.write_to_file(&self.opts.output);
        }
    }
}
