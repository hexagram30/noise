extern crate noise;

use hxgm30noise::common::{ASCIIMapper, Char, LevelsOpts, Opts, Resolution};
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

    let res = Resolution { x: 48, y: 16 };
    let seed = 108;
    let is_ascii = true;
    let am = ASCIIMapper::new(vec![
        Char {
            value: 1.0,
            chr: '^',
            color: Some(String::from("#00aa00")),
        },
        Char {
            value: -1.0,
            chr: '~',
            color: Some(String::from("#0000aa")),
        },
    ]);
    let ls = LevelsOpts {
        min: -1.0,
        max: 1.0,
        steps: 1,
    };

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::complex(Opts {
        output: "land-sea-complex-billow.txt",
        threshold_cutoff: -0.25,
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::fractured(Opts {
        output: "land-sea-fractured-hm.txt",
        threshold_cutoff: 0.1,
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::jagged_walls(Opts {
        output: "land-sea-jagged-walls-fbm.txt",
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::linear(Opts {
        output: "land-sea-linear-rm.txt",
        threshold_cutoff: -0.4,
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::simple(Opts {
        output: "land-sea-simple-perlin.txt",
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::wobbly_walls(Opts {
        output: "land-sea-wobbly-walls-turbulence.txt",
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });
}
