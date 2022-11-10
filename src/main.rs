#[tokio::main]
async fn main() {
    for source_ip in ["127.0.0.1", "127.0.0.1:8080", ":8080"] {
        let result1 = source_ip.parse::<std::net::SocketAddr>();
        let result2 = source_ip.parse::<std::net::IpAddr>();
        let result3 = match result2 {
            Ok(addr) => Some(std::net::SocketAddr::from((addr, 0))),
            _ => None,
        };
        eprintln!("{}: {:?}, {:?}, {:?}", source_ip, result1, result2, result3);
    }
}
