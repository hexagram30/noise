extern crate noise;

use hxgm30noise::gen::base::{Opts, Resolution};
use hxgm30noise::gen::caves;
use twyg;

fn main() {
    let log_opts = twyg::LoggerOpts {
        colored: true,
        file: String::from(""),
        level: String::from("info"),
        report_caller: true,
    };

    match twyg::setup_logger(&log_opts) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };

    let res = Resolution { x: 200, y: 200 };
    let seed = 108;

    caves::complex(&Opts {
        output: &"caves-complex-billow.png".to_string(),
        threshold_cutoff: -0.25,
        res,
        seed,
        ..Default::default()
    });

    caves::fractured(&Opts {
        output: &"caves-fractured-hm.png".to_string(),
        threshold_cutoff: 0.1,
        res,
        seed,
        ..Default::default()
    });

    caves::jagged_walls(&Opts {
        output: &"caves-jagged-walls-fbm.png".to_string(),
        res,
        seed,
        ..Default::default()
    });

    caves::linear(&Opts {
        output: &"caves-linear-rm.png".to_string(),
        threshold_cutoff: -0.4,
        res,
        seed,
        ..Default::default()
    });

    caves::simple(&Opts {
        output: &"caves-simple-perlin.png".to_string(),
        res,
        seed,
        ..Default::default()
    });

    caves::wobbly_walls(&Opts {
        output: &"caves-wobbly-walls-turbulence.png".to_string(),
        res,
        seed,
        ..Default::default()
    });
}
