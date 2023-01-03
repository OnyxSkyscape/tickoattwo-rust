use std::env;

use tickoattwo_rust::backend::Backend;
use tickoattwo_rust::network::serve;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let backend = Backend::new();
    serve(backend, addr).await?;
    Ok(())
}
