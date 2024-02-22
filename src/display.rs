use crate::args::Args;
use hickory_client::op::DnsResponse;
use chrono::Local;

pub struct Display<'a> {
    args: &'a Args,
    message: &'a DnsResponse,
    rtt: u128,
}

impl<'a> Display<'a> {
    pub fn new(args: &'a Args, response: &'a DnsResponse, rtt: u128) -> Display<'a> {
        Display {
            args: args,
            message: response,
            rtt: rtt,
        }
    }

    pub fn print_as_inline(&self) {
        println!("INFO: SERVER: {}, RTT: {} ms, WHEN: {:?}", 
            self.args.server(), 
            self.rtt,
            Local::now()
        );


        println!(
            "HEADER: ID: {}, QR: {}, OpCode: {}, AA: {}, TC: {}, RD: {}, RA: {}, RCODE: {}, QDCOUNT: {}, ANCOUNT: {}, NSCOUNT: {}, ARCOUNT: {}",
            self.message.header().id(),
            self.message.header().message_type(),
            self.message.header().op_code(),
            self.message.header().authoritative(),
            self.message.header().truncated(),
            self.message.header().recursion_desired(),
            self.message.header().recursion_available(),
            self.message.header().response_code(),
            self.message.header().query_count(),
            self.message.header().answer_count(),
            self.message.header().name_server_count(),
            self.message.header().additional_count(),
        );

        for question in self.message.queries() {
            println!(
                "QUESTION: QNAME: {}, QTYPE: {}, QCLASS: {}",
                question.name(),
                question.query_type(),
                question.query_class()
            );
        }

        for answer in self.message.answers() {
            let data_string = match answer.data() {
                Some(rdata) => rdata.to_string(),
                None => "No data".to_string(),
            };
            println!(
                "ANSWER: NAME: {}, TYPE: {}, CLASS: {}, TTL: {}, DATA: {}",
                answer.name(),
                answer.record_type(),
                answer.record_type(),
                answer.ttl(),
                data_string
            );
        }

        // Need to make sure this is working. Not getting anything during testing.
        for authority in self.message.name_servers() {
            println!(
                "AUTHORITY: NAME: {}, TYPE: {}, CLASS: {}, TTL: {}",
                authority.name(),
                authority.record_type(),
                authority.record_type(),
                authority.ttl()
            );
        }

        for additional in self.message.additionals() {
            println!(
                "ADDITIONAL: NAME: {}, TYPE: {}, CLASS: {}, TTL: {}",
                additional.name(),
                additional.record_type(),
                additional.record_type(),
                additional.ttl()
            );
        }
    }

    pub fn print_as_json(&self) {}

    pub fn print_as_yaml(&self) {}

    pub fn print_as_table(&self) {}
}
