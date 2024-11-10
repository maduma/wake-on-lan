use std::{
    net::{Ipv4Addr, UdpSocket},
    vec,
};

fn send_udp_broadcast_packet(buf: &[u8], src_ip: Option<&str>) {
    let src_ip = match src_ip {
        None => Ipv4Addr::UNSPECIFIED,
        Some(s) => s.parse().unwrap(),
    };
    let socket: UdpSocket = UdpSocket::bind((src_ip, 0)).unwrap();
    socket.connect((Ipv4Addr::BROADCAST, 0)).unwrap();
    socket.send(buf).unwrap();
    println!("Using source IP {}", socket.local_addr().unwrap().ip());
    drop(socket);
}

fn parse_mac(mac: &str) -> Vec<u8> {
    mac.split(':')
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect::<Vec<_>>()
}

fn create_magic_wol_frame(mac: &str) -> Vec<u8> {
    let mut buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let mac = parse_mac(mac);
    for _ in 0..16 {
        buf.append(&mut mac.clone());
    }
    buf
}
fn main() {
    let default_mac = "2c:f0:5d:e1:9e:d6";
    let buf = create_magic_wol_frame(default_mac);
    send_udp_broadcast_packet(&buf, None);
    println!("Sending WOL to {default_mac}");
}
