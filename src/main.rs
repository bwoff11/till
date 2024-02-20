use tokio::net::UdpSocket;
use dns::Message as DNSMessage;
use clap::{arg, Command};

mod dns;

#[tokio::main]
async fn main() {
    let cmd = Command::new("till")
        .bin_name("till")
        .version("0.1.0")
        .about("DNS client utility")
        .arg(arg!([domain] "The domain name to query").required(true));

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
                println!("Received {} bytes: {:?}", amt, &buf[..amt]);
                let response = DNSMessage::deserialize(&buf[..amt]);
                println!("Response: {:?}", response);
            },
            Err(e) => {
                eprintln!("Error creating DNSMessage: {}", e);
            },
        }
    }
}
