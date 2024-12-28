use std::collections::HashMap;

fn main() {
    let tuple: Vec<(i32, String)> = vec![
        (1, String::from("Swayam")),
        (2, String::from("Swarangi")),
        (2, String::from("Lokmanya")),
    ];
    let unique_hash_map = return_unique_hash_map(tuple);
    println!("{:?}", unique_hash_map);
}

fn return_unique_hash_map(tuple: Vec<(i32, String)>) -> HashMap<i32, String> {
    let mut hash_map = HashMap::new();

    for (key, value) in tuple {
        hash_map.insert(key, value);
    }

    return hash_map;
}
