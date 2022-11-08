extern crate noise;

use hxgm30noise::common::{Opts, Resolution};
use hxgm30noise::gen::caves;
use twyg;

fn main() {
    let log_opts = twyg::LoggerOpts {
        coloured: true,
        file: None,
        level: String::from("info"),
        report_caller: true,
    };

    match twyg::setup_logger(&log_opts) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };

    let res = Resolution { x: 200, y: 200 };
    let seed = 108;
    let is_image = true;

    caves::complex(Opts {
        output: "caves-complex-billow.png",
        threshold_cutoff: -0.25,
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::fractured(Opts {
        output: "caves-fractured-hm.png",
        threshold_cutoff: 0.1,
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::jagged_walls(Opts {
        output: "caves-jagged-walls-fbm.png",
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::linear(Opts {
        output: "caves-linear-rm.png",
        threshold_cutoff: -0.4,
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::simple(Opts {
        output: "caves-simple-perlin.png",
        is_image,
        res,
        seed,
        ..Default::default()
    });

    caves::wobbly_walls(Opts {
        output: "caves-wobbly-walls-turbulence.png",
        is_image,
        res,
        seed,
        ..Default::default()
    });
}
