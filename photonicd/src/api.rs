extern crate ws;

use photonic::attributes::{Attribute, DynamicValue};
use photonic::attributes::dynamic::{ButtonValue, FaderValue};
use photonic::core::Node;
use photonic::inspection;
use photonic_proto as proto;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::SyncSender;


pub struct Config<'c> {
    pub address: &'c str,
}

pub struct Server<'s> {
    sender: ws::Sender,
    faders: &'s HashMap<String, SyncSender<f64>>,
    buttons: &'s HashMap<String, SyncSender<()>>,
}

impl <'s> ws::Handler for Server<'s> {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let command = proto::Command::decode(&msg.into_data()).map_err(Box::new)?;

        match command {
            proto::Command::ChangeFader { name, value } => {
                if let Some(fader) = self.faders.get(&name) {
                    fader.send(value).unwrap();
                }
            }
            proto::Command::TriggerButton { name } => {
                if let Some(button) = self.buttons.get(&name) {
                    button.send(()).unwrap();
                }
            }
        }

        Ok(())
    }
}

pub fn serve(config: Config<'static>, root_node: &Node) {
    let mut faders = HashMap::new();
    let mut buttons = HashMap::new();

    inspection::visit_attributes(root_node, &mut |attr| {
        if let Attribute::Dynamic { ref name, ref value } = attr {
            match value {
                DynamicValue::Fader(ref fader) => {
                    faders.insert(name.to_owned(), fader.updater());
                }
                DynamicValue::Button(ref button) => {
                    buttons.insert(name.to_owned(), button.updater());
                }
                _ => {}
            }
        }
    });

    thread::spawn(move || {
        ws::listen(config.address, |out| {
            Server {
                sender: out,
                faders: &faders,
                buttons: &buttons,
            }
        });
    });
}

