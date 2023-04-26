enum Light {
    Bright,
    Dark,
}

fn display(light: &Light) {
    match light {
        Light::Bright => println!("It's bright!"),
        Light::Dark => println!("It's dark!"),
    }
}

fn main() {
   let light = Light::Bright;
   let light2 = Light::Dark;
    display(&light);
    display(&light2);
    
}