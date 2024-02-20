use tabled::{Table, Tabled};
use crate::dns::Message as DNSMessage;

// Assuming DNSMessage and other component structs are defined elsewhere
pub struct Output {
    message: DNSMessage,
}

impl Output {
    pub fn new(msg: DNSMessage) -> Output {
        Output { message: msg }
    }

    pub fn print(&self) {
        self.print_header();
        //self.print_questions();
        self.print_answers();
        self.print_authority();
        self.print_extra();
    }

    fn print_header(&self) {
        let header = &self.message.header;
        let table = Table::new(vec![header.clone()]).to_string();
        println!("Header");
        println!("{}", table);
    }

    //fn print_questions(&self) {
    //    let questions = &self.message.question;
    //    let table = Table::new(questions.clone()).to_string();
    //    println!("Questions");
    //    println!("{}", table);
    //}

    fn print_answers(&self) {
        let answers = &self.message.answer;
        let table = Table::new(answers.clone()).to_string();
        println!("Answers");
        println!("{}", table);
    }

    fn print_authority(&self) {
        let authority = &self.message.authority;
        let table = Table::new(authority.clone()).to_string();
        println!("Authority");
        println!("{}", table);
    }

    fn print_extra(&self) {
        let additional = &self.message.extra;
        let table = Table::new(additional.clone()).to_string();
        println!("Additional");
        println!("{}", table);
    }
}
