use noise::{Billow, Fbm, HybridMulti, NoiseFn, Perlin, RidgedMulti, Turbulence};
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use crate::modifiers::{Invert, Threshold};

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
    pub seed: i64,
    pub res: Resolution,
    pub res_str: &'a str,
    pub threshold_cutoff: f64,
    pub threshold_enabled: bool,
    pub tiled: bool,
    pub turbulence: bool,

}

pub struct Builder<'a, T: 'a> {
    pub noise_map: NoiseMap,
    pub opts: &'a Opts<'a>,
    pub noise_fn: dyn NoiseFn<T>,
}

impl<'a, T> Builder<'a, T> {
    pub fn new(opts: &Opts) -> Self {
        Self {
            noise_fn: Perlin::new(),
            noise_map: NoiseMap::new(opts.res.x, opts.res.y),
            opts,
        }
    }
    fn build(&mut self) -> Self {
        self.noise_map = PlaneMapBuilder::new(&self.noise_fn)
            .set_size(self.opts.res.x, self.opts.res.y)
            .set_is_seamless(self.opts.tiled)
            .build();
    }
    fn process_noise_type(&mut self) -> Self {
        match self.noise_type {
            // XXX can't do this in Rust ...
            "billow" => self.noise_fn = Billow::new(),
            "fractured" => self.noise_fn = HybridMulti::new(),
            "jagged_walls" => self.noise_fn = Fbm::new(),
            "linear" => self.noise_fn = RidgedMulti::new(),
            // both simple and wobbly_walls use Perlin
            _ => self.noise_fn = Perlin::new(),
        }
    }
    fn process_invert(&mut self) -> Self {
        if self.opts.threshold_enabled {
            self.noise_fn = Invert::new(self.noise_fn);
        }
    }
    fn process_thredhold(&mut self) -> Self {
        if self.opts.threshold_enabled {
            self.noise_fn = Threshold::new(self.noise_fn)
                .set_cutoff(self.opts.threshold_cutoff);
        }
    }
    fn process_turbulence(&mut self) -> Self {
        if self.opts.turbulence {
            self.noise_fn = Turbulence::new(self.noise_fn);
        }
        return self
    }
    pub fn generate(&self) -> Self {
        self.process_turbulence();
        self.process_thredhold();
        self.process_invert();
        self.build();
        self.noise_map.write_to_file(&self.opts.output);
    }
}

fn lookup_noise_fn(noise_type: &str) {

}
