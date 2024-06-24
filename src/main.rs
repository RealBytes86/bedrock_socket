mod socket;

use socket::server::socket::ServerSocket;
use socket::raknet::message_identifier;

fn main() {

    let server = ServerSocket::new("127.0.0.1".to_string(), 19132);

    println!("Server is Running.");

    loop
    {
        if let Some((packet, src)) = server.read_packet() {

            let mut packet_id: u8 = packet[0];

            match packet_id {
                message_identifier::ID_UNCONNECTED_PING => println!("{:?}", packet),
                _ => println!("Unbekannte Packet {:?}", packet)
            }

        }
    }
}
