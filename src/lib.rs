pub mod wol {
    use std::net::{Ipv4Addr, UdpSocket};

    #[derive(PartialEq, Debug)]
    enum ParseMacError {
        NotHex,
        BadLenght,
    }

    fn send_udp_broadcast_packet(buf: &[u8], src_ip: Option<&str>) {
        let src_ip = match src_ip {
            None => Ipv4Addr::UNSPECIFIED,
            Some(s) => s.parse().unwrap(),
        };
        let socket: UdpSocket = UdpSocket::bind((src_ip, 0)).unwrap();
        socket.connect((Ipv4Addr::BROADCAST, 0)).unwrap();
        socket.send(buf).unwrap();
        println!(
            "Using source IP {} to send udp frame",
            socket.local_addr().unwrap().ip()
        );
        drop(socket);
    }

    fn parse_mac(mac: &str) -> Result<Vec<u8>, ParseMacError> {
        let mac = mac
            .split(':')
            .map(|s| u8::from_str_radix(s, 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseMacError::NotHex)?;
        match mac.len() {
            6 => Ok(mac),
            _ => Err(ParseMacError::BadLenght),
        }
    }

    pub fn is_mac(mac: &str) -> bool {
        match parse_mac(mac) {
            Ok(_) => true,
            _ => false,
        }
    }

    fn create_magic_wol_frame(mac: &str) -> Vec<u8> {
        let mut buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let mac = parse_mac(mac).unwrap();
        for _ in 0..16 {
            buf.append(&mut mac.clone());
        }
        buf
    }

    pub fn wake_on_lan(mac: &str, src_ip: Option<&str>) {
        let buf = create_magic_wol_frame(mac);
        send_udp_broadcast_packet(&buf, src_ip);
        println!("WOL sent to {mac}");
    }
}
