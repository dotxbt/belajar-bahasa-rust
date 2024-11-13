fn this_is_integer() {
    // signed integer
    let _signed8bit: i8 = -128;
    let _signed16bit: i16 = -32768;
    let _signed32bit: i32 = -2147483648;
    let _signed64bit: i64 = 9223372036854775807;
    println!(
        "SIGNED INTEGER : u8: {} | u16: {} | u32: {} | u64: {}",
        _signed8bit,
        _signed16bit,
        _signed32bit,
        _signed64bit
    );

    // unsigned integer
    let _unsigned8bit: i8 = 127;
    let _unsigned16bit: i16 = 32767;
    let _unsigned32bit: i64 = 2147483647;
    let _unsigned64bit: u64 = 18446744073709551615;
    println!(
        "UNSIGNED INTEGER : u8: {} | u16: {} | u32: {} | u64: {}",
        _unsigned8bit,
        _unsigned16bit,
        _unsigned32bit,
        _unsigned64bit
    );
}

fn main() {
    this_is_integer();
}
