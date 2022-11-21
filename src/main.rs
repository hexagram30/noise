#[macro_use]
extern crate clap;

use clap::{App, Arg};
use log;

use confyg::{Confygery, conf};
use twyg;

use hxgm30noise::common::{Opts, Resolution};
use hxgm30noise::gen::caves;

fn main() {
    // Default values /////////////////////////////////////////
    let default_opts: Opts = Opts {
        log_level: "debug",
        output: "/tmp/file.png",
        res_str: "100,100",
        seed: 108,
        config_paths: vec![
            "./config".to_string(),
            "~/.config/hxgm30/noise".to_string(),
        ],

        .. Default::default()
    };

    // Config values //////////////////////////////////////////
    let conf_opts = conf::Options{
        paths: default_opts.config_paths.clone(),

        .. Default::default()
    };
    let mut cfgery = Confygery::new();
    let result = cfgery
        .with_opts(conf_opts)
        .add_struct(&default_opts)
        .add_file("noise.toml")
        .add_file("config.toml")
        .build::<Opts>();
    let cfg = match result {
        Ok(x) => x,
        Err(_) => default_opts.clone(),
    };
    // Noise types ////////////////////////////////////////////
    let noise_types = [
        "basic-multi",
        "billow",
        "fbm",
        "hybrid-multi",
        "simplex",
        "perlin",
        "ridged-multi",
        "worley",
    ];
    let cave_types = [
        "simple",
        "linear",
        "wobbly-walls",
        "jagged-walls",
        "fractured",
        "complex",
    ];
    // Common args ////////////////////////////////////////////
    let invert_arg = Arg::with_name("invert?")
        .short("i")
        .long("invert")
        .help("Swap cave walls and open space");
    let log_level_arg = Arg::with_name("log-level")
        .short("l")
        .long("log-level")
        .help("Level for logging output")
        .takes_value(true);
    let output_arg = Arg::with_name("file-name")
        .short("o")
        .long("output")
        .help("File where generated data will be saved")
        .takes_value(true);
    let res_arg = Arg::with_name("x,y")
        .short("r")
        .long("resolution")
        .help("Image/ascii resolution in x,y pixels")
        .takes_value(true);
    let seed_arg = Arg::with_name("seed-number")
        .short("s")
        .long("seed")
        .help("random seed")
        .takes_value(true);
    let threshold_arg = Arg::with_name("cutoff")
        .short("t")
        .long("threshold")
        .help("Value between -1.0 and 1.0 (inclusive) for threshold cutoff")
        .takes_value(true)
        .allow_hyphen_values(true);
    let tiled_arg = Arg::with_name("tiled?")
        .long("tiled")
        .help("Enable (or diable) tiling");
    let turbulence_arg = Arg::with_name("turbulence?")
        .long("turbulence")
        .help("Enable (or diable) turbulence");
    // Cave command ///////////////////////////////////////////
    let cave = App::new("cave")
        .about("Genereate random cave")
        .arg(
            Arg::with_name("cave-type")
                .help("The type of cave to generate")
                .possible_values(&cave_types)
                .required(true),
        )
        .arg(&invert_arg)
        .arg(&log_level_arg)
        .arg(&output_arg)
        .arg(&res_arg)
        .arg(&seed_arg)
        .arg(&threshold_arg)
        .arg(&tiled_arg);
    // CLI assembly ///////////////////////////////////////////
    let matches = App::new("noise")
        .about("Hexagram30 Noise Generator")
        .version(crate_version!())
        .subcommand(cave)
        .arg(
            Arg::with_name("type")
                .help("Type of noise generator to use")
                .possible_values(&noise_types),
        )
        .arg(&invert_arg)
        .arg(&log_level_arg)
        .arg(&output_arg)
        .arg(&res_arg)
        .arg(&seed_arg)
        .arg(&threshold_arg)
        .arg(&tiled_arg)
        .arg(&turbulence_arg)
        .get_matches();

    // Convert args to appropriate types //////////////////////
    let output = matches.value_of("file-name").unwrap_or(cfg.output);
    let log_level = matches
        .value_of("log-level")
        .unwrap_or(cfg.log_level);
    let opts: Opts = Opts {
        inverted: if matches.is_present("invert?") {
            true
        } else {
            false
        },
        log_level: &log_level.to_string(),
        output: &output.to_string(),
        res: max_coords_or(
            matches.value_of("x,y").unwrap_or(&cfg.res_str),
            cfg.res,
        ),
        seed: value_t!(matches, "seed-number", u32).unwrap_or(cfg.seed),
        threshold_enabled: if matches.is_present("cutoff") {
            true
        } else {
            false
        },
        threshold_cutoff: value_t!(matches, "cutoff", f64).unwrap_or(cfg.threshold_cutoff),
        tiled: if matches.is_present("tiled?") {
            true
        } else {
            false
        },
        turbulence: if matches.is_present("turbulence?") {
            true
        } else {
            false
        },
        ..cfg
    };
    let log_opts = twyg::LoggerOpts {
        coloured: true,
        file: None,
        level: opts.log_level.to_string(),
        report_caller: true,
    };

    match twyg::setup_logger(&log_opts) {
        Ok(_) => {}
        Err(error) => panic!("Could not setup logger: {:?}", error),
    };
    log::trace!("Got options: {:?}", opts);

    // Process the subcommands ////////////////////////////////
    match matches.subcommand() {
        ("cave", Some(cave_matches)) => {
            // Process the top-level args (noise-types)
            match cave_matches.value_of("cave-type").unwrap() {
                "simple" => {
                    log::info!("Creating simple cave ...");
                    caves::simple(opts.clone());
                }
                "linear" => {
                    log::info!("Creating linear cave ...");
                    caves::linear(opts.clone());
                }
                "jagged-walls" => {
                    log::info!("Creating jagged-walls cave ...");
                    caves::jagged_walls(opts.clone());
                }
                "wobbly-walls" => {
                    log::info!("Creating wobbly-walls cave ...");
                    caves::wobbly_walls(opts.clone());
                }
                "fractured" => {
                    log::info!("Creating fractured cave ...");
                    caves::fractured(opts.clone());
                }
                "complex" => {
                    log::info!("Creating complex cave ...");
                    caves::complex(opts);
                }
                _ => unreachable!(),
            }
        }
        // Process the top-level args (noise-types)
        ("", None) => match matches.value_of("type").unwrap() {
            "basic-multi" => {
                log::info!("Using BasicMulti noise ...");
            }
            "billow" => {
                log::info!("Using Billow noise ...");
            }
            "fbm" => {
                log::info!("Using Fbm noise ...");
            }
            "hybrid-multi" => {
                log::info!("Using HybridMulti noise ...");
            }
            "perlin" => {
                log::info!("Using Perlin noise ...");
            }
            "ridged-multi" => {
                log::info!("Using RidgedMulti noise ...");
            }
            "simplex" => {
                log::info!("Using Simplex noise ...");
            }
            "worley" => {
                log::info!("Using Worley noise ...");
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn coord_vec(val: &str) -> Vec<usize> {
    return val.split(',').map(|s| s.parse().unwrap()).collect();
}

fn max_coords_or(val: &str, default: Resolution) -> Resolution {
    if val == "" {
        return default;
    } else {
        let coord = coord_vec(val);
        return Resolution {
            x: coord[0],
            y: coord[1],
        };
    }
}
