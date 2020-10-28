use crate::types::{Probe, ProbeMatch};
use std::net::UdpSocket;
use std::time::Duration;

pub struct HvClient {
    pub socket: UdpSocket,
}

impl Default for HvClient {
    fn default() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_broadcast(true).unwrap();

        Self { socket }
    }
}

impl HvClient {
    pub fn new_with_timeout(dur: Duration) -> Self {
        let client = Self::default();
        client.socket.set_read_timeout(Some(dur)).unwrap();
        client
    }

    pub fn send_broadcast(&self, probe: &Probe) -> std::io::Result<usize> {
        self.socket.send_to(
            serde_xml_rs::to_string(probe).unwrap().as_bytes(),
            "255.255.255.255:37020",
        )
    }

    pub fn read_packet(&self) -> Option<ProbeMatch> {
        let mut recv_buff = [0u8; 5000];
        self.socket
            .recv_from(&mut recv_buff)
            .map(|(size, _addr)| {
                let cropped_buf = &recv_buff[..size];
                std::str::from_utf8(cropped_buf)
                    .map(|str| serde_xml_rs::from_str::<ProbeMatch>(str).unwrap())
                    .unwrap()
            })
            .ok()
    }
}
