use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9001").await?;
    println!("Serveur WebSocket sur ws://0.0.0.0:9001");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client connecté: {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("Erreur avec {addr}: {e}");
            }
        });
    }
}

async fn handle_client(stream: tokio::net::TcpStream) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg?;
        match msg {
            Message::Text(text) => {
                println!("Reçu: {text}");
                write.send(Message::Text(format!("ACK: {text}"))).await?;
            }
            Message::Binary(bin) => {
                println!("Reçu binaire ({} octets)", bin.len());
                write.send(Message::Binary(bin)).await?;
            }
            Message::Close(frame) => {
                write.send(Message::Close(frame)).await?;
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
