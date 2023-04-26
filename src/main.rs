fn main() {
    let mut x = 5;
    println!("The value of x before is: {}", x);
    x = 7;

    let y = 6;
    let z = add(x, y);

    println!("The value of x after is: {}", x);
    println!("The value of y is: {}", y);
    println!("The x is: {x} and y is: {} and the sum is: {}", y,z);
    println!("Hello, Cargo!");

    loop {
        if x == 15 {
            print!("{} not agin", x);
            break;
        }
        print!("{} ", x);
        x += 1;
        println!("again!");
    }
}

fn add (x: i32, y: i32) -> i32 {
    return x + y
}


