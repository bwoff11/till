use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct Message {
    /// The header section of the message.
    pub header: MessageHeader,
    /// A flag indicating whether domain name compression is used in the message.
    compress: bool,
    /// Questions are queries the client has for the server.
    pub question: Vec<Question>,
    /// Answers are responses from the server to the client's questions.
    pub answer: Vec<ResourceRecord>,
    /// Authority records are the servers that are authoritative for the domain in the question section.
    pub authority: Vec<ResourceRecord>,
    /// Additional records contain extra information that may be helpful in processing the response.
    pub extra: Vec<ResourceRecord>,
}

#[derive(Debug, Copy, Clone)]
pub struct MessageHeader {
    /// Assigned by the program that generates any kind of query.
    /// This identifier is copied into the response.
    pub id: u16,
    /// Specifies whether this message is a query (0) or a response (1).
    pub qr: u8,
    /// Specifies the kind of query in this message. 0 represents a standard query (QUERY).
    pub opcode: u8,
    /// Authoritative Answer - set in responses to indicate that the responding server is an authority for the domain.
    pub aa: u8,
    /// Truncation - indicates that this message was truncated.
    pub tc: u8,
    /// Recursion Desired - directs the server to pursue the query recursively.
    pub rd: u8,
    /// Recursion Available - set or cleared in a response to indicate recursive query support.
    pub ra: u8,
    /// Reserved for future use. Must be zero in all queries and responses.
    pub z: u8,
    /// Response code - set as part of responses and indicates success or failure of the query.
    pub rcode: ResponseCode,
    /// The number of entries in the question section.
    pub qdcount: u16,
    /// The number of resource records in the answer section.
    pub ancount: u16,
    /// The number of name server resource records in the authority records section.
    pub nscount: u16,
    /// The number of resource records in the additional records section.
    pub arcount: u16,
}

#[derive(Debug, Clone)]
pub struct Question {
    /// The domain name that is the subject of the query.
    pub qname: Vec<String>,
    /// Specifies the type of the query.
    pub qtype: u16,
    /// Specifies the class of the query.
    pub qclass: u16,
}

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: Vec<String>,
    pub rtype: RecordType,
    pub rclass: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RecordType {
    A,     // = 1, RFC 1035
    AAAA,  // = 28, RFC 3596
    CNAME, // = 5, RFC 1035
    MX,    // = 15, RFC 1035
    NS,    // = 2, RFC 1035
    PTR,   // = 12, RFC 1035
    SOA,   // = 6, RFC 1035
    SRV,   // = 33, RFC 2782
    TXT,   // = 16, RFC 1035
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ResponseCode {
    /// DNS Query completed successfully
    NOERROR = 0,
    /// DNS Query Format Error
    FORMERR = 1,
    /// Server failed to complete the DNS request
    SERVFAIL = 2,
    /// Domain name does not exist
    NXDOMAIN = 3,
    /// Function not implemented
    NOTIMP = 4,
    /// The server refused to answer for the query
    REFUSED = 5,
    /// Name that should not exist, does exist
    YXDOMAIN = 6,
    /// RRset that should not exist, does exist
    XRRSET = 7,
    /// Server not authoritative for the zone
    NOTAUTH = 9,
    /// Name not in zone
    NOTZONE = 10,
}

impl Message {
    pub fn new(domain: &String) -> Result<Message, String> {
        if domain.is_empty() {
            return Err("Domain cannot be empty".to_string());
        }

        Ok(Message {
            header: MessageHeader::new(), // Assuming this cannot fail
            compress: false,
            question: vec![Question {
                qname: domain.split('.').map(|s| s.to_string()).collect(),
                qtype: 1,
                qclass: 1,
            }],
            answer: vec![],
            authority: vec![],
            extra: vec![],
        })
    }

    fn parse_qname(
        data: &[u8],
        start_offset: usize,
    ) -> Result<(Vec<String>, usize), std::io::Error> {
        let mut offset = start_offset;
        let mut labels = Vec::new();
        let mut length = data[offset] as usize;

        while length > 0 {
            offset += 1; // Move past the length byte
            let label = std::str::from_utf8(&data[offset..offset + length]).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid QNAME")
            })?;
            labels.push(label.to_string());

            offset += length; // Move past the current label
            length = data[offset] as usize; // Length of the next label
        }

        Ok((labels, offset + 1)) // Return the labels and the new offset (after the null byte)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.serialize());
        for question in &self.question {
            bytes.extend_from_slice(&question.serialize());
        }
        for answer in &self.answer {
            bytes.extend_from_slice(&answer.serialize());
        }
        for authority in &self.authority {
            bytes.extend_from_slice(&authority.serialize());
        }
        for extra in &self.extra {
            bytes.extend_from_slice(&extra.serialize());
        }
        bytes
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut offset = 0;
    
        let header = MessageHeader::deserialize(&data[offset..])?;
        offset += 12; // Header is 12 bytes
    
        let mut question = Vec::new();
        for _ in 0..header.qdcount {
            let (q, new_offset) = Question::deserialize(data, offset)?;
            offset = new_offset;
            question.push(q);
        }
    
        // Initialize vectors for each section
        let mut answer = Vec::new();
        let mut authority = Vec::new();
        let mut extra = Vec::new();
    
        // Deserialize answer section
        for _ in 0..header.ancount {
            let (record, new_offset) = ResourceRecord::deserialize(data, offset)?;
            offset = new_offset;
            answer.push(record);
        }
    
        // Deserialize authority section
        for _ in 0..header.nscount {
            let (record, new_offset) = ResourceRecord::deserialize(data, offset)?;
            offset = new_offset;
            authority.push(record);
        }
    
        // Deserialize additional section
        for _ in 0..header.arcount {
            let (record, new_offset) = ResourceRecord::deserialize(data, offset)?;
            offset = new_offset;
            extra.push(record);
        }
    
        Ok(Message {
            header,
            compress: false,
            question,
            answer,
            authority,
            extra,
        })
    }
}    

impl Question {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in &self.qname {
            bytes.push(label.len() as u8); // Length octet
            bytes.extend_from_slice(label.as_bytes()); // Label octets
        }
        bytes.push(0); // Null byte to end QNAME
        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.qclass.to_be_bytes());
        bytes
    }

    pub fn deserialize(data: &[u8], start_offset: usize) -> Result<(Self, usize), std::io::Error> {
        let mut offset = start_offset;
        let (qname, new_offset) = Message::parse_qname(data, offset)?;
        offset = new_offset; // Update offset to position after QNAME

        if offset + 4 > data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected end of data",
            ));
        }

        let qtype = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let qclass = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
        offset += 4; // Move past qtype and qclass

        Ok((
            Question {
                qname,
                qtype,
                qclass,
            },
            offset,
        ))
    }
}

impl MessageHeader {
    fn new() -> MessageHeader {
        MessageHeader {
            id: MessageHeader::generate_id(),
            qr: 0,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 1,
            ra: 0,
            z: 0,
            rcode: ResponseCode::NOERROR,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    fn generate_id() -> u16 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.to_be_bytes());
        let mut flag_bytes = [0u8; 2];
        flag_bytes[0] =
            (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd;
        flag_bytes[1] = (self.ra << 7) | (self.z << 4) | self.rcode.to_u8();
        bytes.extend_from_slice(&flag_bytes);
        bytes.extend_from_slice(&self.qdcount.to_be_bytes());
        bytes.extend_from_slice(&self.ancount.to_be_bytes());
        bytes.extend_from_slice(&self.nscount.to_be_bytes());
        bytes.extend_from_slice(&self.arcount.to_be_bytes());
        bytes
    }

    fn deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 12 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Message header is too short",
            ));
        }

        let rcode_value = data[3] & 0x0F; // Extract the lower 4 bits for RCODE
        let rcode = ResponseCode::from_u8(rcode_value).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid RCODE value",
        ))?;

        Ok(MessageHeader {
            id: u16::from_be_bytes([data[0], data[1]]),
            qr: (data[2] >> 7) & 0x1,
            opcode: (data[2] >> 3) & 0xF,
            aa: (data[2] >> 2) & 0x1,
            tc: (data[2] >> 1) & 0x1,
            rd: data[2] & 0x1,
            ra: (data[3] >> 7) & 0x1,
            z: (data[3] >> 4) & 0x7,
            rcode,
            qdcount: u16::from_be_bytes([data[4], data[5]]),
            ancount: u16::from_be_bytes([data[6], data[7]]),
            nscount: u16::from_be_bytes([data[8], data[9]]),
            arcount: u16::from_be_bytes([data[10], data[11]]),
        })
    }
}

impl ResourceRecord {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Serialize the NAME (qname in your structure)
        for label in &self.name {
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label.as_bytes());
        }
        bytes.push(0); // End of name

        // Serialize TYPE, CLASS, TTL, RDLENGTH
        bytes.extend_from_slice(&self.rtype.to_u16().to_be_bytes());
        bytes.extend_from_slice(&self.rclass.to_be_bytes());
        bytes.extend_from_slice(&self.ttl.to_be_bytes());
        // For RDLENGTH and RDATA, ensure you have the correct length and data
        // Here we assume rdata is already the correct length and rdlength is set correctly
        bytes.extend_from_slice(&self.rdlength.to_be_bytes());
        bytes.extend_from_slice(&self.rdata.as_slice());

        bytes
    }

    fn deserialize(data: &[u8], start_offset: usize) -> Result<(Self, usize), std::io::Error> {
        let mut offset = start_offset;
        
        // Deserialize NAME
        let (name, new_offset) = Message::parse_qname(data, offset)?;
        offset = new_offset;
        
        // Deserialize TYPE, CLASS, TTL, RDLENGTH
        if data.len() < offset + 10 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Insufficient data for RR"));
        }
        let rtype = RecordType::from_u16(u16::from_be_bytes([data[offset], data[offset + 1]])).ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid RTYPE"))?;
        let rclass = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
        let ttl = u32::from_be_bytes([data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]]);
        let rdlength = u16::from_be_bytes([data[offset + 8], data[offset + 9]]);
        offset += 10;
        
        // Ensure there's enough data for RDATA
        if data.len() < offset + rdlength as usize {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Insufficient data for RDATA"));
        }
        
        // Deserialize RDATA
        let rdata = data[offset..(offset + rdlength as usize)].to_vec();
        offset += rdlength as usize;
        
        Ok((ResourceRecord {
            name,
            rtype,
            rclass,
            ttl,
            rdlength,
            rdata,
        }, offset))
    }
}

impl RecordType {
    fn to_u16(&self) -> u16 {
        *self as u16
    }

    fn from_u16(value: u16) -> Option<RecordType> {
        match value {
            1 => Some(RecordType::A),
            28 => Some(RecordType::AAAA),
            5 => Some(RecordType::CNAME),
            15 => Some(RecordType::MX),
            2 => Some(RecordType::NS),
            12 => Some(RecordType::PTR),
            6 => Some(RecordType::SOA),
            33 => Some(RecordType::SRV),
            16 => Some(RecordType::TXT),
            _ => None,
        }
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordType::A => write!(f, "A"),
            RecordType::AAAA => write!(f, "AAAA"),
            RecordType::CNAME => write!(f, "CNAME"),
            RecordType::MX => write!(f, "MX"),
            RecordType::NS => write!(f, "NS"),
            RecordType::PTR => write!(f, "PTR"),
            RecordType::SOA => write!(f, "SOA"),
            RecordType::SRV => write!(f, "SRV"),
            RecordType::TXT => write!(f, "TXT"),
        }
    }
}

impl ResponseCode {
    fn to_u8(&self) -> u8 {
        *self as u8
    }

    fn from_u8(value: u8) -> Option<ResponseCode> {
        match value {
            0 => Some(ResponseCode::NOERROR),
            1 => Some(ResponseCode::FORMERR),
            2 => Some(ResponseCode::SERVFAIL),
            3 => Some(ResponseCode::NXDOMAIN),
            4 => Some(ResponseCode::NOTIMP),
            5 => Some(ResponseCode::REFUSED),
            6 => Some(ResponseCode::YXDOMAIN),
            7 => Some(ResponseCode::XRRSET),
            9 => Some(ResponseCode::NOTAUTH),
            10 => Some(ResponseCode::NOTZONE),
            _ => None,
        }
    }
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResponseCode::NOERROR => write!(f, "NOERROR"),
            ResponseCode::FORMERR => write!(f, "FORMERR"),
            ResponseCode::SERVFAIL => write!(f, "SERVFAIL"),
            ResponseCode::NXDOMAIN => write!(f, "NXDOMAIN"),
            ResponseCode::NOTIMP => write!(f, "NOTIMP"),
            ResponseCode::REFUSED => write!(f, "REFUSED"),
            ResponseCode::YXDOMAIN => write!(f, "YXDOMAIN"),
            ResponseCode::XRRSET => write!(f, "XRRSET"),
            ResponseCode::NOTAUTH => write!(f, "NOTAUTH"),
            ResponseCode::NOTZONE => write!(f, "NOTZONE"),
        }
    }
}