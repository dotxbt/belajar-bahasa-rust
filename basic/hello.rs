fn say_hello(name: &str) {
    println!("Hello, {}!", name)
}

fn get_message(name: String) -> String {
    let mut message = String::new();
    message.push_str("Hello, ");
    message.push_str(&name);
    message.push_str("!");
    message
    message
}

fn modified_with_return(mut data: String) -> String {
    data.push_str(" with return");
    data
}

fn modified_without_return(data: &mut String) {
    data.push_str(" without return");
}

fn main() {
    // print hello
    println!("Hello, Wolrd!");

    // print hello using function
    let name = String::from("Sabituddin Bigbang");
    say_hello(&name);

    // using return function
    let message = get_message(name);
    println!("{}",message);

    let mut data = String::from("My Data");
    let new_data = modified_with_return(data.clone());
    modified_without_return(&mut data);
    println!("{}", new_data);
    println!("{}", data);
}