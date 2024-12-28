use std::fs;

fn main() {
    let file_to_read = fs::read_to_string("hello.txt");
    match file_to_read {
        Ok(value) => println!("The string inside the file is : {:?}", value),
        Err(value) => println!("Wasnt able to read file reverted with error: {:?} ", value),
    }
}
