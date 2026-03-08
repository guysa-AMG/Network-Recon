use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn scan_port(addr: SocketAddr, port: u16) -> bool {
    let socket_addr = (addr.ip(), port).to_socket_addrs().unwrap().next().unwrap();
    let future = TcpStream::connect(&socket_addr);

    match timeout(Duration::from_secs(1), future).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}
