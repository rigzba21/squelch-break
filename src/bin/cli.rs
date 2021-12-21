extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("sb")
                          .version("0.0.1")
                          .author("Jon V <rigzba21>")
                          .about("CLI for sending formatted CBOR messages")
                          .subcommand(SubCommand::with_name("send")
                                      .about("sends a CBOR data formatted message"))
                                      .arg(Arg::with_name("message")
                                          .short("m")
                                          .required(true)
                                          .takes_value(true)
                                          .help("message to send CBOR formatted"))
                          .get_matches();


    if let Some(_matches) = matches.subcommand_matches("send") {
        let message: String = matches.value_of("message").unwrap().parse().unwrap();

        println!("Sending Message: {}", message)
    }

}