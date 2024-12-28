use std::vec;

fn main() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    let iterator = vector.iter_mut(); // this can mutate the array

    for val in iterator {
        println!("{}", *val + 1); // val points to the address where the variable is stored and *val gives the actual varible to change
                                  /*
                                  ..
                                  0xabd val -> 0xabc
                                  0xabc vector[0] -> 15 == *val
                                  ..
                                   */
    }

    let mut iter = vector.iter(); // the iterator also need to be mutable as the vector length changes the iterator value should alsp be increased/decreased 

    while let Some(val) = iter.next() {
        println!("{}", val);
    }

    let new_vector = vec![4,5,6];

    let total_sum:i32 = new_vector.iter().sum();
    println!("{}",total_sum);

    let iter2 = new_vector.iter().map(|x| x+6);
    for value in iter2{
        println!("{}",value);
    }

    let iter3 = new_vector.iter().filter(|x| *x%2 == 0);
    for value in iter3{
        println!("{}",value);
    }

}
