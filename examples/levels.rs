extern crate noise;

use hxgm30noise::gen::levels;
use hxgm30noise::options::{LevelsOpts, Options, Resolution};
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

    levels::complex(Options {
        output: "levels-complex-billow-1.png".to_string(),
        levels: LevelsOpts {
            min: -1.0,
            max: 1.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::complex(Options {
        output: "levels-complex-billow-2.png".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::fractured(Options {
        output: "levels-fractured-hm-1.png".to_string(),
        levels: LevelsOpts {
            min: -1.0,
            max: 1.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::fractured(Options {
        output: "levels-fractured-hm-2.png".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::jagged_walls(Options {
        output: "levels-jagged-walls-fbm-1.png".to_string(),
        levels: LevelsOpts {
            min: -1.0,
            max: 1.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::jagged_walls(Options {
        output: "levels-jagged-walls-fbm-2.png".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::linear(Options {
        output: "levels-linear-rm-1.png".to_string(),
        levels: LevelsOpts {
            min: -1.0,
            max: 1.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::linear(Options {
        output: "levels-linear-rm-2.png".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::simple(Options {
        output: "levels-simple-perlin-1.png".to_string(),
        levels: LevelsOpts {
            min: -1.0,
            max: 1.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::simple(Options {
        output: "levels-simple-perlin-2.png".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::wobbly_walls(Options {
        output: "levels-wobbly-walls-turbulence-1.png".to_string(),
        levels: LevelsOpts {
            min: -1.0,
            max: 1.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });

    levels::wobbly_walls(Options {
        output: "levels-wobbly-walls-turbulence-2.png".to_string(),
        levels: LevelsOpts {
            min: -2.0,
            max: 2.0,
            steps: 5,
        },
        is_image,
        res,
        seed,
        ..Default::default()
    });
}
