use clap::arg;
use clap::Command;
use dns_query::DNSQuery;

mod dns_query;

fn main() {
    let cmd = Command::new("till")
        .bin_name("till")
        .version("0.1.0")
        .about("DNS client utility")
        .arg(arg!([domain] "The domain name to query")
            .required(true));

    let matches = cmd.get_matches();

    // Make sure to use "domain" when retrieving the match
    if let Some(domain_str) = matches.get_one::<String>("domain") {
        match DNSQuery::new(domain_str) {
            Ok(target) => {
                println!("Domain is valid: {:?}", target);
                // Proceed with DNS query logic here
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
