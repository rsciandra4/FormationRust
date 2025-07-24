use chrono::Utc;
use std::error::Error;
use std::sync::Arc;
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

const LOG_DIR: &str = "logs";
const LOG_FILE: &str = "logs/server.log";
const BIND_ADDR: &str = "0.0.0.0:4000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Crée le dossier logs/ si nécessaire
    fs::create_dir_all(LOG_DIR).await?;

    // Fichier de log protégé par un Mutex pour accès concurrent
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .await?;
    let log_file = Arc::new(Mutex::new(log_file));

    let listener = TcpListener::bind(BIND_ADDR).await?;
    println!("Serveur de journalisation lancé sur {BIND_ADDR}");

    loop {
        let (socket, addr) = listener.accept().await?;
        let log_file = Arc::clone(&log_file);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr.to_string(), log_file).await {
                eprintln!("Erreur avec {addr}: {e}");
            }
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    client: String,
    log_file: Arc<Mutex<tokio::fs::File>>,
) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(socket);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let timestamp = Utc::now().to_rfc3339();
        let log_entry = format!("[{timestamp}]  {line}\n");

        let mut file = log_file.lock().await;
        file.write_all(log_entry.as_bytes()).await?;
        println!("Log reçu de {client} : {line}");
    }

    Ok(())
}
