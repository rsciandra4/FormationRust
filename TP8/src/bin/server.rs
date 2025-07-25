use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:6000").await?;
    println!("Serveur en écoute sur 0.0.0.0:6000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connecté : {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Erreur avec {addr}: {e}");
            }
        });
    }
}

async fn handle_client(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    while reader.read_line(&mut line).await? != 0 {
        let msg = line.trim();
        println!("Message reçu : {msg}");
        writer.write_all(format!("ACK: {msg}\n").as_bytes()).await?;
        line.clear();
    }
    Ok(())
}
