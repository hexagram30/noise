// Copied from: https://stackoverflow.com/a/47869373
/// produces: [ linear_interpol(start, end, i/steps) | i <- 0..steps ]
/// (does NOT include "end")
///
/// linear_interpol(a, b, p) = (1 - p) * a + p * b
pub struct FloatIterator {
    current: u64,
    current_back: u64,
    steps: u64,
    start: f64,
    end: f64,
}

impl FloatIterator {
    pub fn new(start: f64, end: f64, steps: u64) -> Self {
        FloatIterator {
            current: 0,
            current_back: steps,
            steps: steps,
            start: start,
            end: end,
        }
    }

    /// calculates number of steps from (end - start) / step
    pub fn new_with_step(start: f64, end: f64, step: f64) -> Self {
        let steps = ((end - start) / step).abs().round() as u64;
        Self::new(start, end, steps)
    }

    pub fn length(&self) -> u64 {
        self.current_back - self.current
    }

    fn at(&self, pos: u64) -> f64 {
        let f_pos = pos as f64 / self.steps as f64;
        (1. - f_pos) * self.start + f_pos * self.end
    }

    /// panics (in debug) when len doesn't fit in usize
    fn usize_len(&self) -> usize {
        let l = self.length();
        debug_assert!(l <= ::std::usize::MAX as u64);
        l as usize
    }
}

impl Iterator for FloatIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.current_back {
            return None;
        }
        let result = self.at(self.current);
        self.current += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.usize_len();
        (l, Some(l))
    }

    fn count(self) -> usize {
        self.usize_len()
    }
}

impl DoubleEndedIterator for FloatIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current >= self.current_back {
            return None;
        }
        self.current_back -= 1;
        let result = self.at(self.current_back);
        Some(result)
    }
}

impl ExactSizeIterator for FloatIterator {
    fn len(&self) -> usize {
        self.usize_len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_iterator_count() {
        assert_eq!(FloatIterator::new_with_step(-1.0, 1.0, 0.1).count(), 20);
        assert_eq!(FloatIterator::new_with_step(-2.0, 0.0, 0.5).count(), 4);
        assert_eq!(FloatIterator::new_with_step(-2.0, 1.0, 0.75).count(), 4);
        assert_eq!(FloatIterator::new_with_step(-2.0, 1.75, 0.66667).count(), 6);
        assert_eq!(FloatIterator::new_with_step(-2.0, 1.0, 0.33333).count(), 9);
    }

    #[test]
    fn test_float_iterator() {
        assert_eq!(
            FloatIterator::new_with_step(-1.0, 1.0, 0.1)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            [
                "-1.00", "-0.90", "-0.80", "-0.70", "-0.60", "-0.50", "-0.40", "-0.30", "-0.20",
                "-0.10", "0.00", "0.10", "0.20", "0.30", "0.40", "0.50", "0.60", "0.70", "0.80",
                "0.90"
            ]
        );
        assert_eq!(
            FloatIterator::new_with_step(-2.0, 0.0, 0.5)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            ["-2.00", "-1.50", "-1.00", "-0.50"]
        );
        assert_eq!(
            FloatIterator::new_with_step(-2.0, 1.0, 0.75)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            ["-2.00", "-1.25", "-0.50", "0.25"]
        );
        assert_eq!(
            FloatIterator::new_with_step(-2.0, 1.75, 0.66667)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            ["-2.00", "-1.38", "-0.75", "-0.12", "0.50", "1.13"]
        );
        assert_eq!(
            FloatIterator::new_with_step(-2.0, 1.0, 0.33333)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            ["-2.00", "-1.67", "-1.33", "-1.00", "-0.67", "-0.33", "-0.00", "0.33", "0.67"]
        );
    }
}
