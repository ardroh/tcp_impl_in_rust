#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum State {
    Closed,
    Listen,
    // SynRcvd,
    // Estab,
}

impl Default for State {
    fn default() -> Self {
        State::Listen
    }
}

impl State {
    pub(crate) fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_hdr: etherparse::Ipv4HeaderSlice<'a>,
        tcp_hdr: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let buf = &mut [0u8; 1504];
        // TCP Connection State Diagram at https://www.rfc-editor.org/rfc/rfc793.html
        match *self {
            State::Closed => {
                return Ok(0); // ignore
            }
            State::Listen => {
                if !tcp_hdr.syn() {
                    eprintln!("expected SYN packet");
                    return Ok(0); // ignore
                }
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcp_hdr.destination_port(),
                    tcp_hdr.source_port(),
                    unimplemented!("seq"),
                    unimplemented!("window_size"),
                );
                syn_ack.syn = true;
                syn_ack.ack = true;
                let ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len_u16(),
                    64,
                    ip_hdr.protocol(),
                    [
                        ip_hdr.destination()[0],
                        ip_hdr.destination()[1],
                        ip_hdr.destination()[2],
                        ip_hdr.destination()[3],
                    ],
                    [
                        ip_hdr.source()[0],
                        ip_hdr.source()[1],
                        ip_hdr.source()[2],
                        ip_hdr.source()[3],
                    ],
                )?;
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten)?;
                    syn_ack.write(&mut unwritten)?;
                    unwritten.len()
                };
                let send_bytes = nic.send(&buf[..unwritten])?;
                println!(
                    "sent SYN-ACK: {} bytes to {}",
                    send_bytes,
                    ip_hdr.source_addr()
                );
                return Ok(send_bytes);
            } // State::SynRcvd => Ok(0),
              // State::Estab => Ok(0),
        }
    }
}
