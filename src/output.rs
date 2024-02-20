use crate::dns::Message as DNSMessage;
use crate::dns::RecordType;
use tabled::{builder::Builder, settings::Style};
use tabled::{Table, Tabled};

pub struct Output {
    message: DNSMessage,
}

impl Output {
    pub fn new(msg: DNSMessage) -> Output {
        Output { message: msg }
    }
    pub fn print(&self) {
        self.print_header();
        self.print_questions();
        self.print_answers();
        self.print_authority();
        self.print_additional();
    }

    fn print_header(&self) {
        let header = self.message.header;
        let mut builder = Builder::new();
        builder.push_record(["ID", &header.id.to_string()]);
        builder.push_record(["QR", &header.qr.to_string()]);
        builder.push_record(["Opcode", &header.opcode.to_string()]);
        builder.push_record(["AA", &header.aa.to_string()]);
        builder.push_record(["TC", &header.tc.to_string()]);
        builder.push_record(["RD", &header.rd.to_string()]);
        builder.push_record(["RA", &header.ra.to_string()]);
        builder.push_record(["Z", &header.z.to_string()]);
        builder.push_record(["RCode", &header.rcode.to_string()]);
        builder.push_record(["QDCount", &header.qdcount.to_string()]);
        builder.push_record(["ANCount", &header.ancount.to_string()]);
        builder.push_record(["NSCount", &header.nscount.to_string()]);
        builder.push_record(["ARCount", &header.arcount.to_string()]);

        let table = builder.build()
            .with(Style::ascii_rounded())
            .to_string();

        println!("Header");
        println!("{}", table);
    }

    fn print_questions(&self) {
        let questions = &self.message.question; // Take a reference to the questions
        let mut builder = Builder::new();
        for question in questions {
            builder.push_record(["QName", &question.qname.join(".")]);
            builder.push_record(["QType", &question.qtype.to_string()]);
            builder.push_record(["QClass", &question.qclass.to_string()]);
        }
    
        let table = builder.build()
            .with(Style::ascii_rounded())
            .to_string();
    
        println!("Questions");
        println!("{}", table);
    }    

    fn print_answers(&self) {
        let answers = &self.message.answer;
        let mut builder = Builder::new();
        for answer in answers {
            let rdata_string = match answer.rtype {
                RecordType::A => {
                    if answer.rdlength == 4 {
                        // Assuming rdata represents an IPv4 address
                        format!(
                            "{}.{}.{}.{}",
                            answer.rdata[0], answer.rdata[1], answer.rdata[2], answer.rdata[3]
                        )
                    } else {
                        format!("Invalid IPv4 address format")
                    }
                }
                // Add more cases for other record types as needed
                _ => format!("{:?}", answer.rdata), // Print raw bytes for unknown types
            };
            builder.push_record(["Name", &answer.name.join(".")]);
            builder.push_record(["Type", &answer.rtype.to_string()]);
            builder.push_record(["Class", &answer.rclass.to_string()]);
            builder.push_record(["TTL", &answer.ttl.to_string()]);
            builder.push_record(["RDLength", &answer.rdlength.to_string()]);
            builder.push_record(["RData", &rdata_string]);
        }
    
        let table = builder.build()
            .with(Style::ascii_rounded())
            .to_string();
    
        println!("Answers");
        println!("{}", table);
    }
    

    fn print_authority(&self) {
        let authority = &self.message.authority;
        let mut builder = Builder::new();
        for record in authority {
            let rdata_hex = record.rdata.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");
            builder.push_record(["Name", &record.name.join(".")]);
            builder.push_record(["Type", &record.rtype.to_string()]);
            builder.push_record(["Class", &record.rclass.to_string()]);
            builder.push_record(["TTL", &record.ttl.to_string()]);
            builder.push_record(["RDLength", &record.rdlength.to_string()]);
            builder.push_record(["RData", &rdata_hex]);
        }
    
        let table = builder.build()
            .with(Style::ascii_rounded())
            .to_string();
    
        println!("Authority");
        println!("{}", table);
    }

    fn print_additional(&self) {
        let additional = &self.message.extra;
        let mut builder = Builder::new();
        for record in additional {
            let rdata_hex = record.rdata.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");
            builder.push_record(["Name", &record.name.join(".")]);
            builder.push_record(["Type", &record.rtype.to_string()]);
            builder.push_record(["Class", &record.rclass.to_string()]);
            builder.push_record(["TTL", &record.ttl.to_string()]);
            builder.push_record(["RDLength", &record.rdlength.to_string()]);
            builder.push_record(["RData", &rdata_hex]);
        }
    
        let table = builder.build()
            .with(Style::ascii_rounded())
            .to_string();
    
        println!("Additional");
        println!("{}", table);
    }
    
}
