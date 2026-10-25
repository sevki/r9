//! High-watermark measurements, from Harvey's `port/watermarks.c`.

use core::fmt;

/// Tracks the current and highest values of a bounded quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Watermark {
    highwater: i32,
    curr: i32,
    max: i32,
    /// How many times the value reached `max`.
    hitmax: i32,
    name: &'static str,
}

impl Watermark {
    /// A new, zeroed watermark for values up to `max`.
    pub const fn new(max: i32, name: &'static str) -> Self {
        Self { highwater: 0, curr: 0, max, hitmax: 0, name }
    }

    /// Record `val`, clamped to `0..=max`.
    pub fn note(&mut self, val: i32) {
        // An `else if`, as in C: with a negative `max`, a negative `val`
        // is raised to 0 and not then clamped down to `max`.
        let val = if val < 0 {
            0
        } else if val > self.max {
            self.max
        } else {
            val
        };

        if val > self.highwater {
            self.highwater = val;
            if val == self.max && self.curr < val {
                self.hitmax += 1;
            }
        }
        self.curr = val;
    }

    /// `(highwater, curr, max, hitmax)`.
    pub fn counters(&self) -> (i32, i32, i32, i32) {
        (self.highwater, self.curr, self.max, self.hitmax)
    }
}

/// Formats like `seprintmark`: `name:\thighwater H/M curr C hitmax N\n`.
impl fmt::Display for Watermark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}:\thighwater {}/{} curr {} hitmax {}",
            self.name, self.highwater, self.max, self.curr, self.hitmax
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ffi::CStr;

    #[test]
    fn tracks_highwater_and_hitmax() {
        let mut w = Watermark::new(10, "x");
        for v in [3, 10, 10, 2, 10, -5, 99] {
            w.note(v);
        }
        // Only the first rise to 10 counts: later 10s don't beat the highwater.
        assert_eq!(w.counters(), (10, 10, 10, 1));
    }

    #[test]
    fn display_format() {
        let mut w = Watermark::new(8, "procs");
        w.note(5);
        assert_eq!(w.to_string(), "procs:\thighwater 5/8 curr 5 hitmax 0\n");
    }

    #[test]
    fn matches_c_and_c2rust() {
        // Walks plausible and out-of-range values, including a zero max and
        // a negative max, which the C clamps in its own way.
        let seq = [0, 1, 5, 5, 9, 10, 11, 3, 10, 10, -1, 7, 0, 100, i32::MAX, i32::MIN, 4];
        let name = CStr::from_bytes_with_nul(b"mark\0").unwrap();
        for max in [-3, 0, 1, 7, 10] {
            let mut rust = Watermark::new(max, "mark");
            let mut c = port_oracle::c::initmark(max, name);
            let mut t = port_oracle::translated::initmark(max, name);
            let fields = |w: &port_oracle::Watermark| (w.highwater, w.curr, w.max, w.hitmax);
            assert_eq!(rust.counters(), fields(&c));
            assert_eq!(fields(&c), fields(&t));
            for v in seq {
                rust.note(v);
                port_oracle::c::notemark(&mut c, v);
                port_oracle::translated::notemark(&mut t, v);
                assert_eq!(rust.counters(), fields(&c), "max={max} after {v}");
                assert_eq!(fields(&c), fields(&t), "max={max} after {v}");
                let text = port_oracle::c::seprintmark(&mut c);
                assert_eq!(text, port_oracle::translated::seprintmark(&mut t));
                assert_eq!(rust.to_string(), text, "max={max} after {v}");
            }
        }
    }
}
