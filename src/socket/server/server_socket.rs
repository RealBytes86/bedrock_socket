use std::net::{ UdpSocket, SocketAddr };
use std::thread::sleep;

pub struct ServerSocket {
    socket: UdpSocket
}

impl ServerSocket {
    pub fn new(ip: String, port: u16) -> Self {
        let address  = format!("{}:{}", ip, port);
        let socket = UdpSocket::bind(&address).expect("Could not bind address.");
        socket.set_nonblocking(true).expect("Could not set non-blocking");
        ServerSocket { socket }
    }

    pub fn read_packet(&self) -> Option<(Vec<u8>, SocketAddr)> {
        let mut buf = vec![0; 65535];
        match self.socket.recv_from(&mut buf) {
            Ok((size, src)) => Some((buf[..size].to_vec(), src)),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
            Err(e) => panic!("Encountered an error while receiving a packet: {:?}", e),
        }
    }

    pub fn write_packet(&self, buffer: &[u8], dest: &SocketAddr) -> usize {
        self.socket.send_to(buffer, dest).expect("Failed to send data")
    }

}