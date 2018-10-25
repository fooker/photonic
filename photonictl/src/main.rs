extern crate clap;
extern crate photonic_proto as proto;
extern crate ws;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("Photonic")
            .author("Dustin Frisch <fooker@lab.sh>")
            .about("Shines bright like a diamond")
            .arg(Arg::with_name("remote")
                    .short("r")
                    .long("remote")
                    .value_name("URL")
                    .takes_value(true)
                    .default_value("ws://localhost:1337/")
                    .help("URL of the photonicd instance to control"))
            .subcommand(SubCommand::with_name("fader")
                    .about("Sets a fader to a new value")
                    .arg(Arg::with_name("name")
                            .index(1)
                            .required(true)
                            .help("name of the fader to change"))
                    .arg(Arg::with_name("value")
                            .index(2)
                            .required(true)
                            .help("the value to set")))
            .subcommand(SubCommand::with_name("button")
                    .about("Triggers a button")
                    .arg(Arg::with_name("name")
                            .index(0)
                            .required(true)
                            .help("name of the button to trigger")))
            .get_matches();

    let remote = matches.value_of("remote").unwrap();
    println!("{:?}", remote);

    match matches.subcommand() {
        ("fader", Some(matches)) => fader(remote, matches),
        ("button", Some(matches)) => button(remote, matches),
        (_, _) => {
            // FIXME: ??
        }
    }
}

struct CommandHandler<'c> {
    out: ws::Sender,
    command: &'c proto::Command,
}

impl<'c> ws::Handler for CommandHandler<'c> {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        let data = proto::Command::encode(&self.command).map_err(Box::new)?;
        self.out.send(data)?;
        self.out.close(ws::CloseCode::Normal)?;

        return Ok(());
    }
}

fn send_command(remote: &str,
                command: &proto::Command) {
    ws::connect(remote, |out| {
        return CommandHandler { out, command };
    }).unwrap();
}

fn fader(remote: &str,
         matches: &ArgMatches) {
    // FIXME: Error handling
    let name = matches.value_of("name").unwrap().to_owned();
    let value = matches.value_of("value").unwrap().parse::<f64>().unwrap();

    send_command(remote,
                 &proto::Command::ChangeFader {
                     name,
                     value,
                 });
}

fn button(remote: &str,
          matches: &ArgMatches) {
    // FIXME: Error handling
    let name = matches.value_of("name").unwrap().to_owned();

    send_command(remote,
                 &proto::Command::TriggerButton {
                     name,
                 });
}
