extern crate tokio; // 1.7.1;

use hyper::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let uri = "http://stackoverflow.com".parse().unwrap();
    let work = client.get(uri);

    match tokio::time::timeout(Duration::from_millis(10), work).await {
        Ok(result) => match result {
            Ok(response) => println!("Status: {}", response.status()),
            Err(e) => println!("Network error: {:?}", e),
        },
        Err(_) => println!("Timeout: no response in 10 milliseconds."),
    };
}
