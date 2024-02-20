use hickory_client::op::DnsResponse;
use hickory_client::rr::Record;

pub struct Display<'a> {
    answers: &'a [Record],
}

impl<'a> Display<'a> {
    pub fn new(response: &'a DnsResponse) -> Self {
        Display {
            answers: response.answers()
        }
    }

    pub fn print(&self) {
        for answer in self.answers {
            println!("{:?}", answer);
        }
    }
}