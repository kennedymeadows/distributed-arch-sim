use reqwest::Error;
use tokio::sync::Semaphore;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let semaphore = Arc::new(Semaphore::new(500));
    let mut handles = vec![];

    for i in 1..=10000 { // Adjust the range for the number of requests you want to send
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let client = reqwest::Client::new();
        
        handles.push(tokio::spawn(async move {
            let res = client.post("http://localhost:30000/produce")
                .body(format!("Test message {}", i))
                .send()
                .await;

            match res {
                Ok(_) => println!("Successfully sent message {}", i),
                Err(e) => eprintln!("Error sending message {}: {}", i, e),
            }
            drop(permit); // Release the permit after the request is made
        }));
        
        // Optional: introduce a slight delay to avoid overwhelming your system
        sleep(Duration::from_millis(10)).await;
    }

    // Await all the request tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
