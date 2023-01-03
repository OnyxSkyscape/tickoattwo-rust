use futures::SinkExt;
use futures_util::StreamExt;

use tokio::net::{TcpListener, TcpStream};
use tungstenite::{Error, Result};

use std::{
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{backend::Backend, packet::Packet};

async fn handle_connection(
    backend: Arc<Mutex<Backend>>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut tx, mut rx) = ws_stream.split();

    backend.lock().unwrap().user_join(&addr);

    loop {
        tokio::select! {
            msg = rx.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() {
                            match Packet::decode_message(&msg) {
                                Ok(packet) =>  {
                                    let event = {
                                        backend.lock().unwrap().dispatch_event(packet.event, &addr)
                                    };
                                    if let Some(res) = event {
                                        tx.send(res.encode_message()).await?;
                                    }
                                },
                                Err(err) => {
                                    println!("error: {}", err)
                                }
                            }
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
    }

    println!("{} disconnected", &addr);

    backend.lock().unwrap().user_leave(&addr);

    Ok(())
}

async fn accept_connection(backend: Arc<Mutex<Backend>>, peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(backend, stream, peer).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

pub async fn serve(backend: Backend) -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let backend = Arc::new(Mutex::new(backend));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        tokio::spawn(accept_connection(Arc::clone(&backend), peer, stream));
    }

    Ok(())
}
