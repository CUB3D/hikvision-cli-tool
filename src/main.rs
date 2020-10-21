use std::time::Duration;
use std::net::UdpSocket;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use serde::de::value::StringDeserializer;

struct Probe {
    uuid: String,
    types: String,
}

struct ProbeMatch {
    uuid: String,
    type_: String,
    device_type: String,
    device_description: String,
    device_sn: String,
    command_port: String,
    http_port: String,
    mac: String,
    //ipv4_address: String,
    // ipv6
    dhcp: bool,

}

fn main() {
    let socket:UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_broadcast(true).unwrap();
    println!("Broadcast: {:?}", socket.broadcast());
    println!("Timeout: {:?}", socket.read_timeout());


    socket.send_to("<?xml version=\"1.0\" encoding=\"utf-8\"?><Probe><Uuid>000000F4-0036-0047-A177-F1CFC6BC35D2</Uuid><Types>inquiry</Types></Probe>".as_bytes(), "255.255.255.255:37020");

    // let call:Vec<u8> = self.packer.get_buf()?;
    // println!("Sending call, {} bytes", call.len());
    // match self.socket.send(&call) {
    //     Ok(n) => {
    //         if n != call.len() {
    //             return Err(Error::new(ErrorKind::Other, "Sent the wrong number of bytes"))
    //         }
    //         else {
    //             // Do nothing because we sent the number of bytes we expected to send
    //         }
    //     },
    //     Err(e) => return Err(e),
    // }

    println!("Awaiting responses...");   // self.recv_buff is a [u8; 8092]
    let mut recv_buff = [0u8; 5000];

    loop {
        let (n, addr) = socket.recv_from(&mut recv_buff).unwrap();
        println!("{} bytes response from {:?}", n, addr);
        let buf = &recv_buff[0..n];
        let data = String::new();
        println!("{:?}", data);
    }
}
