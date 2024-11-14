fn this_is_array() {
    let array = [1, 2, 3, 4, 5];
    let mut a: [i8; 5] = [0; 5]; // initiate with 0
    a[0] = -128;
    a[4] = 127;
    println!("ARRAY :\narrary: {:?}\na: {:?}\n", array, a);
}

fn main() {
    this_is_array();
}
