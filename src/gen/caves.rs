extern crate noise;

use noise::{Billow, Fbm, HybridMulti, Perlin, RidgedMulti, Turbulence};
use crate::base::{Builder, Opts};
use crate::modifiers::{Invert, Threshold};

pub fn complex(opts: &Opts) {
    let generator = Billow::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn fractured(opts: &Opts) {
    let generator = HybridMulti::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn jagged_walls(opts: &Opts) {
    let generator = Fbm::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn linear(opts: &Opts) {
    let generator = RidgedMulti::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    let generator = Invert::new(&generator);
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn simple(opts: &Opts) {
    let generator = Perlin::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn wobbly_walls(opts: &Opts) {
    let generator = Perlin::new();
    let generator = Turbulence::new(&generator);
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    let builder = Builder::new(&generator, opts);
    builder.generate()
}
