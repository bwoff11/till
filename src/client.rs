use hickory_client::client::{Client as HClient, SyncClient as HSyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, RecordType};
use hickory_client::udp::UdpClientConnection;

pub struct Client<'a> {
    args: &'a super::args::Args,
}

impl<'a> Client<'a> {
    pub fn new(args: &'a super::args::Args) -> Self {
        Self { args: args }
    }

    pub fn query(&self) -> DnsResponse {
        let conn = UdpClientConnection::new(self.args.socket_addr()).unwrap();
        let client = HSyncClient::new(conn);
        let response: DnsResponse = client.query(&self.args.name(), DNSClass::IN, RecordType::A).unwrap();
        response
    }
}
