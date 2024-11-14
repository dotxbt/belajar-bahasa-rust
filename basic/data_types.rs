fn this_is_integer() {
    let _signed8bit: i8 = -128;
    let _signed16bit: i16 = -32768;
    let _signed32bit: i32 = 2147483647;
    let _signed64bit: i64 = 9223372036854775807;
    let _signed128bit: i128 = 170141183460469231731687303715884105727;
    println!(
        "SIGNED INTEGER :\n- i8:\t{}\n- i16:\t{}\n- i32:\t{}\n- i64:\t{}\n- i128:\t{}\n",
        _signed8bit,
        _signed16bit,
        _signed32bit,
        _signed64bit,
        _signed128bit
    );

    println!(
        "LIMIT SIGNED INTEGER :\n- i8:\t{} ... {} \n- i16:\t{} ... {}\n- i32:\t{} ... {}\n- i64:\t{} ... {}\n- i128:\t{} ... {}\n",
        i8::MIN,
        i8::MAX,
        i16::MIN,
        i16::MAX,
        i32::MIN,
        i32::MAX,
        i64::MIN,
        i64::MAX,
        i128::MIN,
        i128::MAX
    );

    // unsigned integer
    let _unsigned8bit: u8 = 255;
    let _unsigned16bit: u16 = 65535;
    let _unsigned32bit: u32 = 4294967295;
    let _unsigned64bit: u64 = 18446744073709551615;
    let _unsigned128bit: u128 = 340282366920938463463374607431768211455;
    println!(
        "UNSIGNED INTEGER :\n- u8:\t{}\n- u16:\t{}\n- u32:\t{}\n- u64:\t{}\n- u128:\t{}\n",
        _unsigned8bit,
        _unsigned16bit,
        _unsigned32bit,
        _unsigned64bit,
        _unsigned128bit
    );

    println!(
        "LIMIT UNSIGNED INTEGER :\n- u8:\t{} ... {} \n- u16:\t{} ... {}\n- u32:\t{} ... {}\n- u64:\t{} ... {}\n- u128:\t{} ... {}\n",
        u8::MIN,
        u8::MAX,
        u16::MIN,
        u16::MAX,
        u32::MIN,
        u32::MAX,
        u64::MIN,
        u64::MAX,
        u128::MIN,
        u128::MAX
    );
}

fn this_is_floating_point() {
    let _float32bit: f32 = 23424.987654321;
    let _float64bit: f64 = 234242424.987654321;
    println!("FLOATING POINT :\n- f32: {:?}\n- f64: {:?}\n", _float32bit, _float64bit);

    println!(
        "LIMIT FLOATING POINT :\n- f32:\t{:?} ... {:?}\n- f64:\t{:?} ... {:?}\n",
        f32::MIN,
        f32::MAX,
        f64::MIN,
        f64::MAX
    );
}

fn this_is_char() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("CHAR : {} | {} | {} \n", c, z, heart_eyed_cat);
}

fn this_is_boolean() {
    let t = true;
    let f: bool = false;
    println!("BOOLEAN :\nt: {}\nf: {}\n", t, f);
}

fn this_is_string() {
    let s1: &str = "Sabituddin Bigbang";
    println!("{}", s1);

    let s2 = "Hello World!".to_string();
    println!("{}", s2);

    let s3 = String::from("Any body here?");
    println!("{}", s3);

    let mut s4 = s3; // pindahkan kepemilikan (ownership) nilai di s3 ke s4 | lalu free references (s3)
    s4.push_str("...Gasskeun!!!");
    println!("{}", s4);
    // println!("{}\n", s3); // s3 was destroyed
}

fn this_is_tupple() {
    let tup = (500, 43.4, 1, '😻');
    println!("TUPPLE : {:?}\n", tup);
    let (a, b, c, d) = tup;
    println!("TUPPLE TO VARIABLE :\n- a : {}\n- b : {:?}\n- c : {}\n- d : {}\n", a, b, c, d);
    println!(
        "TUPPLE ELEMENT :\n- tup.0 : {}\n- tup.1 : {:?}\n- tup.2 : {}\n- tup.3 : {}\n",
        tup.0,
        tup.1,
        tup.2,
        tup.3
    );
}

fn this_is_array() {
    let data = [1, 2, 3, 4, 5]; // [values]
    println!("ARRAY : {:?}", data);
    for element in data.iter() {
        println!("{}", element);
    }

    let mut a: [i8; 4] = [0; 4]; // [initial_value; length_of_data]
    a[0] = 127;
    a[3] = -128;
    println!("A : {:?}", a);
}

fn main() {
    this_is_integer();
    this_is_floating_point();
    this_is_char();
    this_is_boolean();
    this_is_string();
    this_is_tupple();
    this_is_array();
}
