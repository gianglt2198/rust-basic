use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;
use std::time::Duration;

async fn check_status(client: &Client, url: &str) -> Result<()> {
    // We use the client to make the request
    let res = client.get(url).send().await;

    match res {
        Ok(res) => {
            println!("✅ URL: {:<30} | Status: {}", url, res.status());
        }
        Err(e) => {
            eprintln!("❌ URL: {:<30} | Error: {}", url, e);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder().timeout(Duration::from_secs(5)).build()?;

    let urls = vec![
        "https://google.com",
        "https://github.com",
        "https://rust-lang.org",
        "https://not-a-real-site-at-all.com",
    ];

    // 2. The "Functional" way to create tasks
    let tasks: Vec<_> = urls
        .iter()
        .map(|url| check_status(&client, url))
        .collect();

    println!("Checking {} sites concurrently...\n", urls.len());
    join_all(tasks).await;

    Ok(())
}
