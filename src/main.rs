mod socket;
mod bedrock;

use std::net::SocketAddr;
use socket::server::socket::ServerSocket;
use socket::raknet::message_identifier;
use socket::binary::types::{RaknetWriter, RaknetReader};


fn handle_unconnected_ping(server: &ServerSocket, src: SocketAddr, packet: &[u8]) {
    let mut reader = RaknetReader::new(packet);

    reader.read_byte().unwrap();

    let ping_time = reader.read_long().unwrap();
    let client_guid = reader.read_long().unwrap();

    let mut writer = RaknetWriter::new();
    writer.write_byte(message_identifier::ID_UNCONNECTED_PONG);
    writer.write_long(ping_time);
    writer.write_long(server.guid() as i64);
    writer.write_magic();


    let server_info = format!("MCPE;ServerName;123;1.19.1;0;10;{};Bedrock level;Survival;1;19132;19133;", server.guid());
    writer.write_string(&server_info);

    let response = writer.to_bytes();

    println!("{:?}", response);

    server.write_packet(&response, src);
}

fn main() {
    let server = ServerSocket::new("127.0.0.1:19132").expect("Failed to bind server");

    println!("Server is Running.");

    loop {
        if let Some((packet, src)) = server.read_packet() {
            let mut reader = RaknetReader::new(&packet);
            let packet_id = reader.read_byte().unwrap();

            if packet_id == message_identifier::ID_UNCONNECTED_PING {
                handle_unconnected_ping(&server, src, &packet);
            }



        } else {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
