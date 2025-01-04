use std::{sync::mpsc, thread::spawn};

fn main() {
    let (tx,rx) = mpsc::channel();
    spawn(move || {
        tx.send(String::from("Swayam from Transmitter")).unwrap()
    });

    let val = rx.recv();

    match val {
        Ok(res) => println!("{:?}",res),
        Err(err) => println!("Error : {}",err)
    }
}
