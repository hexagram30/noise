use crate::common::{ASCIIMapper, Builder};
// use crate::util::clamp;
use noise::utils::NoiseMap;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::{self, path::Path};

// const ASCII_MAX_WIDTH: u8 = 255;
// const ASCII_MAX_HEIGHT: u8 = 255;
// const DEFAULT_CHARS: &str = ".,-:;+=!*0%#".chars();

pub trait ASCIIWriter {
    fn write(&self, filename: &str, mapper: &ASCIIMapper);
}

impl ASCIIWriter for NoiseMap {
    fn write(&self, filename: &str, mapper: &ASCIIMapper) {
        // Create the output directory for the images, if it doesn't already exist
        let directory = "example_images/";
        let target_dir = Path::new(directory);

        if !target_dir.exists() {
            std::fs::create_dir(target_dir).expect("failed to create example_images directory");
        }

        // Concatenate the directory to the filename string
        let file_path = target_dir.join(filename);
        let display = file_path.display();
        let (width, height) = self.size();

        // XXX Ugh! self.map is private :-( (external crate) So we can't do this:
        // let data = self.map.iter().map(|x| format!("{}", x)).collect::<Vec<String>>();
        //
        // Which means we're going to have to do this the hard, inefficient way ...
        //
        // If we just want to use the floats as-is:
        // let mut values: Vec<f64> = Vec::with_capacity(width * height);
        // for x in 0..width {
        //     for y in 0..height {
        //         values.push(self.get_value(x, y));
        //     }
        // }
        //
        // If we want to build everything in place:
        let mut tile = String::from("");
        for y in 0..height {
            for x in 0..width {
                let key = format!("{:.1}", self.get_value(x, y));
                match mapper.lookup.get(&key) {
                    Some(ascii) => tile.push(*ascii),
                    _ => (),
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
        self.noise_map
            .write(&self.opts.output, &self.opts.ascii_mapper);
    }
}
