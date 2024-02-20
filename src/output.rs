use crate::dns::Message as DNSMessage;
use tabled::{settings::Panel, settings::Style, Table, Tabled};

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
        let table = Table::new(vec![header.clone()])
            .with(Panel::header("Header"))
            .with(Style::extended())
            .to_string();
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
        let table = Table::new(answers)
            .with(Panel::header("Answers"))
            .with(Style::extended())
            .to_string();
        println!("{}", table);
    }

    fn print_authority(&self) {
        let authority = &self.message.authority;
        let table = Table::new(authority)
            .with(Panel::header("Authority"))
            .with(Style::extended())
            .to_string();
        println!("{}", table);
    }

    fn print_extra(&self) {
        let extra = &self.message.extra;
        let table = Table::new(extra)
            .with(Panel::header("Extra"))
            .with(Style::extended())
            .to_string();
        println!("{}", table);
    }
}
