fn print_vector(vector: &Vec<i32>) {
    println!("data\t: {:?}\nlength\t: {}\n", vector, vector.len());
}

fn vector() {
    println!("CREATE");
    let v: Vec<i32> = Vec::new();
    let mut vector = vec![1, 2, 3, 4, 5];

    print_vector(&vector);
    print_vector(&v);

    println!("GET");
    let el1 = vector.get(0); 
    let el2 = vector[2];
    println!(
        "el1 : {:?}\nel2 : {:?}",
        el1.unwrap(),
        el2
    );
    print_vector(&vector);

    println!("ADD");
    vector.push(120); // single 
    vector.insert(1, 111); // insert into index 
    vector.extend([33, 22, 11]); // batch
    print_vector(&vector);

    println!("UPDATE");
    vector[1] = 100; // using index
    if let Some(element) = vector.get_mut(2) { // using get_mut
        *element = 120;
    }
    print_vector(&vector);

    println!("SORT");
    vector.sort();
    print_vector(&vector);

    println!("REVERSE");
    vector.reverse();
    print_vector(&vector);

    println!("SLICE | APPEND");
    let mut v2 = vector.split_off(7);
    print_vector(&vector);
    print_vector(&v2);say_hello
    vector.append(&mut v2);
    print_vector(&vector);

    println!("RESIZE");
    vector.resize(20, 0);
    print_vector(&vector);
    vector.reserve(7);qwq
    print_vector(&vector);

    println!("REMOVE");
    vector.pop(); // remove last element
    vector.remove(2); // remove using index
    vector.clear(); // remove all elements
    print_vector(&vector);
}

fn main() {
    vector();
}
