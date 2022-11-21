use std::collections::HashMap;
use imgdata::color::{self, Color};
use log;
use nu_ansi_term::Color::Rgb;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Options {
    pub inverted: bool,
    pub is_cave: bool,
    pub is_image: bool,
    pub is_ascii: bool,
    pub log_level: String,
    pub noise_type: String,
    pub output: String,
    pub seed: u32,
    pub res_str: String,
    pub threshold_cutoff: f64,
    pub threshold_enabled: bool,
    pub tiled: bool,
    pub turbulence: bool,
    pub config_paths: Vec<String>,
    pub res: Resolution,
    pub levels: LevelsOpts,
    pub ascii_mapper: ASCIIMapper,
}

impl Options {
    pub fn resolution(&self) -> Resolution {
        if self.res_str == "" {
            return self.res
        }
        let xy: Vec<usize> = self.res_str.split(',').map(|s| s.parse().unwrap()).collect();
        Resolution {
            x: xy[0],
            y: xy[1],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Char {
    pub value: f64,
    pub chr: char,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
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
                    chr_str = Rgb(r, g, b).paint(chr_str.as_str()).to_string();
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
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
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
