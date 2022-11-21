use crate::common::Builder;
use crate::options::Options;

use noise::{Billow, Fbm, HybridMulti, Perlin, RidgedMulti, Seedable, Turbulence};

pub fn complex(opts: Options) {
    log::debug!("Setting up generator for complex cave type ...");
    let cave_opts: Options = Options {
        noise_type: &"billow".to_string(),
        threshold_enabled: true,
        ..opts
    };
    let generator = Billow::new();
    let generator = generator.set_seed(opts.seed);
    let mut builder = Builder::new(&generator, &cave_opts);
    builder.generate();
    if opts.is_image {
        use crate::image::BuilderWriter;
        builder.write();
    }
    if opts.is_ascii {
        use crate::ascii::BuilderWriter;
        builder.write();
    }
}

pub fn fractured(opts: Options) {
    log::debug!("Setting up generator for fractured cave type ...");
    let cave_opts: Options = Options {
        noise_type: &"hybrid-multi".to_string(),
        threshold_enabled: true,
        ..opts
    };
    let generator = HybridMulti::new();
    let generator = generator.set_seed(opts.seed);
    let mut builder = Builder::new(&generator, &cave_opts);
    builder.generate();
    if opts.is_image {
        use crate::image::BuilderWriter;
        builder.write();
    }
    if opts.is_ascii {
        use crate::ascii::BuilderWriter;
        builder.write();
    }
}

pub fn jagged_walls(opts: Options) {
    log::debug!("Setting up generator for jagged cave type ...");
    let cave_opts: Options = Options {
        noise_type: &"fbm".to_string(),
        threshold_enabled: true,
        ..opts
    };
    let generator = Fbm::new();
    let generator = generator.set_seed(opts.seed);
    let mut builder = Builder::new(&generator, &cave_opts);
    builder.generate();
    if opts.is_image {
        use crate::image::BuilderWriter;
        builder.write();
    }
    if opts.is_ascii {
        use crate::ascii::BuilderWriter;
        builder.write();
    }
}

pub fn linear(opts: Options) {
    log::debug!("Setting up generator for linear cave type ...");
    let cave_opts: Options = Options {
        inverted: true,
        noise_type: &"ridge-multi".to_string(),
        threshold_enabled: true,
        ..opts
    };
    let generator = RidgedMulti::new();
    let generator = generator.set_seed(opts.seed);
    let mut builder = Builder::new(&generator, &cave_opts);
    builder.generate();
    if opts.is_image {
        use crate::image::BuilderWriter;
        builder.write();
    }
    if opts.is_ascii {
        use crate::ascii::BuilderWriter;
        builder.write();
    }
}

pub fn simple(opts: Options) {
    log::debug!("Setting up generator for simple cave type ...");
    let cave_opts: Options = Options {
        noise_type: &"perlin".to_string(),
        threshold_enabled: true,
        ..opts
    };
    let generator = Perlin::new();
    let generator = generator.set_seed(opts.seed);
    let mut builder = Builder::new(&generator, &cave_opts);
    builder.generate();
    if opts.is_image {
        use crate::image::BuilderWriter;
        builder.write();
    }
    if opts.is_ascii {
        use crate::ascii::BuilderWriter;
        builder.write();
    }
}

pub fn wobbly_walls(opts: Options) {
    log::debug!("Setting up generator for wobbly cave type ...");
    let cave_opts: Options = Options {
        noise_type: &"perlin".to_string(),
        threshold_enabled: true,
        turbulence: true,
        ..opts
    };
    let generator = Perlin::new();
    let generator = generator.set_seed(opts.seed);
    let generator = Turbulence::new(&generator);
    let mut builder = Builder::new(&generator, &cave_opts);
    builder.generate();
    if opts.is_image {
        use crate::image::BuilderWriter;
        builder.write();
    }
    if opts.is_ascii {
        use crate::ascii::BuilderWriter;
        builder.write();
    }
}
