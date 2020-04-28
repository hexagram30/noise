use crate::common::Builder;
// use crate::util::clamp;
use noise::utils::NoiseMap;
use std::fs::File;
use std::io::Write;
use std::{self, path::Path};

// const ASCII_MAX_WIDTH: u8 = 255;
// const ASCII_MAX_HEIGHT: u8 = 255;
// const DEFAULT_CHARS: &str = ".,-:;+=!*0%#".chars();

pub trait ASCIIWriter {
    fn write(&self, filename: &str);
}

impl ASCIIWriter for NoiseMap {
    fn write(&self, filename: &str) {
        // Create the output directory for the images, if it doesn't already exist
        let directory = "example_images/";
        let target_dir = Path::new(directory);

        if !target_dir.exists() {
            std::fs::create_dir(target_dir).expect("failed to create example_images directory");
        }

        // Concatenate the directory to the filename string
        let file_path = target_dir.join(filename);
        let display = file_path.display();

        // Collect the values from f64 into u8 in a separate vec
        // let (width, height) = self.size();
        // let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

        // for i in &self.map {
        //     pixels.push((clamp(i * 0.5 + 0.5, 0.0, 1.0) * 255.0) as u8);
        // }

        // let _ = image::save_buffer(
        //     &Path::new(&file_path),
        //     &*pixels,
        //     self.size.0 as u32,
        //     self.size.1 as u32,
        //     image::ColorType::L8,
        // );
        let mut file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
            Ok(file) => file,
        };
        match file.write_all("XXX".as_bytes()) {
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
        self.noise_map.write(&self.opts.output);
    }
}
