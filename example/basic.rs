use dotenvy::dotenv;
use nvi_rs::{KPSClient, KPSClientConfig};

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let cfg = KPSClientConfig {
        username: std::env::var("KPS_USERNAME").unwrap_or_default(),
        password: std::env::var("KPS_PASSWORD").unwrap_or_default(),
        ..Default::default()
    };

    let client = KPSClient::new(cfg);
    match client
        .verify("12345678901", "UĞUR", "PEKESEN", "1995", None, None)
        .await
    {
        Ok(res) => println!(
            "result: status={} code={} description={:?}",
            res.status, res.code, res.description
        ),
        Err(e) => eprintln!("error: {}", e),
    }
}
