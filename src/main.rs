use args::Args;
use args::Output;
use clap::Parser;
use hickory_client::client::{Client as HClient, SyncClient as HSyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, RecordType};
use hickory_client::udp::UdpClientConnection;

mod args;
mod display;

fn main() {
    let args = Args::parse();

    let conn = UdpClientConnection::new(args.socket_addr()).unwrap();
    let client = HSyncClient::new(conn);
    let response: DnsResponse = client
        .query(&args.name(), DNSClass::IN, RecordType::A)
        .unwrap();

    let display = display::Display::new(&response);

    match args.output {
        Output::Json => display.print_as_json(),
        Output::Yaml => display.print_as_yaml(),
        Output::Inline => display.print_as_inline(),
        Output::Table => display.print_as_table(),
    }
}
