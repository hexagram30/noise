use noise::NoiseFn;

/// Noise function that inverts the output value from the source function.
pub struct Invert<'a, T: 'a> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T>,
}

impl<'a, T> Invert<'a, T> {
    pub fn new(source: &'a dyn NoiseFn<T>) -> Self {
        Self {
            source,
        }
    }
}

impl<'a, T> NoiseFn<T> for Invert<'a, T> {
    fn get(&self, point: T) -> f64 {
        -1.0 * self.source.get(point)
    }
}
