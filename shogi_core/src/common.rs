// To avoid calling `format_args!("{}", u8_value)` and `write_fmt`, which results in code bloating
// core::str::count::do_count_chars and core::fmt::Formatter::pad_integral take 3.2KiB
pub(crate) fn write_u8<W: core::fmt::Write>(sink: &mut W, value: u8) -> core::fmt::Result {
    if value <= 9 {
        // Safety: b'0' <= value + b'0' <= b'9'
        return unsafe { write_ascii_byte(sink, value + b'0') };
    }
    if value <= 99 {
        let value1 = value / 10;
        let value2 = value % 10;
        // Safety: b'0' <= value1 + b'0' <= b'9'
        unsafe { write_ascii_byte(sink, value1 + b'0') }?;
        // Safety: b'0' <= value2 + b'0' <= b'9'
        unsafe { write_ascii_byte(sink, value2 + b'0') }?;
        return Ok(());
    }
    let value1 = value / 100;
    let value2 = (value / 10) % 10;
    let value3 = value % 10;
    // Safety: b'0' <= value1 + b'0' <= b'9'
    unsafe { write_ascii_byte(sink, value1 + b'0') }?;
    // Safety: b'0' <= value2 + b'0' <= b'9'
    unsafe { write_ascii_byte(sink, value2 + b'0') }?;
    // Safety: b'0' <= value3 + b'0' <= b'9'
    unsafe { write_ascii_byte(sink, value3 + b'0') }
}

// To avoid calling `format_args!("{}", u16_value)` and `write_fmt`, which results in code bloating
// core::str::count::do_count_chars and core::fmt::Formatter::pad_integral take 3.2KiB
pub(crate) fn write_u16<W: core::fmt::Write>(sink: &mut W, mut value: u16) -> core::fmt::Result {
    if value == 0 {
        return unsafe { write_ascii_byte(sink, b'0') };
    }
    let bases = [1, 10, 100, 1000, 10000];
    let mut displayed = false;
    for base in bases.into_iter().rev() {
        let digit = value / base; // always < 10
        if displayed || digit != 0 {
            // Safety: b'0' <= digit + b'0' <= b'9'
            unsafe { write_ascii_byte(sink, digit as u8 + b'0') }?;
            displayed = true;
        }
        value -= digit * base;
    }
    Ok(())
}

/// # Safety
/// `ascii_byte` must be an ASCII byte, i.e., 0 <= ascii_byte < 128 must hold.
pub(crate) unsafe fn write_ascii_byte<W: core::fmt::Write>(
    sink: &mut W,
    ascii_byte: u8,
) -> core::fmt::Result {
    // `[ascii_byte]` is always a valid UTF-8 encoding.
    let str = core::str::from_utf8_unchecked(core::slice::from_ref(&ascii_byte));
    sink.write_str(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_u8_test() {
        for value in 0..=255 {
            let mut string = String::new();
            write_u8(&mut string, value).unwrap();
            assert_eq!(string, format!("{}", value));
        }
    }

    #[test]
    fn write_u16_test() {
        for value in 0..=65535 {
            let mut string = String::new();
            write_u16(&mut string, value).unwrap();
            assert_eq!(string, format!("{}", value));
        }
    }
}
