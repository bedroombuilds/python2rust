use futures::future::join_all;
use http_client::person;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let futures: Vec<_> = (0..50)
        .into_iter()
        .map(|_| reqwest::get(person::URL))
        .collect();
    for p in join_all(futures).await {
        println!(
            "{:#?}",
            person::summary(p?.json::<serde_json::Value>().await?)
        );
    }
    println!("{:.2}secs runtime", now.elapsed().as_secs_f64());
    Ok(())
}
