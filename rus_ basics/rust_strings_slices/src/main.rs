fn main() {
    let str = String::from("Swayam Karle");
    println!("{:?}",return_first_word(str));
}

fn return_first_word_using_slice(str_pointer:&String) -> &str{
    let mut index = 0;
    for i in str_pointer.chars(){
        if i == ' ' {
            break;
        }
        index += 1;
    }
    return &str_pointer[0..index];

}

fn return_first_word(str:String) -> String{
    let mut ans = String::from("");
    for val in str.chars(){
        if val == ' ' {
            break;
        }
        ans.push_str(&val.to_string());
    }
    return ans;
}
