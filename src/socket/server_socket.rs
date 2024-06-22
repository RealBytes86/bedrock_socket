use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::thread::sleep;

struct ServerSocket {
    socket: UdpSocket
}

impl ServerSocket {
    fn new(ip: String, port: u16) -> Self {
        let address  = format!("{}:{}", ip, port);
        let socket = UdpSocket::bind(&address).expect("Could not bind address.");
        socket.set_nonblocking(true).expect("Could not set non-blocking");
        ServerSocket { socket }
    }

    fn read_packet(&self) -> Option<(Vec<u8>, SocketAddr)> {
        let mut
    }

}