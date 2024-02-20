use clap::Parser;
use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, RecordType};
use hickory_client::udp::UdpClientConnection;

use args::Args;

mod display;
mod args;

fn main() {
    let args = Args::parse();

    let conn = UdpClientConnection::new(args.socket_addr()).unwrap();
    let client = SyncClient::new(conn);
    let response: DnsResponse = client.query(&args.name(), DNSClass::IN, RecordType::A).unwrap();

    display::Display::new(&response).print_as_json();
}
