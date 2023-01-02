use std::{
    io::Error as IoError,
};

use network::network_main;

mod network;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    network_main().await?;
    Ok(())
}
