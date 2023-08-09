#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main ()  {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    // println!("Hello, your ip {:?}, {:?}", home, loopback);

  is_ip(home);

    is_ip(loopback);
}


fn is_ip(ip: IpAddr) {
    match ip {
        IpAddr::V4(_) => println!("Hello, your ip is v4"),
        IpAddr::V6(_) => println!("Hello, your ip is v6"),
    }

}