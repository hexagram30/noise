extern crate noise;

use noise::utils::*;
use noise::{Billow, Fbm, HybridMulti, Perlin, RidgedMulti, Turbulence};
use hxgm30noise::modifiers::{Invert, Threshold};

fn main() {
    let rm = RidgedMulti::new();
    let thresh = Threshold::new(&rm)
        .set_cutoff(-0.4);
    let invert = Invert::new(&thresh);

    PlaneMapBuilder::new(&invert)
        .set_size(500, 500)
        .build()
        .write_to_file("caves-rm.png");

    let billow = Billow::new();
    let thresh = Threshold::new(&billow)
        .set_cutoff(-0.25);

    PlaneMapBuilder::new(&thresh)
        .set_size(500, 500)
        .build()
        .write_to_file("caves-billow.png");

    let perlin = Perlin::new();
    let thresh = Threshold::new(&perlin);

    PlaneMapBuilder::new(&thresh)
        .set_size(500, 500)
        .build()
        .write_to_file("caves-perlin.png");

    let fbm = Fbm::new();
    let thresh = Threshold::new(&fbm);

    PlaneMapBuilder::new(&thresh)
        .set_size(500, 500)
        .build()
        .write_to_file("caves-fbm.png");

    let hm = HybridMulti::new();
    let thresh = Threshold::new(&hm);

    PlaneMapBuilder::new(&thresh)
        .set_size(500, 500)
        .build()
        .write_to_file("caves-hm.png");

    let perlin = Perlin::new();
    let t = Turbulence::new(&perlin);
    let thresh = Threshold::new(&t);

    PlaneMapBuilder::new(&thresh)
        .set_size(500, 500)
        .build()
        .write_to_file("caves-turbulence.png");
}
