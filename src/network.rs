use futures_channel::mpsc::unbounded;
use futures_util::StreamExt;

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

use crate::backend::Backend;
use crate::user::User;

pub async fn handle_connection(
    backend: Arc<Mutex<Backend>>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let user = Arc::new(Mutex::new(User::new()));

    backend.lock().unwrap().user_join(user.clone());

    let (tx, rx) = unbounded::<Message>();
    let (outgoing, incoming) = ws_stream.split();

    println!("{} disconnected", &addr);

    backend.lock().unwrap().user_leave(*user.lock().unwrap());
}

use std::{
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub async fn serve(backend: Backend) -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:12080".to_string());

    let arc_backend = Arc::new(Mutex::new(backend));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(arc_backend.clone(), stream, addr));
    }

    Ok(())
}
