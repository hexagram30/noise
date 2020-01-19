extern crate noise;

use hxgm30noise::gen::caves;
use hxgm30noise::gen::caves::{Opts, Size};

fn main() {

    let size = Size{x: 500, y: 500};

    caves::complex(Opts{
        output: "caves-complex-billow.png".to_string(),
        threshold_cutoff: -0.25,
        inverted: true,
        size,
        .. Default::default()
    });

    caves::fractured(Opts{
        output: "caves-fractured-hm.png".to_string(),
        threshold_cutoff: 0.1,
        size,
        .. Default::default()
    });

    caves::jagged_walls(Opts{
        output: "caves-jagged-walls-fbm.png".to_string(),
        size,
        .. Default::default()
    });

    caves::linear(Opts{
        output: "caves-linear-rm.png".to_string(),
        threshold_cutoff: -0.4,
        size,
        .. Default::default()
    });

    caves::simple(Opts{
        output: "caves-simple-perlin.png".to_string(),
        size,
        .. Default::default()
    });

   caves::wobbly_walls(Opts{
        output: "caves-wobbly-walls-turbulence.png".to_string(),
        size,
        .. Default::default()
    });

}
