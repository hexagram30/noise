// Copied from https://github.com/Razaekel/noise-rs/blob/develop/src/math.rs
#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    assert!(max >= min);
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

pub fn binary_search(data: &Vec<f64>, item: f64) -> f64 {
    let result = data.binary_search_by(|x| {
        x.partial_cmp(&item)
            .expect("Binary search comparison failed!")
    });
    let index = match result {
        Ok(r) => r,
        Err(r) => r,
    };
    if index >= data.len() {
        let last_item = data.last().unwrap();
        return *last_item;
    }
    let found = data[index];
    if index == 0 || data[index] == item {
        return found;
    }
    let lower = data[index - 1];
    if item - lower < found - item {
        return lower;
    }
    return found;
}

// Copied from https://github.com/Razaekel/noise-rs/blob/develop/src/math.rs
#[inline]
pub fn linear_interpolate(a: f64, b: f64, x: f64) -> f64 {
    x.mul_add(b - a, a)
}

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
        let steps = steps - 1;
        FloatIterator {
            current: 0,
            current_back: steps + 1,
            steps: steps,
            start: start,
            end: end,
        }
    }

    /// calculates number of steps from (end - start) / step
    pub fn new_with_step(start: f64, end: f64, step: f64) -> Self {
        let steps = ((end - start) / step).abs().round() as u64;
        FloatIterator {
            current: 0,
            current_back: steps,
            steps: steps,
            start: start,
            end: end,
        }
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
        // if self.current == self.current_back {
        //     self.current += 1;
        //     Some(self.end);
        // }
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
    fn test_binary_search() {
        let nums = vec![-2.0, -1.0, 0.0, 1.0, 1.5, 2.0, 10.0];
        assert_eq!(binary_search(&nums, -2.5), -2.0);
        assert_eq!(binary_search(&nums, -1.9), -2.0);
        assert_eq!(binary_search(&nums, -1.1), -1.0);
        assert_eq!(binary_search(&nums, -0.9), -1.0);
        assert_eq!(binary_search(&nums, -0.01), 0.0);
        assert_eq!(binary_search(&nums, 0.01), 0.0);
        assert_eq!(binary_search(&nums, 1.2), 1.0);
        assert_eq!(binary_search(&nums, 1.4), 1.5);
        assert_eq!(binary_search(&nums, 1.9), 2.0);
        assert_eq!(binary_search(&nums, 2.1), 2.0);
        assert_eq!(binary_search(&nums, 5.0), 2.0);
        assert_eq!(binary_search(&nums, 7.0), 10.0);
        assert_eq!(binary_search(&nums, 21.0), 10.0);
    }

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
            FloatIterator::new(-2.0, 0.0, 5)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            ["-2.00", "-1.50", "-1.00", "-0.50", "0.00"]
        );
        assert_eq!(
            FloatIterator::new(-2.0, 2.0, 5)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            ["-2.00", "-1.00", "0.00", "1.00", "2.00"]
        );
        assert_eq!(
            FloatIterator::new(-2.0, 2.0, 11)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            [
                "-2.00", "-1.60", "-1.20", "-0.80", "-0.40", "0.00", "0.40", "0.80", "1.20",
                "1.60", "2.00"
            ]
        );
        assert_eq!(
            FloatIterator::new(-4.0, 10.0, 15)
                .map(|x| format!("{:.1$}", x, 2))
                .collect::<Vec<String>>(),
            [
                "-4.00", "-3.00", "-2.00", "-1.00", "-0.00", "1.00", "2.00", "3.00", "4.00",
                "5.00", "6.00", "7.00", "8.00", "9.00", "10.00"
            ]
        );
    }

    #[test]
    fn test_float_iterator_with_step() {
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
