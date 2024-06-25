
use crate::socket::server::socket::ServerSocket;

pub trait Server {
    fn on_connect(&self);
    fn on_disconnect(&self);
    fn on_packet_ack(&self);
    fn on_packet(&self);
}

impl Server for ServerSocket {

    fn on_connect(&self) {

    }
    fn on_disconnect(&self) {

    }

    fn on_packet_ack(&self) {

    }

    fn on_packet(&self) {

    }
}