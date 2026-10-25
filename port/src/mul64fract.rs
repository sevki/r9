//! Fixed-point multiplication helper, from Harvey's `port/mul64fract.c`.

/// Multiply two 64-bit numbers and return the middle 64 bits of the 128-bit
/// product.
///
/// One operand is assumed to be a fixed-point number with the integer part in
/// the high word and the fraction in the low word. With `a = ah:al` and
/// `b = bh:bl`:
///
/// ```text
/// ignored         r0 = lo(al*bl)
/// lsw of result   r1 = hi(al*bl) + lo(al*bh) + lo(ah*bl)
/// msw of result   r2 =             hi(al*bh) + hi(ah*bl) + lo(ah*bh)
/// ignored         r3 = hi(ah*bh)
/// ```
///
/// Arithmetic wraps, as it did in the C original.
pub fn mul64fract(a: u64, b: u64) -> u64 {
    let (ah, al) = (a >> 32, a & 0xffff_ffff);
    let (bh, bl) = (b >> 32, b & 0xffff_ffff);

    ((al * bl) >> 32).wrapping_add(al * bh).wrapping_add(ah * bl).wrapping_add((ah * bh) << 32)
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Runs the input through the C and the c2rust translation of it, which
    /// must agree with each other, and returns their common result.
    fn oracle(a: u64, b: u64) -> u64 {
        let c = port_oracle::c::mul64fract(a, b);
        assert_eq!(c, port_oracle::translated::mul64fract(a, b), "C and c2rust differ");
        c
    }

    const EDGES: [u64; 9] = [
        0,
        1,
        0xffff_ffff,
        0x1_0000_0000,
        0x1_0000_0001,
        0x8000_0000_0000_0000,
        0xdead_beef_cafe_f00d,
        u64::MAX - 1,
        u64::MAX,
    ];

    #[test]
    fn known_values() {
        // 1.0 (as 32.32 fixed point) times x is x.
        assert_eq!(mul64fract(1 << 32, 0x1234_5678_9abc_def0), 0x1234_5678_9abc_def0);
        // 0.5 times 3.0 is 1.5.
        assert_eq!(mul64fract(1 << 31, 3 << 32), 3 << 31);
        assert_eq!(mul64fract(0, u64::MAX), 0);
    }

    #[test]
    fn matches_c_on_edges() {
        for a in EDGES {
            for b in EDGES {
                assert_eq!(mul64fract(a, b), oracle(a, b), "a={a:#x} b={b:#x}");
            }
        }
    }

    #[test]
    fn matches_c_on_pseudorandom() {
        // xorshift64*: deterministic, no dependency needed.
        let mut x = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            x.wrapping_mul(0x2545_f491_4f6c_dd1d)
        };
        for _ in 0..100_000 {
            let (a, b) = (next(), next());
            assert_eq!(mul64fract(a, b), oracle(a, b), "a={a:#x} b={b:#x}");
        }
    }
}
