fn main() {
    let mut str = String::from("Swayam");
    pass_ref(&str);
    pass_mut_ref(&mut str);
}

fn pass_ref(s: &String) {
    println!("{}", s);
}

fn pass_mut_ref(s: &mut String) {
    s.push_str(" Karle");
    println!("{}",s);
}

// You can have as many readable references for a immutable variable 
// but you cant have more than one refernce for mutable variable as the refernce can delete the variable which may lead to memory error
