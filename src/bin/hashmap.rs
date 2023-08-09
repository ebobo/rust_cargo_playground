use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("Qi", 41);
    map.insert("Ellen", 41);
    map.insert("Wei", 40);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}