
#![allow(unused_variables)]
fn main() {
    let name_string_slice = "Qi";

    // let name_string = String::from("Qi");
    let name_string = name_string_slice.to_string();

    println!("Hello, {}, {}!", name_string_slice, name_string);

    let greeting = "Hello, Qi!";
    let part = &greeting[0..5]; // creates a string slice of "Hello"
    print_slice(part);


    print_concatination("Hello", "Qi");
}

fn print_slice(slice: &str) {
    println!("{}", slice);
}


fn print_concatination(slice1: &str, slice2: &str) {
    let new = format!("{} {}", slice1, slice2);
    println!("{}", new);
}
