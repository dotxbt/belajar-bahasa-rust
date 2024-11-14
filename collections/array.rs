fn this_is_array() {
    let array = [1, 2, 3, 4, 5];
    let mut a: [i8; 5] = [0; 5]; // initiate with 0
    a[0] = -128;
    a[4] = 127;
    println!("ARRAY :\narrary: {:?}\na: {:?}\n", array, a);
}

fn this_is_tupple() {
    let tupple = (1i8, "String", "C", 43.5f32, 28903840928422738947298472974924i128);
    let (a, b, c, d, e) = tupple;// copy element to a variable
    println!(
        "TUPPLE :\ntupple: {:?}\n- a: {}\n- b: {}\n- c: {}\n- d: {}\n- e: {}",
        tupple,
        a,
        b,
        c,
        d,
        e
    );
    println!(
        "- tupple.0: {}\n- tupple.1: {}\n- tupple.2: {}\n- tupple.3: {}\n- tupple.4: {}\n",
        tupple.0,
        tupple.1,
        tupple.2,
        tupple.3,
        tupple.4
    );
}

fn this_is_vector() {
    let mut vector = vec![1, 2, 3, 4, 5]; //array with manipulation
    println!("VECTOR :\nvector: {:?}\nlength: {}\n", vector, vector.len());

    vector.push(6); // tambah element
    println!("VECTOR PUSH :\nvector: {:?}\nlength: {}\n", vector, vector.len());

    vector.pop(); // hapus element terakhir
    println!("VECTOR POP :\nvector: {:?}\nlength: {}\n", vector, vector.len());

    vector.remove(2); // hapus element pada index 2
    println!("VECTOR REMOVE :\nvector: {:?}\nlength: {}\n", vector, vector.len());

    let el = vector.get(0); // ambil element pada index 0
    println!("VECTOR GET :\nvector: {:?}\nlength: {}\n- el: {:?}\n", vector, vector.len(), el.unwrap());

    vector.extend([2, 9, 8, 6, 3]); // menambah beberapa element
    println!("VECTOR  EXTEND:\nvector: {:?}\nlength: {}\n", vector, vector.len());

    let mut vector2 = vec![11, 0, 7]; // harus mutable
    vector.append(&mut vector2); // menggabungkan vector lain
    println!("VECTOR  APPEND:\nvector: {:?}\nlength: {}\n", vector, vector.len());

    vector.sort(); // mengurutkan mulai dari nilai terkecil
    println!("VECTOR  SORT:\nvector: {:?}\nlength: {}\n", vector, vector.len());


    vector.reverse(); // membalikkan positi element
    println!("VECTOR REVERSE :\nvector: {:?}\nlength: {}\n", vector, vector.len());
    vector.reverse();

    vector.clear(); // hapus semua element
    println!("VECTOR CLEAR :\nvector: {:?}\nlength: {}\n", vector, vector.len());
    println!("VECTOR Empty Check : {} \n", vector.is_empty()); // cek kosong apa tidak
}

fn main() {
    this_is_array();
    this_is_tupple();
    this_is_vector();
}
