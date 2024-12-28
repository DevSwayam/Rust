#[tokio::main]
async fn main() {
    println!("Start fetching...");
    
    let data = fetch_data().await; // Wait for the async task to finish
    println!("{}", data); // Output: "Data fetched!"
    
    println!("Done!");
}

async fn fetch_data() -> String {
    // Simulate a delay
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    "Data fetched!".to_string()
}
