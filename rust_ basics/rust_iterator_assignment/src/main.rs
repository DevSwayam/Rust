use std::collections::HashMap;

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(2);
    vec.push(4);
    vec.push(21);
    vec.push(23);

    let odd_value_iterator = vec.iter().filter(|x| *x % 2 != 0).map(|x| x * 2);

    let vec: Vec<i32> = odd_value_iterator.collect();

    println!("{:?}", vec);

    let mut hash_map = HashMap::new();
    hash_map.insert(1, "Swayam");
    hash_map.insert(2, "Rajkumar");
    hash_map.insert(3, "Karle");

    let hash_map_new: HashMap<i32,String> = hash_map
        .iter()
        .map(|(k, v)| (k+1 ,format!("{} Cool", v)))
        .collect();

    println!("{:?}", hash_map_new);
}
