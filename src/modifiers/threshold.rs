use noise::NoiseFn;

/// Noise function that applies a threshold to the output value from the source
/// function, converting it to either 1.0 or -1.0. This is done by comparing
/// the output value to the cutoff: if the value is equal to or less than the
/// cutoff, Threshold converts the value to -1.0; above, and it is converted to
/// 1.0.
pub struct Threshold<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,

    /// Cutoff of the threshold. Default is 0.0.
    pub cutoff: f64,
}

impl<'a, T> Threshold<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Self {
            source,
            cutoff: 0.0,
        }
    }

    pub fn set_cutoff(self, cutoff: f64) -> Self {
        Self {
            cutoff: cutoff,
            ..self
        }
    }

}

impl<'a, T> NoiseFn<T> for Threshold<'a, T> {
    fn get(&self, point: T) -> f64 {
        let value = self.source.get(point);
        if value <= self.cutoff {
            -1.0
        } else {
            1.0
        }
    }
}
