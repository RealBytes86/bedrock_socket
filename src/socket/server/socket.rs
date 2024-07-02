use std::io;
use std::net::{UdpSocket, SocketAddr};

pub struct ServerSocket {
    socket: UdpSocket,
}

impl ServerSocket {
    pub fn new(bind_address: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(bind_address)?;
        socket.set_nonblocking(true)?;
        Ok(ServerSocket { socket })
    }

    pub fn read_packet(&self) -> Option<(Vec<u8>, SocketAddr)> {
        let mut buf = [0; 65535];
        match self.socket.recv_from(&mut buf) {
            Ok((size, addr)) => Some((buf[..size].to_vec(), addr)),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
            Err(e) => {
                eprintln!("recv_from error: {}", e);
                None
            }
        }
    }

    pub fn write_packet(&self, buffer: &[u8], dest: SocketAddr) -> usize {
        match self.socket.send_to(buffer, dest) {
            Ok(size) => size,
            Err(e) => {
                eprintln!("send_to error: {}", e);
                0
            }
        }
    }

    pub fn guid(&self) -> u64 {
        12345678901234567890
    }
}
