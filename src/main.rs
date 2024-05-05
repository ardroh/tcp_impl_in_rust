fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                println!(
                    "{} -> {}, protocol: {:?}, read {} bytes",
                    ip_hdr.source_addr(),
                    ip_hdr.destination_addr(),
                    ip_hdr.protocol(),
                    nbytes,
                );
            }
            Err(e) => {
                eprintln!("not an IPv4 packet: {:?}", e);
            }
        }
    }
    Ok(())
}
