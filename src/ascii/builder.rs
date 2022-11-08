use crate::common::{Builder, Opts};
use crate::util::{binary_search, FloatIterator};
use log;
use noise::utils::NoiseMap;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::{self, path::Path};

pub trait ASCIIWriter {
    fn write(&self, filename: &str, opts: &Opts);
}

impl ASCIIWriter for NoiseMap {
    fn write(&self, filename: &str, opts: &Opts) {
        log::debug!("Writing ASCII cave file ...");
        let ranges = FloatIterator::new(opts.levels.min, opts.levels.max, opts.levels.steps)
            .collect::<Vec<f64>>();
        log::trace!("Ranges: {:?}", ranges);
        let mapper = &opts.ascii_mapper;
        // Create the output directory for the images, if it doesn't already exist
        let directory = "example_images/";
        let target_dir = Path::new(directory);

        if !target_dir.exists() {
            std::fs::create_dir(target_dir).expect("failed to create example_images directory");
        }

        // Concatenate the directory to the filename string
        let file_path = target_dir.join(filename);
        let display = file_path.display();
        // Build an ASCII grid that will be saved to a file
        let (width, height) = self.size();
        let mut tile = String::from("");
        for y in 0..height {
            for x in 0..width {
                let key = binary_search(&ranges, self.get_value(x, y));
                if key < opts.levels.min {
                    log::error!("Key ({}) is less than min ({})!", key, opts.levels.min);
                }
                if key > opts.levels.max {
                    log::error!("Key ({}) is greater than max ({})!", key, opts.levels.max);
                }
                let str_key = format!("{:.2}", key);
                log::trace!("Lookup: {:?}", mapper.lookup);
                match mapper.lookup.get(&str_key) {
                    Some(ascii) => {
                        log::debug!(
                            "Processing coord: <{}, {}>; key: {}; ascii: {}",
                            x,
                            y,
                            key,
                            ascii
                        );
                        tile.push_str(ascii);
                    }
                    _ => log::warn!("Didn't find match for {} at <{}, {}> ...", str_key, x, y),
                }
            }
            tile.push('\n');
        }

        let file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
            Ok(file) => file,
        };
        let mut file = LineWriter::new(file);
        match file.write_all(tile.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
            Ok(_) => println!("\nFinished generating {}", filename),
        }
    }
}

pub trait BuilderWriter {
    fn write(&self);
}

impl BuilderWriter for Builder<'_> {
    fn write(&self) {
        self.noise_map.write(&self.opts.output, &self.opts);
    }
}
