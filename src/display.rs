use crate::args::Args;
use chrono::Local;
use hickory_client::op::DnsResponse;
use serde::Serialize;

pub struct Display {
    info: Info,
    message: Message,
}

pub struct Info {
    server: String,
    rtt: u128,
}

#[derive(Serialize)]
pub struct Message {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    extra: Vec<ResourceRecord>,
}

#[derive(Serialize)]
pub struct Header {
    id: u16,
    #[serde(rename = "qr")]
    message_type: String,
    #[serde(rename = "opcode")]
    op_code: String,
    #[serde(rename = "aa")]
    authoritative: bool,
    #[serde(rename = "tc")]
    truncated: bool,
    #[serde(rename = "rd")]
    recursion_desired: bool,
    #[serde(rename = "ra")]
    recursion_available: bool,
    #[serde(rename = "rcode")]
    response_code: String,
    #[serde(rename = "qdcount")]
    query_count: u16,
    #[serde(rename = "ancount")]
    answer_count: u16,
    #[serde(rename = "nscount")]
    name_server_count: u16,
    #[serde(rename = "arcount")]
    additional_count: u16,
}

#[derive(Serialize)]
pub struct Question {
    #[serde(rename = "qname")]
    name: String,
    #[serde(rename = "qtype")]
    query_type: String,
    #[serde(rename = "qclass")]
    query_class: String,
}

#[derive(Serialize)]
pub struct ResourceRecord {
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    #[serde(rename = "class")]
    record_class: String,
    ttl: u32,
    data: String,
}

impl Display {
    pub fn new(args: &Args, response: &DnsResponse, rtt: u128) -> Display {
        Display {
            info: Info {
                server: args.server().to_string(),
                rtt,
            },
            message: Message {
                header: Header {
                    id: response.header().id(),
                    message_type: response.header().message_type().to_string(),
                    op_code: response.header().op_code().to_string(),
                    authoritative: response.header().authoritative(),
                    truncated: response.header().truncated(),
                    recursion_desired: response.header().recursion_desired(),
                    recursion_available: response.header().recursion_available(),
                    response_code: response.header().response_code().to_string(),
                    query_count: response.header().query_count(),
                    answer_count: response.header().answer_count(),
                    name_server_count: response.header().name_server_count(),
                    additional_count: response.header().additional_count(),
                },
                questions: response
                    .queries()
                    .iter()
                    .map(|q| Question {
                        name: q.name().to_string(),
                        query_type: q.query_type().to_string(),
                        query_class: q.query_class().to_string(),
                    })
                    .collect(),
                answers: response
                    .answers()
                    .iter()
                    .map(|a| ResourceRecord {
                        name: a.name().to_string(),
                        record_type: a.record_type().to_string(),
                        record_class: a.dns_class().to_string(),
                        ttl: a.ttl(),
                        data: a
                            .data()
                            .map(|d| d.to_string())
                            .unwrap_or("No data".to_string()),
                    })
                    .collect(),
                authority: response
                    .name_servers()
                    .iter()
                    .map(|ns| ResourceRecord {
                        name: ns.name().to_string(),
                        record_type: ns.record_type().to_string(),
                        record_class: ns.dns_class().to_string(),
                        ttl: ns.ttl(),
                        data: "".to_string(),
                    })
                    .collect(),
                extra: response
                    .additionals()
                    .iter()
                    .map(|a| ResourceRecord {
                        name: a.name().to_string(),
                        record_type: a.record_type().to_string(),
                        record_class: a.dns_class().to_string(),
                        ttl: a.ttl(),
                        data: "".to_string(),
                    })
                    .collect(),
            },
        }
    }

    pub fn print_as_inline(&self) {
        println!(
            "INFO: SERVER: {}, RTT: {} ms, WHEN: {:?}",
            self.info.server,
            self.info.rtt,
            Local::now()
        );

        println!(
            "HEADER: ID: {}, QR: {}, OpCode: {}, AA: {}, TC: {}, RD: {}, RA: {}, RCODE: {}, QDCOUNT: {}, ANCOUNT: {}, NSCOUNT: {}, ARCOUNT: {}",
            self.message.header.id,
            self.message.header.message_type,
            self.message.header.op_code,
            self.message.header.authoritative,
            self.message.header.truncated,
            self.message.header.recursion_desired,
            self.message.header.recursion_available,
            self.message.header.response_code,
            self.message.header.query_count,
            self.message.header.answer_count,
            self.message.header.name_server_count,
            self.message.header.additional_count
        );

        for question in self.message.questions.iter() {
            println!(
                "QUESTION: QNAME: {}, QTYPE: {}, QCLASS: {}",
                question.name, question.query_type, question.query_class
            );
        }

        for answer in self.message.answers.iter() {
            println!(
                "ANSWER: NAME: {}, TYPE: {}, CLASS: {}, TTL: {}, DATA: {}",
                answer.name, answer.record_type, answer.record_class, answer.ttl, answer.data
            );
        }

        for authority in self.message.authority.iter() {
            println!(
                "AUTHORITY: NAME: {}, TYPE: {}, CLASS: {}, TTL: {}, DATA: {}",
                authority.name, authority.record_type, authority.record_class, authority.ttl, authority.data
            );
        }

        for extra in self.message.extra.iter() {
            println!(
                "EXTRA: NAME: {}, TYPE: {}, CLASS: {}, TTL: {}, DATA: {}",
                extra.name, extra.record_type, extra.record_class, extra.ttl, extra.data
            );
        }
    }

    pub fn print_as_json(&self) {
        match serde_json::to_string_pretty(&self.message) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Failed to serialize to JSON: {}", e),
        }
    }

    pub fn print_as_yaml(&self) {
        match serde_yaml::to_string(&self.message) {
            Ok(yaml) => println!("{}", yaml),
            Err(e) => eprintln!("Failed to serialize to YAML: {}", e),
        }
    }

    pub fn print_as_table(&self) {}
}
