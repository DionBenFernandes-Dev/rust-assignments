struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let user1 = User{
        name: String::from("Dion Ben Fernandes"),
        age: 25,
        active: true,
    };
    println!("User name is {} of age {} active status: {}",user1.name, user1.age, user1.active);
}
