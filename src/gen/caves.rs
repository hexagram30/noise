extern crate noise;

use crate::base::{Builder, Opts};

pub fn complex(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        is_cave: true,
        noise_type: &"billow".to_string(),
        ..*opts
    };
    let builder = Builder::new(cave_opts);
    builder.generate();
}

pub fn fractured(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        is_cave: true,
        noise_type: &"fractured".to_string(),
        ..*opts
    };
    let builder = Builder::new(cave_opts);
    builder.generate();
}

pub fn jagged_walls(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        is_cave: true,
        noise_type: &"jagged_walls".to_string(),
        ..*opts
    };
    let builder = Builder::new(cave_opts);
    builder.generate();
}

pub fn linear(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        inverted: true,
        is_cave: true,
        ..*opts
    };
    let builder = Builder::new(cave_opts);
    builder.generate();
}

pub fn simple(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        is_cave: true,
        ..*opts
    };
    let builder = Builder::new(cave_opts);
    builder.generate();
}

pub fn wobbly_walls(opts: &Opts) {
    let cave_opts: &Opts = &Opts{
        is_cave: true,
        turbulence: true,
        ..*opts
    };
    let builder = Builder::new(cave_opts);
    builder.generate();
}
