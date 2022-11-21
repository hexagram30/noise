use log;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::NoiseFn;
use super::modifiers::{Invert, Threshold};
use super::options::Options;




pub struct Builder<'a> {
    pub noise_fn: &'a dyn NoiseFn<[f64; 3]>,
    pub opts: &'a Options,
    pub noise_map: NoiseMap,
}

impl<'a> Builder<'a> {
    pub fn new(noise_fn: &'a dyn NoiseFn<[f64; 3]>, opts: &'a Options) -> Self {
        Self {
            noise_fn,
            opts,
            noise_map: NoiseMap::new(0, 0),
        }
    }
    fn build(&self, noise_fn: &'a dyn NoiseFn<[f64; 3]>) -> NoiseMap {
        let noise_map = PlaneMapBuilder::new(noise_fn)
            .set_size(self.opts.resolution().x, self.opts.resolution().y)
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
}
