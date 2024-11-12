
use wake_on_lan::wol::wake_on_lan;

fn main() {
    let default_mac = "2c:f0:5d:e1:9e:d6";
    wake_on_lan(&default_mac, Some("172.18.100.68"));
    
}