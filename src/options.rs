use std::collections::HashMap;
use imgdata::color::{self, Color};
use log;
use nu_ansi_term::Color::Rgb;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(unused)]
pub struct Options<'a> {
    pub inverted: bool,
    pub is_cave: bool,
    pub is_image: bool,
    pub is_ascii: bool,
    pub log_level: &'a str,
    pub noise_type: &'a str,
    pub output: &'a str,
    pub seed: u32,
    pub res_str: &'a str,
    pub threshold_cutoff: f64,
    pub threshold_enabled: bool,
    pub tiled: bool,
    pub turbulence: bool,
    pub config_paths: Vec<String>,
    pub res: Resolution,
    pub levels: LevelsOpts,
    pub ascii_mapper: ASCIIMapper,
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
