mod socket;

use socket::server::socket::ServerSocket;
use socket::raknet::message_identifier;

fn main() {

    let server = ServerSocket::new("127.0.0.1".to_string(), 19132);

    loop
    {
        if let Some((packet, src)) = server.read_packet() {
            if packet[0] == message_identifier::ID_UNCONNECTED_PING {
                println!("{:?}", packet)
            }

        }
    }
}
