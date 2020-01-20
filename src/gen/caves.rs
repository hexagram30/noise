extern crate noise;

use noise::{Billow, Fbm, HybridMulti, Perlin, RidgedMulti, Turbulence};
use crate::base::{Builder, Opts};

pub fn complex(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        noise_type: &"billow".to_string(),
        threshold_enabled: true,
        ..*opts
    };
    let generator = Billow::new();
    let builder = Builder::new(&generator, cave_opts);
    builder.generate()
}

pub fn fractured(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        noise_type: &"hybrid-multi".to_string(),
        threshold_enabled: true,
        ..*opts
    };
    let generator = HybridMulti::new();
    let builder = Builder::new(&generator, cave_opts);
    builder.generate()
}

pub fn jagged_walls(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        noise_type: &"fbm".to_string(),
        threshold_enabled: true,
        ..*opts
    };
    let generator = Fbm::new();
    let builder = Builder::new(&generator, cave_opts);
    builder.generate()
}

pub fn linear(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        inverted: true,
        noise_type: &"ridge-multi".to_string(),
        threshold_enabled: true,
        ..*opts
    };
    let generator = RidgedMulti::new();
    let builder = Builder::new(&generator, cave_opts);
    builder.generate()
}

pub fn simple(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        noise_type: &"perlin".to_string(),
        threshold_enabled: true,
        ..*opts
    };
    let generator = Perlin::new();
    let builder = Builder::new(&generator, cave_opts);
    builder.generate()
}

pub fn wobbly_walls(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        noise_type: &"perlin".to_string(),
        threshold_enabled: true,
        turbulence: true,
        ..*opts
    };
    let generator = Perlin::new();
    let generator = Turbulence::new(&generator);
    let builder = Builder::new(&generator, cave_opts);
    builder.generate()
}
