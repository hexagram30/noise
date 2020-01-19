extern crate noise;

use noise::{Billow, Fbm, HybridMulti, Perlin, RidgedMulti, Turbulence};
use crate::base::{Builder, Opts};

pub fn complex(opts: &Opts) {
    let generator = Billow::new();
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn fractured(opts: &Opts) {
    let generator = HybridMulti::new();
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn jagged_walls(opts: &Opts) {
    let generator = Fbm::new();
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn linear(opts: &Opts) {
    let linear_opts: &Opts = &Opts{
        inverted: true,
        ..*opts
    };
    let generator = RidgedMulti::new();
    let builder = Builder::new(&generator, linear_opts);
    builder.generate()
}

pub fn simple(opts: &Opts) {
    let generator = Perlin::new();
    let builder = Builder::new(&generator, opts);
    builder.generate()
}

pub fn wobbly_walls(opts: &Opts) {
    let generator = Perlin::new();
    let generator = Turbulence::new(&generator);
    let builder = Builder::new(&generator, opts);
    builder.generate()
}
