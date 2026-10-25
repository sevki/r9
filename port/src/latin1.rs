//! Latin-1 style compose sequences, from Harvey's `port/latin1.c`.
//!
//! The keyboard driver feeds typed characters here; a sequence such as
//! `'`, `e` composes to `é`. The table is `port/latin1.h` verbatim.

/// A Unicode code point as the kernel sees it. Not a `char`: `Xnnnn`
/// sequences can name surrogates, which the C accepted.
pub type Rune = u32;

/// Outcome of looking up a compose sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Latin1 {
    /// The sequence composes to this rune.
    Rune(Rune),
    /// The sequence is not a valid compose sequence.
    Invalid,
    /// More input is needed: the sequence needs at least this many runes.
    NeedMore(usize),
}

/// One compose rule.
struct Compose {
    /// Leading characters that must be seen first (one or two).
    ld: &'static str,
    /// Options for the last input character.
    si: &'static str,
    /// The rune for each `si` entry, in the same order.
    so: &'static str,
}

/// Rules are tried in order. Two assumptions hold: `ld` is one or two bytes
/// long, and an `ld` can be a prefix of a later `ld` only, never an earlier.
#[rustfmt::skip]
static LATIN_TAB: &[Compose] = &[
    Compose { ld: " ", si: " i", so: "␣ı" },
    Compose { ld: "!~", si: "-=~", so: "≄≇≉" },
    Compose { ld: "!", si: "!<=>?bmp", so: "¡≮≠≯‽⊄∉⊅" },
    Compose { ld: "\"*", si: "IUiu", so: "ΪΫϊϋ" },
    Compose { ld: "\"", si: "\"AEIOUYaeiouy", so: "¨ÄËÏÖÜŸäëïöüÿ" },
    Compose { ld: "$*", si: "fhk", so: "ϕϑϰ" },
    Compose { ld: "$", si: "BEFHILMRVaefglopv", so: "ℬℰℱℋℐℒℳℛƲɑℯƒℊℓℴ℘ʋ" },
    Compose { ld: "\'\"", si: "Uu", so: "Ǘǘ" },
    Compose { ld: "\'", si: "\'ACEILNORSUYZacegilnorsuyz", so: "´ÁĆÉÍĹŃÓŔŚÚÝŹáćéģíĺńóŕśúýź" },
    Compose { ld: "*", si: "*ABCDEFGHIKLMNOPQRSTUWXYZabcdefghiklmnopqrstuwxyz", so: "∗ΑΒΞΔΕΦΓΘΙΚΛΜΝΟΠΨΡΣΤΥΩΧΗΖαβξδεφγθικλμνοπψρστυωχηζ" },
    Compose { ld: "+", si: "-O", so: "±⊕" },
    Compose { ld: ",", si: ",ACEGIKLNORSTUacegiklnorstu", so: "¸ĄÇĘĢĮĶĻŅǪŖŞŢŲąçęģįķļņǫŗşţų" },
    Compose { ld: "-*", si: "l", so: "ƛ" },
    Compose { ld: "-", si: "+-2:>DGHILOTZbdghiltuz~", so: "∓\u{AD}ƻ÷→ÐǤĦƗŁ⊖ŦƵƀðǥℏɨłŧʉƶ≂" },
    Compose { ld: ".", si: ".CEGILOZceglz", so: "·ĊĖĠİĿ⊙Żċėġŀż" },
    Compose { ld: "/", si: "Oo", so: "Øø" },
    Compose { ld: "1", si: ".234568", so: "․½⅓¼⅕⅙⅛" },
    Compose { ld: "2", si: "-.35", so: "ƻ‥⅔⅖" },
    Compose { ld: "3", si: ".458", so: "…¾⅗⅜" },
    Compose { ld: "4", si: "5", so: "⅘" },
    Compose { ld: "5", si: "68", so: "⅚⅝" },
    Compose { ld: "7", si: "8", so: "⅞" },
    Compose { ld: ":", si: "()-=", so: "☹☺÷≔" },
    Compose { ld: "<!", si: "=~", so: "≨⋦" },
    Compose { ld: "<", si: "-<=>~", so: "←«≤≶≲" },
    Compose { ld: "=", si: ":<=>OV", so: "≕⋜≡⋝⊜⇒" },
    Compose { ld: ">!", si: "=~", so: "≩⋧" },
    Compose { ld: ">", si: "<=>~", so: "≷≥»≳" },
    Compose { ld: "?", si: "!?", so: "‽¿" },
    Compose { ld: "@\'", si: "\'", so: "ъ" },
    Compose { ld: "@@", si: "\'EKSTYZekstyz", so: "ьЕКСТЫЗекстыз" },
    Compose { ld: "@C", si: "Hh", so: "ЧЧ" },
    Compose { ld: "@E", si: "Hh", so: "ЭЭ" },
    Compose { ld: "@K", si: "Hh", so: "ХХ" },
    Compose { ld: "@S", si: "CHch", so: "ЩШЩШ" },
    Compose { ld: "@T", si: "Ss", so: "ЦЦ" },
    Compose { ld: "@Y", si: "AEOUaeou", so: "ЯЕЁЮЯЕЁЮ" },
    Compose { ld: "@Z", si: "Hh", so: "ЖЖ" },
    Compose { ld: "@c", si: "h", so: "ч" },
    Compose { ld: "@e", si: "h", so: "э" },
    Compose { ld: "@k", si: "h", so: "х" },
    Compose { ld: "@s", si: "ch", so: "щш" },
    Compose { ld: "@t", si: "s", so: "ц" },
    Compose { ld: "@y", si: "aeou", so: "яеёю" },
    Compose { ld: "@z", si: "h", so: "ж" },
    Compose { ld: "@", si: "ABDFGIJLMNOPRUVXabdfgijlmnopruvx", so: "АБДФГИЙЛМНОПРУВХабдфгийлмнопрувх" },
    Compose { ld: "A", si: "E", so: "Æ" },
    Compose { ld: "C", si: "ACU", so: "⋂ℂ⋃" },
    Compose { ld: "Dv", si: "Zz", so: "Ǆǅ" },
    Compose { ld: "D", si: "-e", so: "Ð∆" },
    Compose { ld: "G", si: "-", so: "Ǥ" },
    Compose { ld: "H", si: "-H", so: "Ħℍ" },
    Compose { ld: "I", si: "-J", so: "ƗĲ" },
    Compose { ld: "L", si: "&-Jj|", so: "⋀ŁǇǈ⋁" },
    Compose { ld: "M", si: "#48bs", so: "♮♩♪♭♯" },
    Compose { ld: "N", si: "JNj", so: "Ǌℕǋ" },
    Compose { ld: "O", si: "*+-./=EIcoprx", so: "⊛⊕⊖⊙⊘⊜ŒƢ©⊚℗®⊗" },
    Compose { ld: "P", si: "P", so: "ℙ" },
    Compose { ld: "Q", si: "Q", so: "ℚ" },
    Compose { ld: "R", si: "R", so: "ℝ" },
    Compose { ld: "S", si: "123S", so: "¹²³§" },
    Compose { ld: "T", si: "-u", so: "Ŧ⊨" },
    Compose { ld: "V", si: "=", so: "⇐" },
    Compose { ld: "Y", si: "R", so: "Ʀ" },
    Compose { ld: "Z", si: "-ACSZ", so: "Ƶℤ" },
    Compose { ld: "^", si: "ACEGHIJOSUWYaceghijosuwy", so: "ÂĈÊĜĤÎĴÔŜÛŴŶâĉêĝĥîĵôŝûŵŷ" },
    Compose { ld: "_\"", si: "AUau", so: "ǞǕǟǖ" },
    Compose { ld: "_,", si: "Oo", so: "Ǭǭ" },
    Compose { ld: "_.", si: "Aa", so: "Ǡǡ" },
    Compose { ld: "_", si: "AEIOU_aeiou", so: "ĀĒĪŌŪ¯āēīōū" },
    Compose { ld: "`\"", si: "Uu", so: "Ǜǜ" },
    Compose { ld: "`", si: "AEIOUaeiou", so: "ÀÈÌÒÙàèìòù" },
    Compose { ld: "a", si: "ben", so: "↔æ∠" },
    Compose { ld: "b", si: "()+-0123456789=bknpqru", so: "₍₎₊₋₀₁₂₃₄₅₆₇₈₉₌♝♚♞♟♛♜•" },
    Compose { ld: "c", si: "$Oagu", so: "¢©∩≅∪" },
    Compose { ld: "dv", si: "z", so: "ǆ" },
    Compose { ld: "d", si: "-adegz", so: "ð↓‡°†ʣ" },
    Compose { ld: "e", si: "$lmns", so: "€⋯—–∅" },
    Compose { ld: "f", si: "a", so: "∀" },
    Compose { ld: "g", si: "$-r", so: "¤ǥ∇" },
    Compose { ld: "h", si: "-v", so: "ℏƕ" },
    Compose { ld: "i", si: "-bfjps", so: "ɨ⊆∞ĳ⊇∫" },
    Compose { ld: "l", si: "\"$&\'-jz|", so: "“£∧‘łǉ⋄∨" },
    Compose { ld: "m", si: "iou", so: "µ∈×" },
    Compose { ld: "n", si: "jo", so: "ǌ¬" },
    Compose { ld: "o", si: "AOUaeiu", so: "Å⊚Ůåœƣů" },
    Compose { ld: "p", si: "Odgrt", so: "℗∂¶∏∝" },
    Compose { ld: "r", si: "\"\'O", so: "”’®" },
    Compose { ld: "s", si: "()+-0123456789=abnoprstu", so: "⁽⁾⁺⁻⁰ⁱ⁲⁳⁴⁵⁶⁷⁸⁹⁼ª⊂ⁿº⊃√ß∍∑" },
    Compose { ld: "t", si: "-efmsu", so: "ŧ∃∴™ς⊢" },
    Compose { ld: "u", si: "-AEGIOUaegiou", so: "ʉĂĔĞĬŎŬ↑ĕğĭŏŭ" },
    Compose { ld: "v\"", si: "Uu", so: "Ǚǚ" },
    Compose { ld: "v", si: "ACDEGIKLNORSTUZacdegijklnorstuz", so: "ǍČĎĚǦǏǨĽŇǑŘŠŤǓŽǎčďěǧǐǰǩľňǒřšťǔž" },
    Compose { ld: "w", si: "bknpqr", so: "♗♔♘♙♕♖" },
    Compose { ld: "x", si: "O", so: "⊗" },
    Compose { ld: "y", si: "$", so: "¥" },
    Compose { ld: "z", si: "-", so: "ƶ" },
    Compose { ld: "|", si: "Pp|", so: "Þþ¦" },
    Compose { ld: "~!", si: "=", so: "≆" },
    Compose { ld: "~", si: "-=AINOUainou~", so: "≃≅ÃĨÑÕŨãĩñõũ≈" },
];

/// Parse the four hex digits after the `X` in `Xnnnn`.
fn unicode(k: &[Rune]) -> Option<Rune> {
    k[1..5].iter().try_fold(0, |c, &d| Some(c << 4 | char::from_u32(d)?.to_digit(16)?))
}

/// Find the rune for the compose sequence `k`.
///
/// Returns [`Latin1::NeedMore`] with the required length when `k` is too
/// short to decide, and treats an empty `k` as needing one rune.
pub fn latin1(k: &[Rune]) -> Latin1 {
    let Some(&first) = k.first() else {
        return Latin1::NeedMore(1);
    };
    if first == Rune::from(b'X') {
        return match k.len() {
            5.. => unicode(k).map_or(Latin1::Invalid, Latin1::Rune),
            _ => Latin1::NeedMore(5),
        };
    }
    for rule in LATIN_TAB {
        let ld = rule.ld.as_bytes();
        if first != Rune::from(ld[0]) {
            continue;
        }
        let Some(&second) = k.get(1) else {
            return Latin1::NeedMore(2);
        };
        let last = match ld.get(1) {
            None => second,
            Some(&b) if Rune::from(b) != second => continue,
            Some(_) => match k.get(2) {
                Some(&third) => third,
                None => return Latin1::NeedMore(3),
            },
        };
        return rule
            .si
            .bytes()
            .position(|b| Rune::from(b) == last)
            .and_then(|i| rule.so.chars().nth(i))
            .map_or(Latin1::Invalid, |c| Latin1::Rune(c.into()));
    }
    Latin1::Invalid
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Runs the input through the C and the c2rust translation of it, which
    /// must agree with each other, and returns their common result.
    fn oracle(k: &[u32]) -> i32 {
        let c = port_oracle::c::latin1(k);
        assert_eq!(c, port_oracle::translated::latin1(k), "C and c2rust differ");
        c
    }

    /// The C's status code for an outcome.
    fn code(l: Latin1) -> i32 {
        match l {
            Latin1::Rune(r) => r as i32,
            Latin1::Invalid => -1,
            Latin1::NeedMore(n) => -(n as i32),
        }
    }

    fn runes(s: &str) -> Vec<Rune> {
        s.chars().map(Rune::from).collect()
    }

    #[test]
    fn table_is_well_formed() {
        for rule in LATIN_TAB {
            assert!(matches!(rule.ld.len(), 1 | 2), "{:?}", rule.ld);
            assert!(rule.si.is_ascii(), "{:?}", rule.ld);
            assert_eq!(rule.si.len(), rule.so.chars().count(), "{:?}", rule.ld);
        }
    }

    #[test]
    fn known_sequences() {
        assert_eq!(latin1(&runes("'e")), Latin1::Rune('é'.into()));
        assert_eq!(latin1(&runes("\"A")), Latin1::Rune('Ä'.into()));
        assert_eq!(latin1(&runes("!~=")), Latin1::Rune('≇'.into()));
        assert_eq!(latin1(&runes("XfFfF")), Latin1::Rune(0xffff));
        assert_eq!(latin1(&runes("zz")), Latin1::Invalid);
        assert_eq!(latin1(&runes("'")), Latin1::NeedMore(2));
        assert_eq!(latin1(&runes("!~")), Latin1::NeedMore(3));
        assert_eq!(latin1(&runes("X12")), Latin1::NeedMore(5));
        assert_eq!(latin1(&runes("Xg000")), Latin1::Invalid);
        assert_eq!(latin1(&[]), Latin1::NeedMore(1));
    }

    #[test]
    fn matches_c_on_every_prefix_of_every_rule() {
        // Each rule's ld, then each si option, truncated at every length.
        for rule in LATIN_TAB {
            for opt in rule.si.chars() {
                let full: Vec<Rune> = rule.ld.chars().chain([opt]).map(Rune::from).collect();
                for n in 1..=full.len() {
                    assert_eq!(code(latin1(&full[..n])), oracle(&full[..n]), "{:?}", &full[..n]);
                }
            }
        }
    }

    #[test]
    fn matches_c_on_exhaustive_short_sequences() {
        // Every one-, two- and three-rune sequence over the alphabet the
        // table uses, plus a few that it doesn't.
        let mut alphabet: Vec<Rune> = (0x20..0x7f).collect();
        alphabet.extend([0, 0x80, 0xe9, 0x2260, 0xd800, 0x10ffff]);
        for &a in &alphabet {
            assert_eq!(code(latin1(&[a])), oracle(&[a]), "{a:#x}");
            for &b in &alphabet {
                assert_eq!(code(latin1(&[a, b])), oracle(&[a, b]), "{a:#x} {b:#x}");
                for &c in &alphabet {
                    let k = [a, b, c];
                    assert_eq!(code(latin1(&k)), oracle(&k), "{k:x?}");
                }
            }
        }
    }

    #[test]
    fn matches_c_on_unicode_escapes() {
        for s in ["X0000", "X0041", "XFFFF", "Xd800", "XDfFf", "Xabcg", "XabcG", "X00 1", "X123"] {
            let k = runes(s);
            assert_eq!(code(latin1(&k)), oracle(&k), "{s}");
            // A longer buffer must behave the same: only five runes are read.
            let longer = runes(&format!("{s}zz"));
            assert_eq!(code(latin1(&longer)), oracle(&longer), "{s}zz");
        }
    }
}
