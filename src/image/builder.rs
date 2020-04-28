use crate::common::Builder;

pub trait BuilderWriter {
    fn write(&self);
}

impl BuilderWriter for Builder<'_> {
    fn write(&self) {
        self.noise_map.write_to_file(&self.opts.output);
    }
}
