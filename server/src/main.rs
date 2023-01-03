use std::env;

use simple_logger::SimpleLogger;
use tickoattwo_server::backend::Backend;
use tickoattwo_server::network::serve;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Debug)
        .with_module_level("tungstenite", log::LevelFilter::Error)
        .init()
        .expect("logger error");

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9000".to_string());

    let backend = Backend::new();

    serve(backend, addr).await?;

    Ok(())
}
