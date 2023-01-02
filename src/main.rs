use std::io::Error as IoError;

use tickoattwo_rust::network::network_main;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    network_main().await?;
    Ok(())
}
