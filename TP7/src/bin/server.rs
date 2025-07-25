use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:5300")?;
    println!("Serveur DNS simplifié en écoute sur 0.0.0.0:5300");

    let mut buf = [0u8; 512];
    loop {
        let (len, addr) = socket.recv_from(&mut buf)?;
        let req = &buf[..len];
        let domain = extract_domain(req);

        let resp = build_response(req, &domain);
        socket.send_to(&resp, addr)?;
        println!("Requête pour {domain} depuis {addr}");
    }
}

fn extract_domain(req: &[u8]) -> String {
    let mut i = 12; 
    let mut labels = Vec::new();
    while i < req.len() && req[i] != 0 {
        let len = req[i] as usize;
        i += 1;
        labels.push(String::from_utf8_lossy(&req[i..i + len]).to_string());
        i += len;
    }
    labels.join(".")
}

fn build_response(req: &[u8], domain: &str) -> Vec<u8> {
    let mut resp = req.to_vec();
    resp[2] = 0x81; resp[3] = 0x80; // réponse standard
    resp[7] = 1; // 1 réponse

    // Ajoute réponse si domaine connu
    if domain == "example.com" {
        resp.extend_from_slice(&[0xC0, 0x0C]); 
        resp.extend_from_slice(&[0x00, 0x01]); // TYPE A
        resp.extend_from_slice(&[0x00, 0x01]); // CLASS IN
        resp.extend_from_slice(&[0x00, 0x00, 0x00, 0x3C]); 
        resp.extend_from_slice(&[0x00, 0x04]); // IPv4 = 4 octets
        resp.extend_from_slice(&[93, 184, 216, 34]); 
    }
    resp
}
