use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, connect_async, tungstenite::protocol::Message};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncMessage {
    pub widget_id: String,
    pub payload: Vec<u8>,
}

pub async fn start_host(addr: &str) -> (tokio::sync::mpsc::Sender<SyncMessage>, tokio::sync::mpsc::Receiver<SyncMessage>) {
    let (tx_bcast, _rx) = broadcast::channel::<String>(100);
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);
    
    let tx_bcast_clone = tx_bcast.clone();
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let tx_inner = tx_bcast_clone.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, tx_inner).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    });

    let (tx_out, mut rx_out) = tokio::sync::mpsc::channel::<SyncMessage>(100);
    let (tx_in, rx_in) = tokio::sync::mpsc::channel::<SyncMessage>(100);

    let tx_bcast_out = tx_bcast.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx_out.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = tx_bcast_out.send(json);
            }
        }
    });

    let mut rx_bcast_in = tx_bcast.subscribe();
    tokio::spawn(async move {
        while let Ok(text) = rx_bcast_in.recv().await {
            if let Ok(msg) = serde_json::from_str::<SyncMessage>(&text) {
                let _ = tx_in.send(msg).await;
            }
        }
    });

    (tx_out, rx_in)
}

async fn handle_connection(raw_stream: TcpStream, tx: broadcast::Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(raw_stream).await?;
    let (mut write, mut read) = ws_stream.split();
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        let _ = tx.send(text.to_string());
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            Ok(msg) = rx.recv() => {
                write.send(Message::Text(msg.into())).await?;
            }
        }
    }
    Ok(())
}

pub struct SyncClient;

impl SyncClient {
    pub async fn connect(url: &str) -> Result<(tokio::sync::mpsc::Sender<SyncMessage>, tokio::sync::mpsc::Receiver<SyncMessage>), Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        let (tx_out, mut rx_out) = tokio::sync::mpsc::channel::<SyncMessage>(100);
        let (tx_in, rx_in) = tokio::sync::mpsc::channel::<SyncMessage>(100);

        tokio::spawn(async move {
            while let Some(msg) = rx_out.recv().await {
                if let Ok(json) = serde_json::to_string(&msg) {
                    let _ = write.send(Message::Text(json.into())).await;
                }
            }
        });

        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = read.next().await {
                if let Ok(msg) = serde_json::from_str::<SyncMessage>(&text) {
                    let _ = tx_in.send(msg).await;
                }
            }
        });

        Ok((tx_out, rx_in))
    }
}
