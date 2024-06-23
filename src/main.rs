mod socket;

use socket::server::server_socket::ServerSocket;
use socket::raknet::message_identifier;

fn main() {

    let server = ServerSocket::new("127.0.0.1".to_string(), 19132);

    println!("Server is running!");

    loop {
        if let Some((packet, src)) = server.read_packet() {


            let mut id: u8 = packet[0];

            if id == message_identifier::ID_UNCONNECTED_PING {

            }
        }
    }
}
