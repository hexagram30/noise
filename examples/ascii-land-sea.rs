extern crate noise;

use hxgm30noise::options::{ASCIIMapper, Char, LevelsOpts, Options, Resolution};
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
    caves::complex(Options {
        output: "land-sea-complex-billow.txt".to_string(),
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
    caves::fractured(Options {
        output: "land-sea-fractured-hm.txt".to_string(),
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
    caves::jagged_walls(Options {
        output: "land-sea-jagged-walls-fbm.txt".to_string(),
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::linear(Options {
        output: "land-sea-linear-rm.txt".to_string(),
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
    caves::simple(Options {
        output: "land-sea-simple-perlin.txt".to_string(),
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    let levels = ls.clone();
    caves::wobbly_walls(Options {
        output: "land-sea-wobbly-walls-turbulence.txt".to_string(),
        is_ascii,
        ascii_mapper,
        levels,
        res,
        seed,
        ..Default::default()
    });
}
