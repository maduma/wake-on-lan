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

pub mod alias {
    use std::collections::HashMap;
    use std::env;
    use std::path::{Path, PathBuf};
    use std::fs;

    fn get_db_path() -> PathBuf {
        let home_dir = env::var("HOMEPATH").unwrap();
        Path::new(&home_dir).join(".wake_on_lan.json")
    }

    fn open_db() -> HashMap<String, String> {
        let db_file = get_db_path();
        match fs::read_to_string(db_file) {
            Ok(str) => serde_json::from_str::<HashMap<String, String>>(&str).unwrap(),
            _ => HashMap::new(),
        }
    }

    fn close_db(db: &HashMap<String, String>) {
        if db.is_empty() {
            return;
        }
        let json = serde_json::to_string(db).unwrap();
        let db_file = get_db_path();
        fs::write(db_file, json).unwrap();
    } 

    pub fn create_alias(alias: &str, mac: &str) {
        let db = &mut open_db();
        db.insert(alias.to_string(), mac.to_string());
        close_db(db);
    }

    pub fn remove_alias(_alias: &str) {
        let db = &mut open_db();
        db.remove(_alias);
        close_db(db)
    }

    pub fn get_alias(_alias: &str) -> Option<String> {
        let db = &open_db();
        db.get(_alias).map(|s|s.to_string())
    }
}
