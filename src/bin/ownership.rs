//At any given time, you can have either one mutable reference or any number of immutable references.
//References must always be valid.


fn main ()  {

    let mut s = String::from("hello");
    s.push_str(", Qi!");

    let len = calculate_length(&s);   
    println!("The length of '{}' is {}.", s, len);

    
    change(&mut s);
    println!("{}", s);
}


// take references as parameters instead of the actual values
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", changed!");
} 