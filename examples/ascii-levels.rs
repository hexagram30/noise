extern crate noise;

use hxgm30noise::options::{ASCIIMapper, Char, LevelsOpts, Options, Resolution};
use hxgm30noise::gen::levels;
use twyg;

fn main() {
    let log_opts = twyg::LoggerOpts {
        coloured: true,
        file: None,
        level: "warn".to_string(),
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
            value: 2.0,
            chr: '#',
            color: Some(String::from("#794c2f")),
        },
        Char {
            value: 1.0,
            chr: '*',
            color: Some(String::from("#29254")),
        },
        Char {
            value: 0.0,
            chr: '=',
            color: Some(String::from("#d59268")),
        },
        Char {
            value: -1.0,
            chr: ';',
            color: Some(String::from("#3c639a")),
        },
        Char {
            value: -2.0,
            chr: '.',
            color: Some(String::from("#d7d8ee")),
        },
    ]);

    let ascii_mapper = am.clone();
    levels::complex(Options {
        output: "levels-complex-billow.txt".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_ascii,
        ascii_mapper,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    levels::fractured(Options {
        output: "levels-fractured-hm.txt".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_ascii,
        ascii_mapper,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    levels::jagged_walls(Options {
        output: "levels-jagged-walls-fbm.txt".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_ascii,
        ascii_mapper,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    levels::linear(Options {
        output: "levels-linear-rm.txt".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_ascii,
        ascii_mapper,
        res,
        seed,
        ..Default::default()
    });

    let ascii_mapper = am.clone();
    levels::simple(Options {
        output: "levels-simple-perlin.txt".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_ascii,
        ascii_mapper,
        res,
        seed,
        ..Default::default()
    });

    let am = ASCIIMapper::new(vec![
        Char {
            value: 1.0,
            chr: '#',
            color: Some(String::from("#794c2f")),
        },
        Char {
            value: 0.5,
            chr: '*',
            color: Some(String::from("#29254")),
        },
        Char {
            value: 0.0,
            chr: '=',
            color: Some(String::from("#d59268")),
        },
        Char {
            value: -0.5,
            chr: ';',
            color: Some(String::from("#3c639a")),
        },
        Char {
            value: -1.0,
            chr: '.',
            color: Some(String::from("#d7d8ee")),
        },
    ]);
    let ascii_mapper = am.clone();
    levels::wobbly_walls(Options {
        output: "levels-wobbly-walls-turbulence.txt".to_string(),
        levels: LevelsOpts {
            min: -1.0,
            max: 1.0,
            steps: 5,
        },
        is_ascii,
        ascii_mapper,
        res,
        seed,
        ..Default::default()
    });
}
