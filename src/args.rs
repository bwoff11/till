use clap::{Parser, ValueEnum};
use hickory_client::rr::Name;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "Brandon Wofford",
    about = "DNS query tool.",
    long_about = "Till is a DNS query tool. It sends a query to a DNS server and prints the response."
)]
pub struct Args {
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
        default_value = "a"
    )]
    record: RType,
    #[arg(
        help = "Transport protocol to use.",
        short = 't',
        long = "transport",
        default_value = "udp"
    )]
    transport: Transport,
    #[arg(
        help = "Output format to use.",
        short = 'o',
        long = "output",
        default_value = "inlined"
    )]
    output: Output,
}

impl Args {
    pub fn name(&self) -> Name {
        Name::from_str(&self.query).expect("Failed to parse domain name.")
    }

    pub fn socket_addr(&self) -> std::net::SocketAddr {
        format!("{}:{}", self.server, self.port)
            .parse()
            .expect("Failed to parse socket address.")
    }
}

#[derive(ValueEnum, Debug, Clone)]
enum RType {
    A,
    AAAA,
    ANAME,
    ANY,
    AXFR,
    CAA,
    CDS,
    CDNSKEY,
    CNAME,
    CSYNC,
    DNSKEY,
    DS,
    HINFO,
    HTTPS,
    IXFR,
    KEY,
    MX,
    NAPTR,
    NS,
    NSEC,
    NSEC3,
    NSEC3PARAM,
    NULL,
    OPENPGPKEY,
    OPT,
    PTR,
    RRSIG,
    SIG,
    SOA,
    SRV,
    SSHFP,
    SVCB,
    TLSA,
    TSIG,
    TXT,
    ZERO,
}

#[derive(ValueEnum, Debug, Clone)]
enum Transport {
    Udp,
    Tcp,
}

#[derive(ValueEnum, Debug, Clone)]
enum Output {
    Inlined,
    Json,
    Yaml,
}
