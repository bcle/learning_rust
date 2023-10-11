struct User {
    name: String,
    age: i32,
}



fn main() {
    let u1 = User {name: String::from("John"), age: 15};
    let u2 = User {age: 17, ..u1};
    println!("User name and age: {} {}", u1.name, u1.age);
    println!("User name and age: {} {}", u2.name, u2.age);
}
