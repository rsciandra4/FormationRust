use futures_util::{SinkExt, StreamExt};
use std::io::{self, Write};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "ws://127.0.0.1:9001";
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connecté à {url}");

    let (mut write, mut read) = ws_stream.split();

    // Tâche qui lit les messages du serveur et les affiche
    let reader_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(t)) => println!("\n< {}", t),
                Ok(Message::Binary(b)) => println!("\n< ({} octets binaires)", b.len()),
                Ok(Message::Close(_)) => {
                    println!("Fermeture par le serveur");
                    break;
                }
                _ => {}
            }
        }
    });

    // Boucle d'entrée utilisateur -> envoi au serveur
    let writer_task = tokio::spawn(async move {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                break;
            }
            if line.trim().is_empty() {
                continue;
            }
            if line.trim().eq_ignore_ascii_case("quit") {
                let _ = write.send(Message::Close(None)).await;
                break;
            }
            if write.send(Message::Text(line.trim().to_string())).await.is_err() {
                break;
            }
        }
    });

    let _ = tokio::join!(reader_task, writer_task);
    Ok(())
}
