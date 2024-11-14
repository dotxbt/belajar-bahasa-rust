fn this_is_tupple() {
    let tupple = (1i8, "String", "C", 43.5f32, 28903840928422738947298472974924i128);
    let (a, b, c, d, e) = tupple; // copy element to a variable
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

fn main() {
    this_is_tupple();
}
