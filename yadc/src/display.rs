//! Printing dimensions and quantities: the functions the derived `Display`
//! impls call, usable by hand-written ones too.
//!
//! A dimension prints as the product of its axes with superscript exponents,
//! positive first (`kg·m⁻¹·s⁻²`), then a non-plain label in braces
//! (`K{Affine}`); dimension one prints as nothing. A
//! quantity prints its number with the caller's flags, then a space and its
//! dimension (`101325.00 kg·m⁻¹·s⁻²`) - nothing after a plain number.

use core::fmt::{self, Display, Formatter, Write};

/// A fixed buffer to format one exponent into: no allocation.
struct Buf {
    bytes: [u8; 32],
    len: usize,
}

impl Write for Buf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len + s.len();
        if end > self.bytes.len() {
            return Err(fmt::Error);
        }
        self.bytes[self.len..end].copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

impl Buf {
    fn as_str(&self) -> &str {
        // Only whole `&str`s were copied in.
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("?")
    }
}

/// `c` as a superscript, where Unicode has one.
fn superscript(c: char) -> char {
    match c {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '-' => '⁻',
        '+' => '⁺',
        '/' => 'ᐟ',
        other => other,
    }
}

/// Writes a dimension: each axis `(symbol, exponent, nonzero)` with its
/// exponent as a superscript (none for `1`), the positive exponents first,
/// joined by `·`; then each tag `(tag, shown)` that is shown, in braces (a
/// label that is not plain). An exponent is
/// negative when its `Display` starts with `-`. Writes nothing for no axes
/// and no tags.
pub fn write_dimension(
    f: &mut Formatter<'_>,
    axes: &[(&str, &dyn Display, bool)],
    tags: &[(&dyn Display, bool)],
) -> fmt::Result {
    let mut first = true;
    for negative in [false, true] {
        for (symbol, exponent, nonzero) in axes {
            if !nonzero {
                continue;
            }
            let mut buf = Buf {
                bytes: [0; 32],
                len: 0,
            };
            write!(buf, "{exponent}")?;
            let e = buf.as_str();
            if e.starts_with('-') != negative {
                continue;
            }
            if !first {
                f.write_char('·')?;
            }
            first = false;
            f.write_str(symbol)?;
            if e != "1" {
                for c in e.chars() {
                    f.write_char(superscript(c))?;
                }
            }
        }
    }
    for (tag, shown) in tags {
        if *shown {
            write!(f, "{{{tag}}}")?;
        }
    }
    Ok(())
}

/// Writes a quantity: `number` through its own `Display` with the caller's
/// flags (`{:.2}`, width, sign), then a space and `dimension` - or nothing
/// more, when the dimension prints as nothing (a plain number).
pub fn write_quantity<N: Display + ?Sized, D: Display + ?Sized>(
    f: &mut Formatter<'_>,
    number: &N,
    dimension: &D,
) -> fmt::Result {
    Display::fmt(number, f)?;
    struct Spaced<'a, 'b> {
        f: &'a mut Formatter<'b>,
        started: bool,
    }
    impl Write for Spaced<'_, '_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if s.is_empty() {
                return Ok(());
            }
            if !self.started {
                self.f.write_char(' ')?;
                self.started = true;
            }
            self.f.write_str(s)
        }
    }
    write!(Spaced { f, started: false }, "{dimension}")
}
