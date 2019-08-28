use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use super::SphinxCore::Env::*;

pub struct Server {
    pub sx: UdpSocket,
    pub rx: UdpSocket,
}

impl Server {
    pub fn new() -> Self {
        Self {
            rx: UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, RX_PORT)).expect("rx UDP Bind failed"),
            sx: UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, SX_PORT)).expect("sx UDP Bind failed"),
        }
    }
    pub fn Run() {}
}
