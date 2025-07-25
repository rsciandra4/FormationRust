use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:6000").await?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut resp = String::new();

    println!("Connecté au serveur. Tapez un message :");

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;

        writer.write_all(input.as_bytes()).await?;
        reader.read_line(&mut resp).await?;
        println!("Réponse : {}", resp.trim());
        resp.clear();
    }
}
