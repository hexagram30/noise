use crate::modifiers::{Invert, Threshold};
use log;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::NoiseFn;

#[derive(Clone, Copy, Debug, Default)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LevelsOpts {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

#[derive(Debug, Default)]
pub struct Opts<'a> {
    pub inverted: bool,
    pub is_cave: bool,
    pub levels: LevelsOpts,
    pub log_level: &'a str,
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
    pub noise_map: NoiseMap,
}

impl<'a> Builder<'a> {
    pub fn new(noise_fn: &'a dyn NoiseFn<[f64; 3]>, opts: &'a Opts) -> Self {
        Self {
            noise_fn,
            opts,
            noise_map: NoiseMap::new(0, 0),
        }
    }
    fn build(&self, noise_fn: &'a dyn NoiseFn<[f64; 3]>) -> NoiseMap {
        let noise_map = PlaneMapBuilder::new(noise_fn)
            .set_size(self.opts.res.x, self.opts.res.y)
            .set_is_seamless(self.opts.tiled)
            .build();
        return noise_map;
    }
    pub fn generate(&mut self) {
        log::debug!("Generating ...");
        // Permute options
        if self.opts.threshold_enabled && self.opts.inverted {
            let thresh = Threshold::new(self.noise_fn).set_cutoff(self.opts.threshold_cutoff);
            let invert = Invert::new(&thresh);
            self.noise_map = self.build(&invert);
        } else if self.opts.inverted {
            let invert = Invert::new(self.noise_fn);
            self.noise_map = self.build(&invert);
        } else if self.opts.threshold_enabled {
            let thresh = Threshold::new(self.noise_fn).set_cutoff(self.opts.threshold_cutoff);
            self.noise_map = self.build(&thresh);
        } else {
            self.noise_map = self.build(self.noise_fn);
        }
    }

    pub fn write_image(&self) {
        self.noise_map.write_to_file(&self.opts.output);
    }
}
