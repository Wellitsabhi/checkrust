struct User{
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let name  = String::from("ethg");
    let user = User{
        name: name,
        age: 30,
        active: false,
    };
    println!("{} is {} years old",user.name, user.age);
}
