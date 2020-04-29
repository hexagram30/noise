use crate::modifiers::{Invert, Threshold};
use ansi_term::Colour::RGB;
use imgdata::color::{self, Color};
use log;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::NoiseFn;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Char {
    pub value: f64,
    pub chr: char,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ASCIIMapper {
    pub chars: Vec<Char>,
    pub lookup: HashMap<String, String>,
}

impl ASCIIMapper {
    pub fn new(chars: Vec<Char>) -> Self {
        let mut lookup = HashMap::new();
        for Char { value, chr, color } in chars.iter() {
            let mut chr_str = chr.to_string();
            match color {
                Some(color) => {
                    let c = color::from_hex(color.clone());
                    let [r, g, b] = c.rgb();
                    chr_str = RGB(r, g, b).paint(chr_str.as_str()).to_string();
                }
                None => (),
            }
            let k = format!("{:.2}", value);
            log::debug!("Adding key, value: <{}, {}>", k, chr_str);
            lookup.insert(k, chr_str);
        }
        return ASCIIMapper { chars, lookup };
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct LevelsOpts {
    pub min: f64,
    pub max: f64,
    pub steps: u64,
    // pub precision: u8,
}

impl LevelsOpts {
    pub fn step(&self) -> f64 {
        return (self.max - self.min) / (self.steps as f64);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Opts<'a> {
    pub inverted: bool,
    pub is_cave: bool,
    pub is_image: bool,
    pub is_ascii: bool,
    pub ascii_mapper: ASCIIMapper,
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
}
