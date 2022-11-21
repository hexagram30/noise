extern crate noise;

use hxgm30noise::options::{Options, Resolution};
use hxgm30noise::gen::caves;
use twyg;

fn main() {
    let log_opts = twyg::LoggerOpts {
        coloured: true,
        file: None,
        level: "info".to_string(),
        report_caller: true,
    };

    match twyg::setup_logger(&log_opts) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };

    let res = Resolution { x: 200, y: 200 };
    let seed = 108;
    let is_image = true;

    caves::complex(Options {
        output: "caves-complex-billow.png".to_string(),
        threshold_cutoff: -0.25,
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::fractured(Options {
        output: "caves-fractured-hm.png".to_string(),
        threshold_cutoff: 0.1,
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::jagged_walls(Options {
        output: "caves-jagged-walls-fbm.png".to_string(),
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::linear(Options {
        output: "caves-linear-rm.png".to_string(),
        threshold_cutoff: -0.4,
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::simple(Options {
        output: "caves-simple-perlin.png".to_string(),
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::wobbly_walls(Options {
        output: "caves-wobbly-walls-turbulence.png".to_string(),
        is_image,
        res,
        seed,
        ..Default::default()
    });
}
