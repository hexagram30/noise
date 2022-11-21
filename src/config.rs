use confyg::{Confygery, conf};
use super::options::Options;

pub fn build(default_opts: Options) -> Options {
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
        .build::<Options>();
    match result {
        Ok(x) => x.clone(),
        Err(_) => default_opts.clone(),
    }
}
