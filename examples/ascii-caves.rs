extern crate noise;

use hxgm30noise::common::{Opts, Resolution};
use hxgm30noise::gen::caves;
use twyg;

fn main() {
    let log_opts = twyg::LoggerOpts {
        colored: true,
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
    let is_ascii = true;

    caves::complex(&Opts {
        output: "caves-complex-billow.txt",
        threshold_cutoff: -0.25,
        is_ascii,
        res,
        seed,
        ..Default::default()
    });

    caves::fractured(&Opts {
        output: "caves-fractured-hm.txt",
        threshold_cutoff: 0.1,
        is_ascii,
        res,
        seed,
        ..Default::default()
    });

    caves::jagged_walls(&Opts {
        output: "caves-jagged-walls-fbm.txt",
        is_ascii,
        res,
        seed,
        ..Default::default()
    });

    caves::linear(&Opts {
        output: "caves-linear-rm.txt",
        threshold_cutoff: -0.4,
        is_ascii,
        res,
        seed,
        ..Default::default()
    });

    caves::simple(&Opts {
        output: "caves-simple-perlin.txt",
        is_ascii,
        res,
        seed,
        ..Default::default()
    });

    caves::wobbly_walls(&Opts {
        output: "caves-wobbly-walls-turbulence.txt",
        is_ascii,
        res,
        seed,
        ..Default::default()
    });
}
