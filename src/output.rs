use crate::dns::Message as DNSMessage;

pub struct Output { 
    message: DNSMessage
}

impl Output {
    pub fn new(msg: DNSMessage) -> Output {
        Output {
            message: msg
        }
    }
    pub fn print(&self) {
        println!("Header: {:?}", self.message.header);
        println!("Questions: {:?}", self.message.question);
        println!("Answers: {:?}", self.message.answer);
        println!("Authority: {:?}", self.message.authority);
        println!("Extra: {:?}", self.message.extra);
    }
}
