#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct State {}

impl Default for State {
    fn default() -> Self {
        State {}
    }
}

impl State {
    pub(crate) fn on_packet<'a>(
        &mut self,
        ip_hdr: etherparse::Ipv4HeaderSlice<'a>,
        tcp_hdr: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        println!(
            "TCP packet received {}:{} -> {}:{}, payload size {} bytes",
            ip_hdr.source_addr(),
            tcp_hdr.source_port(),
            ip_hdr.destination_addr(),
            tcp_hdr.destination_port(),
            data.len()
        );
    }
}
