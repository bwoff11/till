use regex::Regex;

/// Represents a DNS query for a domain name.
#[derive(Debug)]
pub struct DNSQuery {
    domain: String,
}

impl DNSQuery {
    pub fn new(domain: &str) -> Result<Self, String> {
        let domain_regex = Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,6}$").unwrap();
        if domain_regex.is_match(domain) {
            Ok(DNSQuery {
                domain: domain.to_string(),
            })
        } else {
            Err(format!("Invalid domain: {}", domain))
        }
    }
}
