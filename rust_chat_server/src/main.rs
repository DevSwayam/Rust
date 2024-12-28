use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Server is listening on localhost:8080");

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (socket, address) = listener.accept().await.unwrap();
        println!("New connection established!");
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split(); // Split the socket into a reader and writer
            let mut reader = BufReader::new(reader);

            loop {
                let mut line = String::new();

                tokio::select! {
                    result = reader.read_line(&mut line) =>{

                        if result.unwrap() == 0 {
                            println!("Client disconnected");
                            break; // Exit loop on EOF
                        }
                        tx.send((line.clone(),address)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() =>{
                        let (msg,other_address) = result.unwrap();
                        if address != other_address {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }

                    }
                }
            }
        });
    }
}
