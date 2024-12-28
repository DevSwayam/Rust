use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    users.insert(1, String::from("Swayam"));

    let retrieved_value = users.get(&1);

    match retrieved_value {
        Some(value) => println!("The 1st user is : {:?}", value),
        None => println!("User not found")
    }
    
}
