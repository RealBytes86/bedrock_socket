mod socket;
use socket::server::server_socket::ServerSocket;

fn main() {
    let server = ServerSocket::new("127.0.0.1".to_string(), 19132);
    println!("Server is running!");

    loop {
        if let Some((packet, src)) = server.read_packet() {
            println!("Received packet from {}: {:?}", src, packet);
        }
    }
}
