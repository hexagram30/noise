#[macro_use]
extern crate clap;

use clap::{App, Arg};
use hxgm30noise::gen::base::{Opts, Resolution};
use hxgm30noise::gen::caves;

fn main() {
    // Default values /////////////////////////////////////////
    let default_opts: Opts = Opts {
        res: Resolution{x: 100, y: 100},
        res_str: &"100,100".to_string(),
        output: &"/tmp/file.png".to_string(),
        seed: 108,
        .. Default::default()
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
        "worley"];
    let cave_types = [
        "simple",
        "linear",
        "wobbly-walls",
        "jagged-walls",
        "fractured",
        "complex"];
    // Common args ////////////////////////////////////////////
    let invert_arg = Arg::with_name("invert?")
        .short("i")
        .long("invert")
        .help("Swap cave walls and open space");
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
        .allow_hyphen_values(true) ;
    let tiled_arg = Arg::with_name("tiled?")
        .long("tiled")
        .help("Enable (or diable) tiling");
    let turbulence_arg = Arg::with_name("turbulence?")
        .long("turbulence")
        .help("Enable (or diable) turbulence");
    // Cave command ///////////////////////////////////////////
    let cave = App::new("cave")
        .about("Genereate random cave")
        .arg(Arg::with_name("cave-type")
            .help("The type of cave to generate")
            .possible_values(&cave_types)
            .required(true))
        .arg(&invert_arg)
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
        .arg(Arg::with_name("type")
            .help("Type of noise generator to use")
            .possible_values(&noise_types))
        .arg(&invert_arg)
        .arg(&output_arg)
        .arg(&res_arg)
        .arg(&seed_arg)
        .arg(&threshold_arg)
        .arg(&tiled_arg)
        .arg(&turbulence_arg)
        .get_matches();

    // Convert args to appropriate types //////////////////////
    let output = matches.value_of("file-name")
            .unwrap_or(default_opts.output);
    let opts: Opts = Opts{
        inverted: if matches.is_present("invert?") {true} else {false},
        output: &output.to_string(),
        res: max_coords_or(matches.value_of("x,y")
            .unwrap_or(&default_opts.res_str), default_opts.res),
        seed: value_t!(matches, "seed-number", u32)
            .unwrap_or(default_opts.seed),
        threshold_enabled: if matches.is_present("cutoff") {true} else {false},
        threshold_cutoff: value_t!(matches, "cutoff", f64)
            .unwrap_or(default_opts.threshold_cutoff),
        tiled: if matches.is_present("tiled?") {true} else {false},
        turbulence: if matches.is_present("turbulence?") {true} else {false},
        ..default_opts
    };
    println!("Got inverted: {}", opts.inverted);
    println!("Got output: {}", opts.output);
    println!("Got resolution: <{}, {}>", opts.res.x, opts.res.y);
    println!("Got seed: {}", opts.seed);
    println!("Got threshold: {}", opts.threshold_cutoff);
    println!("Got tiled: {}", opts.tiled);
    println!("Got turbulence: {}", opts.turbulence);

    // Process the subcommands ////////////////////////////////
    match matches.subcommand() {
        ("cave", Some(cave_matches)) => {
            // Process the top-level args (noise-types)
            match cave_matches.value_of("cave-type").unwrap() {
                "simple" => {
                    println!("Creating simple cave ...");
                    caves::simple(&opts);
                }
                "linear" => {
                    println!("Creating linear cave ...");
                    caves::linear(&opts);
                }
                "jagged-walls" => {
                    println!("Creating jagged-walls cave ...");
                    caves::jagged_walls(&opts);
                }
                "wobbly-walls" => {
                    println!("Creating wobbly-walls cave ...");
                    caves::wobbly_walls(&opts);
                }
                "fractured" => {
                    println!("Creating fractured cave ...");
                    caves::fractured(&opts);
                }
                "complex" => {
                    println!("Creating complex cave ...");
                    caves::complex(&opts);
                }
                _ => unreachable!(),
            }
        }
        // Process the top-level args (noise-types)
        ("", None) => {
           match matches.value_of("type").unwrap() {
                "basic-multi" => {
                    println!("Using BasicMulti noise ...");
                }
                "billow" => {
                    println!("Using Billow noise ...");
                }
                "fbm" => {
                    println!("Using Fbm noise ...");
                }
                "hybrid-multi" => {
                    println!("Using HybridMulti noise ...");
                }
                "perlin" => {
                    println!("Using Perlin noise ...");
                }
                "ridged-multi" => {
                    println!("Using RidgedMulti noise ...");
                }
                "simplex" => {
                    println!("Using Simplex noise ...");
                }
                "worley" => {
                    println!("Using Worley noise ...");
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn coord_vec(val: &str) -> Vec<usize> {
    return val.split(',')
              .map(|s| s.parse().unwrap())
              .collect()
}

fn max_coords_or(val: &str, default: Resolution) -> Resolution {
    if val == "" {
        return default
    } else {
        let coord = coord_vec(val);
        return Resolution{x: coord[0], y: coord[1]}
    }
}
