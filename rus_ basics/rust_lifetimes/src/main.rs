struct User<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Swayam");
    let user = User { name: &name };
    println!("My name is : {}", user.name);
}
