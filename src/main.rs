use std::io::Error as IoError;

use tickoattwo_rust::network::serve;
use tickoattwo_rust::backend::Backend;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let backend = Backend::new();
    serve(backend).await?;
    Ok(())
}
