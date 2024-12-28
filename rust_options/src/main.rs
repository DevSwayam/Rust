fn main() {
    let index: Option<i32> = find_char_index(String::from("Swayam"));

    match index {
        Some(value) => println!("a is at index : {}", value),
        None => println!("A not found"),
    };
}

fn find_char_index(word: String) -> Option<i32> {
    for (index, char) in word.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}
