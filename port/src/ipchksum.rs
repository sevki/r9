//! Internet checksum, from Harvey's `port/ipchecksum.c`.

/// The 16-bit one's complement Internet checksum of `data`.
///
/// 16-bit words are read in native byte order, as the C original did; a
/// trailing odd byte is zero-padded. The C accumulated into a signed `int`,
/// so inputs of 64 KiB or more were undefined behaviour there; here the sum
/// simply wraps, and the two only agree below that size.
pub fn ipchksum(data: &[u8]) -> u16 {
    let (words, rest) = data.as_chunks::<2>();
    let odd = rest.first().map_or(0, |&b| u16::from_ne_bytes([b, 0]));

    let sum = words
        .iter()
        .map(|&w| u16::from_ne_bytes(w))
        .chain(core::iter::once(odd))
        .fold(0u32, |sum, w| sum.wrapping_add(u32::from(w)));

    let sum = (sum >> 16) + (sum & 0xffff);
    let sum = sum + (sum >> 16);
    !(sum as u16)
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Runs the input through the C and the c2rust translation of it, which
    /// must agree with each other, and returns their common result.
    fn oracle(data: &[u8]) -> u16 {
        let c = port_oracle::c::ipchksum(data);
        assert_eq!(c, port_oracle::translated::ipchksum(data), "C and c2rust differ");
        c
    }

    #[test]
    fn rfc1071_example() {
        // RFC 1071 section 3: 00 01 f2 03 f4 f5 f6 f7 sums to ddf2 (big
        // endian words), so the checksum is 220d. On the little-endian hosts
        // we run on, the native-order result is the byte swap.
        let data = [0x00, 0x01, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7];
        assert_eq!(ipchksum(&data).to_be(), 0x220d);
    }

    #[test]
    fn empty_and_single_byte() {
        assert_eq!(ipchksum(&[]), oracle(&[]));
        for b in 0..=255u8 {
            assert_eq!(ipchksum(&[b]), oracle(&[b]), "byte {b:#x}");
        }
    }

    #[test]
    fn matches_c_on_pseudorandom() {
        let mut x = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            x.wrapping_mul(0x2545_f491_4f6c_dd1d)
        };
        // Lengths up to 64 KiB, covering odd and even sizes and word-overflow
        // carries; all-0xff stresses the carry folding.
        for len in (0..300).chain([1023, 1500, 4095, 65535, 65536]) {
            let random: Vec<u8> = (0..len).map(|_| next() as u8).collect();
            assert_eq!(ipchksum(&random), oracle(&random), "random len {len}");
            let ones = vec![0xffu8; len];
            assert_eq!(ipchksum(&ones), oracle(&ones), "0xff len {len}");
        }
    }
}
