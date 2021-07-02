fn main() {
    let f = 1.23_f32;
    let i = f as u8;

    dbg!(i, f);

    let word: u16 = 128;
    let byte = word as i8;
    let ubyte = word as u8;
    dbg!(word, byte, ubyte);

    let too_big = 1000;
    let too_small = too_big as u8;
    dbg!(too_big, too_small);

    dbg!(unchecked(0x0fff_ffff, 0x0fff_ffff));
    dbg!(checked(0x0fff_ffff, 0x0fff_ffff));

    dbg!(float_eq(1.321, 1.32));
    dbg!(float_eq(1.32, 1.321));
    dbg!(float_eq(1.320000001, 1.32));
    dbg!(1.320000001_f32 == 1.32_f32);
    assert_eq!(1.320000001, 1.32);
}

/// wraps like it would in C
pub fn unchecked(x: i32, y: i32) -> i32 {
    x.wrapping_mul(y)
}

/// in debug builds this will panic!
pub fn checked(x: i32, y: i32) -> i32 {
    x * y
}

/// float equality (hack! - does not handle NaN/Infinity, see https://floating-point-gui.de/errors/comparison/)
pub fn float_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= f32::EPSILON
}
