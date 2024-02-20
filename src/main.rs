use clap::Parser;
use args::Args;
use client::Client;

mod display;
mod args;
mod client;

fn main() {
    let args = Args::parse();
    let client = Client::new(&args);
    let resp = client.query();

    display::Display::new(&resp).print_as_json();
}
