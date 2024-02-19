use std::net::{Ipv4Addr, Ipv6Addr};
use regex::Regex;

/// Represents a network target, which can be an IPv4 address, an IPv6 address, or a domain name.
#[derive(Debug)]
pub struct Target {
    value: TargetType,
}

/// Defines the types of network targets supported.
#[derive(Debug)]
pub enum TargetType {
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
    Domain(String),
}

impl Target {
    /// Constructs a new `Target` from a string slice.
    ///
    /// The function attempts to parse the input as an IPv4 or IPv6 address.
    /// If parsing fails, it tries to validate the input as a domain name.
    ///
    /// # Arguments
    /// * `value` - A string slice representing the potential network target.
    ///
    /// # Returns
    /// * `Ok(Target)` if the input is a valid target.
    /// * `Err(String)` with an error message if the input is not a valid target.
    pub fn new(value: &str) -> Result<Self, String> {
        let parsed_value = if let Ok(ipv4) = value.parse::<Ipv4Addr>() {
            TargetType::IPv4(ipv4)
        } else if let Ok(ipv6) = value.parse::<Ipv6Addr>() {
            TargetType::IPv6(ipv6)
        } else {
            // If not an IP address, attempt to validate as a domain name
            Target::parse_as_domain(value)?
        };

        Ok(Target { value: parsed_value })
    }

    /// Validates and parses a domain name from a string slice.
    ///
    /// This function checks if the input string matches the pattern for a valid domain name.
    /// The regex used for validation matches typical domain name patterns, but it might not cover all valid cases.
    ///
    /// # Arguments
    /// * `value` - A string slice representing the domain name to validate.
    ///
    /// # Returns
    /// * `Ok(TargetType::Domain)` if the input is a valid domain name.
    /// * `Err(String)` with an error message if the input is not a valid domain name.
    fn parse_as_domain(value: &str) -> Result<TargetType, String> {
        // Regex for basic domain name validation
        let domain_regex = Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,6}$").unwrap();
        if domain_regex.is_match(value) {
            Ok(TargetType::Domain(value.to_string()))
        } else {
            Err(format!("'{}' is not a valid domain name", value))
        }
    }
}
