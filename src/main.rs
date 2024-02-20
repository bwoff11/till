use tokio::net::UdpSocket;
use dns::Message as DNSMessage;
use output::Output;
use clap::{arg, Command};

mod dns;
mod output;

#[tokio::main]
async fn main() {
    let cmd = Command::new("till")
        .bin_name("till")
        .version("0.1.0")
        .about("DNS client utility")
        .author("Brandon Wofford")
        .arg(arg!([domain] "The domain name to query").required(true))
        .arg(arg!(-t --type [TYPE] "The type of query to perform")
            .default_value("A"))
        .arg(arg!(-s --server [SERVER] "The DNS server to query")
            .default_value("127.0.0.1"))
        .arg(arg!(-p --port [PORT] "The port to send the query to")
            .default_value("53"));

    let matches = cmd.get_matches();

    if let Some(domain_str) = matches.get_one::<String>("domain") {
        match DNSMessage::new(domain_str) {
            Ok(msg) => {
                let data = msg.serialize();
                // Assuming "data" is a Vec<u8> or similar
                
                let server_address = "10.0.0.27:53";
                
                // Bind to any address to send data
                let socket = UdpSocket::bind("0.0.0.0:0").await.expect("couldn't bind to address");
                
                // Send the serialized message
                socket.send_to(&data, server_address).await.expect("couldn't send data");
                
                let mut buf = [0u8; 4096]; // Buffer for the response
                let (amt, _) = socket.recv_from(&mut buf).await.expect("didn't receive data");
                
                // Handle the response here...
                let response = DNSMessage::deserialize(&buf[..amt]).unwrap();

                let output = Output::new(response);
                output.print();

            },
            Err(e) => {
                eprintln!("Error creating DNSMessage: {}", e);
            },
        }
    }
}
