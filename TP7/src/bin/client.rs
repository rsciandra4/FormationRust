use std::env;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: client <domaine>");
        return Ok(());
    }
    let domain = &args[1];

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:5300")?;

    let req = build_query(domain);
    socket.send(&req)?;

    let mut buf = [0u8; 512];
    let len = socket.recv(&mut buf)?;
    parse_response(&buf[..len]);

    Ok(())
}

fn build_query(domain: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0x12, 0x34, 0x01, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]); // Header
    for part in domain.split('.') {
        buf.push(part.len() as u8);
        buf.extend_from_slice(part.as_bytes());
    }
    buf.push(0); // fin du nom
    buf.extend_from_slice(&[0x00, 0x01, 0x00, 0x01]); // QTYPE A, QCLASS IN
    buf
}

fn parse_response(resp: &[u8]) {
    if resp.len() < 32 { return; }
    let ip = format!("{}.{}.{}.{}", resp[resp.len()-4], resp[resp.len()-3], resp[resp.len()-2], resp[resp.len()-1]);
    println!("Réponse : {ip}");
}
