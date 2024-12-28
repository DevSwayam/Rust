fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    println!("{:?}", vec);

    println!("{:?}", return_even(vec));
    let numbers: Vec<u64> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);
}

fn return_even(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}
