use futures::SinkExt;
use futures_util::StreamExt;

use hyper::{
    header::{CONNECTION, SEC_WEBSOCKET_ACCEPT, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_VERSION, UPGRADE},
    http::HeaderValue,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    upgrade::Upgraded,
    Body, Method, Request, Response, Server, StatusCode, Version,
};
use log::{info, warn};
use tokio_tungstenite::WebSocketStream;
use tungstenite::{handshake::derive_accept_key, protocol::Role, Error, Result};

use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{backend::Backend, packet::Packet};

async fn handle_connection(
    backend: Arc<Mutex<Backend>>,
    ws_stream: WebSocketStream<Upgraded>,
    addr: SocketAddr,
) -> Result<()> {
    info!("WS: Connected: {}", addr);

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
                                    warn!("Decode error: {}", err)
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

    info!("WS: Disconnected: {}", &addr);

    backend.lock().unwrap().user_leave(&addr);

    Ok(())
}

async fn handle_request(
    backend: Arc<Mutex<Backend>>,
    mut req: Request<Body>,
    addr: SocketAddr,
) -> Result<Response<Body>, Infallible> {
    info!("HTTP: {}: {}", req.method().as_str(), req.uri().path());

    let headers = req.headers();
    let key = headers.get(SEC_WEBSOCKET_KEY);
    let derived = key.map(|k| derive_accept_key(k.as_bytes()));
    if req.method() != Method::GET
        || req.version() < Version::HTTP_11
        || !headers
            .get(CONNECTION)
            .and_then(|h| h.to_str().ok())
            .map(|h| {
                h.split(|c| c == ' ' || c == ',')
                    .any(|p| p.eq_ignore_ascii_case("upgrade"))
            })
            .unwrap_or(false)
        || !headers
            .get(UPGRADE)
            .and_then(|h| h.to_str().ok())
            .map(|h| h.eq_ignore_ascii_case("websocket"))
            .unwrap_or(false)
        || !headers
            .get(SEC_WEBSOCKET_VERSION)
            .map(|h| h == "13")
            .unwrap_or(false)
        || key.is_none()
        || req.uri() != "/api/ws"
    {
        return Ok(Response::new(Body::from("Hello World!")));
    }

    let ver = req.version();

    tokio::task::spawn(async move {
        match hyper::upgrade::on(&mut req).await {
            Ok(upgraded) => {
                if let Err(e) = handle_connection(
                    backend,
                    WebSocketStream::from_raw_socket(upgraded, Role::Server, None).await,
                    addr,
                )
                .await
                {
                    match e {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        err => warn!("HTTP: Error processing connection: {}", err),
                    }
                }
            }
            Err(e) => warn!("HTTP: Upgrade error: {}", e),
        }
    });

    let mut res = Response::new(Body::empty());
    *res.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
    *res.version_mut() = ver;
    res.headers_mut()
        .append(CONNECTION, HeaderValue::from_static("Upgrade"));
    res.headers_mut()
        .append(UPGRADE, HeaderValue::from_static("websocket"));
    res.headers_mut()
        .append(SEC_WEBSOCKET_ACCEPT, derived.unwrap().parse().unwrap());
    Ok(res)
}

pub async fn serve(backend: Backend, addr: String) -> Result<(), hyper::Error> {
    let backend = Arc::new(Mutex::new(backend));

    let make_svc = make_service_fn(move |conn: &AddrStream| {
        let remote_addr = conn.remote_addr();
        let backend = backend.clone();
        let service = service_fn(move |req| handle_request(backend.clone(), req, remote_addr));
        async { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr.parse().unwrap()).serve(make_svc);

    info!("Listening on {}", addr);

    server.await?;

    Ok::<_, hyper::Error>(())
}
