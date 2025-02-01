use std::net::{Ipv4Addr, UdpSocket};
use std::iter;

type Mac = [u8; 6];
type WOLFrame = [u8; 102];

#[derive(PartialEq, Debug)]
enum ParseMacError {
    NotHex,
    BadLenght,
}

fn send_udp_broadcast_packet(buf: &WOLFrame, src_ip: Ipv4Addr) {
    let socket: UdpSocket = UdpSocket::bind((src_ip, 0)).unwrap();
    socket.connect((Ipv4Addr::BROADCAST, 0)).unwrap();
    socket.send(buf).unwrap();
    println!(
        "Using source IP {} to send udp frame",
        socket.local_addr().unwrap().ip()
    );
    drop(socket);
}

fn parse_mac(word: &str) -> Result<Mac, ParseMacError> {
    let bytes = word
        .split(':')
        .map(|s| u8::from_str_radix(s, 16))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ParseMacError::NotHex)?;
    match bytes.len() {
        6 => Ok(bytes.try_into().unwrap()),
        _ => Err(ParseMacError::BadLenght),
    }
}

pub fn is_mac(word: &str) -> bool {
    match parse_mac(word) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn create_magic_wol_frame(mac: &Mac) -> WOLFrame {
    let mut buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    buf.extend(iter::repeat(mac).take(16).flatten());
    buf.try_into().unwrap()
}

pub fn wake_on_lan(mac: &str, src_ip: Option<&str>) {
    let bytes = parse_mac(mac).unwrap();
    let buf = create_magic_wol_frame(&bytes);
    let src_ip = match src_ip {
        Some(s) => s.parse().unwrap(),
        None => Ipv4Addr::UNSPECIFIED,
    };
    send_udp_broadcast_packet(&buf, src_ip);
    println!("WOL sent to {mac}");
}