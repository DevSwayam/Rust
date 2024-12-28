use chrono::{Local,Utc};

fn main() {
    println!("The current Local time is : {:?}",Local::now());
    println!("The current UTC time is : {:?}",Utc::now());
}
