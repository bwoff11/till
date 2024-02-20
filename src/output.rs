use crate::dns::Message as DNSMessage;
use tabled::{
    settings::{
        object::{Rows, Segment},
        style::{HorizontalLine, Style},
        Alignment, Modify, Padding, Width, Panel
    },
    Table,
};

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
        //self.print_authority();
        //self.print_extra();
    }

    fn print_header(&self) {
        let header = &self.message.header;
        let table = Table::new(vec![header.clone()])
            .with(Panel::vertical(0, "Header"))
            .with(Style::modern().remove_horizontal())
            .to_string();
        println!("{}", table);
    }

    fn print_answers(&self) {
        let answers = &self.message.answer;
        let table = Table::new(answers)
            .with(Panel::vertical(0, "Answers"))
            .with(Style::modern().remove_horizontal())
            .to_string();
        println!("{}", table);
    }
}
