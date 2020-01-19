extern crate noise;

use noise::utils::*;
use noise::{Billow, Fbm, HybridMulti, Perlin, RidgedMulti, Turbulence};
use crate::modifiers::{Invert, Threshold};

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
    pub size_str: String,
    pub threshold_cutoff: f64,
    pub tiled: bool,
    pub turbulence: bool,
}

pub fn complex(opts: Opts) {
    let generator = Billow::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    PlaneMapBuilder::new(&generator)
        .set_size(opts.res.x, opts.res.y)
        .set_is_seamless(opts.tiled)
        .build()
        .write_to_file(&opts.output);
}

pub fn fractured(opts: Opts) {
    let generator = HybridMulti::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    PlaneMapBuilder::new(&generator)
        .set_size(opts.res.x, opts.res.y)
        .set_is_seamless(opts.tiled)
        .build()
        .write_to_file(&opts.output);
}

pub fn jagged_walls(opts: Opts) {
    let generator = Fbm::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    PlaneMapBuilder::new(&generator)
        .set_size(opts.res.x, opts.res.y)
        .set_is_seamless(opts.tiled)
        .build()
        .write_to_file(&opts.output);
}

pub fn linear(opts: Opts) {
    let generator = RidgedMulti::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    let generator = Invert::new(&generator);
    PlaneMapBuilder::new(&generator)
        .set_size(opts.res.x, opts.res.y)
        .set_is_seamless(opts.tiled)
        .build()
        .write_to_file(&opts.output);
}

pub fn simple(opts: Opts) {
    let generator = Perlin::new();
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    PlaneMapBuilder::new(&generator)
        .set_size(opts.res.x, opts.res.y)
        .set_is_seamless(opts.tiled)
        .build()
        .write_to_file(&opts.output);
}

pub fn wobbly_walls(opts: Opts) {
    let generator = Perlin::new();
    let generator = Turbulence::new(&generator);
    let generator = Threshold::new(&generator)
        .set_cutoff(opts.threshold_cutoff);
    PlaneMapBuilder::new(&generator)
        .set_size(opts.res.x, opts.res.y)
        .set_is_seamless(opts.tiled)
        .build()
        .write_to_file(&opts.output);
}
