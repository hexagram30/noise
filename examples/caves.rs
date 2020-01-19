extern crate noise;

use hxgm30noise::gen::base::{Opts, Resolution};
use hxgm30noise::gen::caves;

fn main() {

    let res = Resolution{x: 80, y: 40};
    let is_cave = true;

    caves::complex(&Opts{
        output: &"caves-complex-billow.png".to_string(),
        threshold_cutoff: -0.25,
        res,
        is_cave,
        .. Default::default()
    });

    caves::fractured(&Opts{
        output: &"caves-fractured-hm.png".to_string(),
        threshold_cutoff: 0.1,
        res,
        is_cave,
        .. Default::default()
    });

    caves::jagged_walls(&Opts{
        output: &"caves-jagged-walls-fbm.png".to_string(),
        res,
        is_cave,
        .. Default::default()
    });

    caves::linear(&Opts{
        output: &"caves-linear-rm.png".to_string(),
        threshold_cutoff: -0.4,
        res,
        is_cave,
        .. Default::default()
    });

    caves::simple(&Opts{
        output: &"caves-simple-perlin.png".to_string(),
        res,
        is_cave,
        .. Default::default()
    });

   caves::wobbly_walls(&Opts{
        output: &"caves-wobbly-walls-turbulence.png".to_string(),
        res,
        is_cave,
        .. Default::default()
    });

}
