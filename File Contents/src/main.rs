use reqwest::Client;
use chrono::Utc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let endpoints = vec![
        "http://localhost:8080/health",
        "http://localhost:3000/status",
    ];

    loop {
        for url in &endpoints {
            match client.get(*url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    println!("✅ {} [{}] is UP", url, Utc::now());
                }
                Ok(resp) => {
                    println!("⚠️ {} [{}] returned {}", url, Utc::now(), resp.status());
                }
                Err(e) => {
                    println!("❌ {} [{}] unreachable: {}", url, Utc::now(), e);
                }
            }
        }

        // Sleep for 30 seconds before next check
        sleep(Duration::from_secs(30)).await;
    }
                      }
