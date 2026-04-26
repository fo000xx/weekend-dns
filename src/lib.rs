use std::net::UdpSocket;

pub mod protocol;
pub mod reader;

pub use protocol::{build_query, DNSPacket, CLASS_IN, TYPE_A, TYPE_NS};
pub use reader::Reader;

pub fn resolve(domain_name: &str, record_type: u16) -> String {
    let mut nameserver = "198.41.0.4".to_string(); // a.root-servers.net
    loop {
        println!("Querying {} for {}", nameserver, domain_name);
        let response = send_query(&nameserver, domain_name, record_type);
        
        if let Some(ip) = get_answer(&response) {
            return ip;
        } else if let Some(ns_ip) = get_nameserver_ip(&response) {
            nameserver = ns_ip;
        } else if let Some(ns_domain) = get_nameserver(&response) {
            nameserver = resolve(&ns_domain, TYPE_A);
        } else {
            panic!("Failed to resolve {}", domain_name);
        }
    }
}

fn get_answer(packet: &DNSPacket) -> Option<String> {
    for answer in &packet.answers {
        if answer.rtype == TYPE_A {
            return Some(format!(
                "{}.{}.{}.{}",
                answer.data[0], answer.data[1], answer.data[2], answer.data[3]
            ));
        }
    }
    None
}

fn get_nameserver_ip(packet: &DNSPacket) -> Option<String> {
    for additional in &packet.additionals {
        if additional.rtype == TYPE_A {
            return Some(format!(
                "{}.{}.{}.{}",
                additional.data[0], additional.data[1], additional.data[2], additional.data[3]
            ));
        }
    }
    None
}

fn get_nameserver(packet: &DNSPacket) -> Option<String> {
    for authority in &packet.authorities {
        if authority.rtype == TYPE_NS {
            return Some(String::from_utf8_lossy(&authority.data).to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::header::Header;
    use crate::protocol::record::Record;

    #[test]
    fn test_get_answer() {
        let packet = DNSPacket {
            header: Header::default(),
            questions: vec![],
            answers: vec![Record {
                name: "example.com".to_string(),
                rtype: TYPE_A,
                rclass: 1,
                ttl: 60,
                data: vec![93, 184, 216, 34],
            }],
            authorities: vec![],
            additionals: vec![],
        };
        assert_eq!(get_answer(&packet), Some("93.184.216.34".to_string()));
    }

    #[test]
    fn test_get_nameserver_ip() {
        let packet = DNSPacket {
            header: Header::default(),
            questions: vec![],
            answers: vec![],
            authorities: vec![],
            additionals: vec![Record {
                name: "ns1.example.com".to_string(),
                rtype: TYPE_A,
                rclass: 1,
                ttl: 60,
                data: vec![1, 1, 1, 1],
            }],
        };
        assert_eq!(get_nameserver_ip(&packet), Some("1.1.1.1".to_string()));
    }

    #[test]
    fn test_get_nameserver() {
        let packet = DNSPacket {
            header: Header::default(),
            questions: vec![],
            answers: vec![],
            authorities: vec![Record {
                name: "example.com".to_string(),
                rtype: TYPE_NS,
                rclass: 1,
                ttl: 60,
                data: "ns1.example.com".to_string().into_bytes(),
            }],
            additionals: vec![],
        };
        assert_eq!(get_nameserver(&packet), Some("ns1.example.com".to_string()));
    }

    #[test]
    #[ignore] // Requires internet access
    fn test_resolve_integration() {
        let ip = resolve("google.com", TYPE_A);
        // Google has many IPs, but we just check if it's a valid-looking IPv4
        assert!(ip.split('.').count() == 4);
        assert!(ip.chars().all(|c| c.is_digit(10) || c == '.'));
    }
}

pub fn send_query(ip_address: &str, domain_name: &str, record_type: u16) -> DNSPacket {
    let query = build_query(domain_name.to_string(), record_type);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to local socket");

    let server_addr = if ip_address.contains(':') {
        ip_address.to_string()
    } else {
        format!("{}:53", ip_address)
    };

    socket
        .send_to(&query, server_addr)
        .expect("Failed to send query");

    let mut buf = [0u8; 1024];
    let (amt, _) = socket
        .recv_from(&mut buf)
        .expect("Failed to receive response");

    let mut reader = Reader::new(&buf[..amt]);
    DNSPacket::from_reader(&mut reader)
}

pub fn lookup_domain(domain: &str) {
    // Use our new send_query function
    let packet = send_query("8.8.8.8", domain, TYPE_A);

    // Print the IP addresses from the A records
    for answer in packet.answers {
        if answer.rtype == TYPE_A && answer.data.len() == 4 {
            let ip = format!(
                "{}.{}.{}.{}",
                answer.data[0], answer.data[1], answer.data[2], answer.data[3]
            );
            println!("{} has address {}", domain, ip);
        }
    }
}
