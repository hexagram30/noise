#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};

struct Opts {
    ascii_size: Vec<i64>,
    ascii_size_str: String,
    image_size: Vec<i64>,
    image_size_str: String,
    seed: i64,
    threshold_cutoff: f64,
    tiled: bool,
    turbulence: bool,
}

fn main() {
    // Default values /////////////////////////////////////////
    let default_opts: Opts = Opts {
        ascii_size: vec![1i64, 20, 80],
        ascii_size_str: "20,80".to_string(),
        image_size: vec![1i64, 100, 100],
        image_size_str: "100,100".to_string(),
        seed: 108,
        threshold_cutoff: 0.0,
        tiled: false,
        turbulence: false,
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
    let bools = [
        "true",
        "false"];
    // Common args ////////////////////////////////////////////
    let ascii_size_arg = Arg::with_name("col,row")
        .short("a")
        .long("ascii")
        .help("image size in col,row chars")
        .takes_value(true);
    let image_size_arg = Arg::with_name("x,y")
        .short("i")
        .long("image")
        .help("image size in x,y pixels")
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
        .help("Enable (or diabled) tiling")
        .takes_value(true)
        .possible_values(&bools);
    let turbulence_arg = Arg::with_name("turbulence?")
        .long("turbulence")
        .help("Enable (or diabled) turbulence")
        .takes_value(true)
        .possible_values(&bools);
    // Clone command //////////////////////////////////////////
    let clone = App::new("cave")
        .about("Genereate random cave")
        .arg(Arg::with_name("cave-type")
            .help("The type of cave to generate")
            .possible_values(&cave_types)
            .required(true))
        .arg(&ascii_size_arg)
        .arg(&image_size_arg)
        .arg(&seed_arg)
        .arg(&threshold_arg)
        .arg(&tiled_arg);
    // Push command and sub-commands //////////////////////////
    let push_remote = App::new("remote")
        .about("pushes remote things")
        .arg(Arg::with_name("repo")
            .required(true)
            .help("The remote repo to push things to"));
    let push_local = App::new("local")
        .about("pushes local things");
    let push = App::new("push")
        .about("pushes things")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(push_remote)
        .subcommand(push_local);
    // Add command ////////////////////////////////////////////
    let add = App::new("add")
        .about("adds things")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("stuff")
            .long("stuff")
            .help("Stuff to add")
            .takes_value(true)
            .multiple(true));
    // CLI assembly ///////////////////////////////////////////
    let matches = App::new("noise")
        .about("Hexagram30 Noise Generator")
        .version(crate_version!())
        .subcommand(clone)
        .subcommand(push)
        .subcommand(add)
        .arg(Arg::with_name("type")
            .help("Type of noise generator to use")
            .possible_values(&noise_types))
        .arg(&ascii_size_arg)
        .arg(&image_size_arg)
        .arg(&seed_arg)
        .arg(&threshold_arg)
        .arg(&tiled_arg)
        .arg(&turbulence_arg)
        .get_matches();

    // Convert args to appropriate types //////////////////////
    let opts: Opts = Opts{
        ascii_size: coord_vec_or(matches.value_of("col,row")
            .unwrap_or(&default_opts.ascii_size_str), default_opts.ascii_size),
        image_size: coord_vec_or(matches.value_of("x,y")
            .unwrap_or(&default_opts.image_size_str), default_opts.image_size),
        seed: value_t!(matches, "seed-number", i64)
            .unwrap_or(default_opts.seed),
        threshold_cutoff: value_t!(matches, "cutoff", f64)
            .unwrap_or(default_opts.threshold_cutoff),
        tiled: value_t!(matches, "tiled?", bool)
            .unwrap_or(default_opts.tiled),
        turbulence: value_t!(matches, "turbulence?", bool)
            .unwrap_or(default_opts.turbulence),
        ..default_opts
    };
    println!("Got ascii size: {:?}", opts.ascii_size);
    println!("Got image size: {:?}", opts.image_size);
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
                }
                "linear" => {
                    println!("Creating linear cave ...");
                }
                "jagged-walls" => {
                    println!("Creating jagged-walls cave ...");
                }
                "wobbly-walls" => {
                    println!("Creating wobbly-walls cave ...");
                }
                "fractured" => {
                    println!("Creating fractured cave ...");
                }
                "complex" => {
                    println!("Creating complex cave ...");
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

fn coord_vec(val: &str) -> Vec<i64> {
    return val.split(',')
              .map(|s| s.parse().unwrap())
              .collect()
}

fn coord_vec_or(val: &str, default: Vec<i64>) -> Vec<i64> {
    if val == "" {
        return default
    } else {
        return coord_vec(val)
    }
}
