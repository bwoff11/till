use std::net::{Ipv4Addr, Ipv6Addr};
use regex::Regex;

#[derive(Debug)]
pub struct Target {
    value: TargetType,
}

#[derive(Debug)]
pub enum TargetType {
    IPv4(std::net::Ipv4Addr),
    IPv6(std::net::Ipv6Addr),
    Domain(String),
}

impl Target {
    pub fn new(value: &str) -> Result<Self, String> {
        let parsed_value = if let Ok(ipv4) = value.parse::<Ipv4Addr>() {
            TargetType::IPv4(ipv4)
        } else if let Ok(ipv6) = value.parse::<Ipv6Addr>() {
            TargetType::IPv6(ipv6)
        } else {
            Target::parse_as_domain(value)?
        };

        Ok(Target {
            value: parsed_value,
        })
    }

    fn parse_as_domain(value: &str) -> Result<TargetType, String> {
        let domain_regex = Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,6}$").unwrap();
        if domain_regex.is_match(value) {
            Ok(TargetType::Domain(value.to_string()))
        } else {
            Err(format!("'{}' is not a valid domain name", value))
        }
    }
}
