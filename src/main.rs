use clap::Parser;
use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RData, Record, RecordType};
use hickory_client::udp::UdpClientConnection;
use std::net::Ipv4Addr;
use std::str::FromStr;

mod display;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "Brandon Wofford",
    about = "DNS query tool.",
    long_about = "Till is a DNS query tool. It sends a query to a DNS server and prints the response."
)]
struct Args {
    #[arg(required = true, help = "Query to send.")]
    query: String,
    #[arg(
        help = "Server to send the query to.",
        short = 's',
        long = "server",
        default_value = "8.8.8.8"
    )]
    server: String,
    #[arg(
        help = "Port to send the query to.",
        short = 'p',
        long = "port",
        default_value = "53"
    )]
    port: u16,
    #[arg(
        help = "Record type of the query.",
        short = 'r',
        long = "record",
        default_value = "A"
    )]
    record: String,
    #[arg(
        help = "Transport protocol to use.",
        short = 't',
        long = "transport",
        default_value = "udp"
    )]
    transport: String,
}

fn main() {
    let args = Args::parse();

    let address = format!("{}:{}", args.server, args.port).parse().unwrap();
    let name = Name::from_str(&args.query).unwrap();

    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();

    display::Display::new(&response).print();
}
