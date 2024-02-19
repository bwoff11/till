use clap::arg;
use clap::Command;
use target::{Target, TargetType};

mod target;

fn main() {
    let cmd = Command::new("till")
        .bin_name("till")
        .version("0.1.0")
        .about("DNS client utility")
        .arg(arg!([host] "The host to connect to")
            .required(true))
        .arg(arg!([port] "The port to connect to")
            .required(false)
            .default_value("53"));

    let matches = cmd.get_matches();

    if let Some(host_str) = matches.get_one::<String>("host") {
        match Target::new(host_str) {
            Ok(target) => {
                println!("Host is valid: {:?}", target);
                // proceed here
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
