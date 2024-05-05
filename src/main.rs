use std::{collections::HashMap, net::Ipv4Addr};

use etherparse::IpNumber;

mod tcp;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connections: HashMap<Quad, tcp::State> = HashMap::new();
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = vec![0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        // let eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        //https://en.wikipedia.org/wiki/EtherType
        const ETH_P_IP: u16 = 0x0800;
        if eth_proto != ETH_P_IP {
            // not IPv4
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_hdr) => {
                if ip_hdr.protocol() != IpNumber(6) {
                    // not TCP
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip_hdr.slice().len()..nbytes])
                {
                    Ok(tcp_hdr) => {
                        let datai = 4 + ip_hdr.slice().len() + tcp_hdr.slice().len();
                        connections
                            .entry(Quad {
                                src: (ip_hdr.source_addr(), tcp_hdr.source_port()),
                                dst: (ip_hdr.destination_addr(), tcp_hdr.destination_port()),
                            })
                            .or_default()
                            .on_packet(ip_hdr, tcp_hdr, &buf[datai..nbytes]);
                    }
                    Err(e) => {
                        eprintln!("not a TCP packet: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("not an IPv4 packet: {:?}", e);
            }
        }
    }
}
