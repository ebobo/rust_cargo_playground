use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

impl User {
    fn new(id: u64, name: String, email: String) -> User {
        User {
            id,
            name,
            email,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: u64,
    name: String,
    category: String,
}

fn main()  {
    let user = User {
        id: 1,
        name: String::from("Qi"),
        email: String::from("qi.xu@carrier.com"),
    };

    let serialized_user = serde_json::to_string(&user).unwrap();
    println!("Serialized user: {:?}", serialized_user);
    let deserialized_user: User = serde_json::from_str(&serialized_user).unwrap();
    println!("Deserialized person: {:?}", deserialized_user);

    let product = Product {
        id: 10272,
        name: String::from("Old Trafford"),
        category: String::from("Lego"),
    };

    println!("Hello, Qi! your info {:?}, {}, {}", user.id, user.name, user.email);

     // use debug trait to print struct
    print!("user {:?} \n", user);
    print!("product {:?}", product);
}