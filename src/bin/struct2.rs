use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}


impl User {
    fn new(id: u64, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
        }
    }
    fn print(&self) {
        println!("Hello, {}! your id is {:?}, your email is {}", self.name, self.id, self.email);
    }
}

fn main ()  {
  let user = User::new(1, String::from("Qi"), String::from("bobo@gmail.com"));

  user.print();}