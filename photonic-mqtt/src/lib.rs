use std::collections::HashMap;
use std::str::FromStr;
use std::thread;

use anyhow::Error;
use rumqtt::{MqttClient, MqttOptions, Notification, QoS, ReconnectOptions};

use photonic_core::input::{Input, InputValue};

pub struct MqttHandleBuilder {
    id: String,
    host: String,
    port: u16,

    realm: Option<String>,

    endpoints: HashMap<String, Box<dyn FnMut(String) + Send>>,
}

impl MqttHandleBuilder {
    pub fn new<Id, Host>(id: Id, host: Host, port: u16) -> Self
        where Id: Into<String>,
              Host: Into<String> {
        return Self {
            id: id.into(),
            host: host.into(),
            port,
            realm: None,
            endpoints: HashMap::new(),
        };
    }

    pub fn with_realm<Realm>(mut self, realm: Realm) -> Self
        where Realm: Into<String> {
        self.realm = Some(realm.into());
        return self;
    }

    pub fn endpoint<T, F, Topic>(&mut self, topic: Topic, f: F) -> Input<T>
        where T: InputValue,
              F: Fn(String) -> Option<T> + Send + 'static,
              Topic: Into<String> {
        let input = Input::new();
        let mut sink = input.sink();

        let mut topic = topic.into();
        if let Some(ref realm) = self.realm {
            topic = format!("{}/{}", realm, topic);
        };

        self.endpoints.insert(topic, Box::new(move |msg| {
            if let Some(msg) = f(msg) {
                sink.send(msg);
            }
        }));

        return input;
    }

    pub fn trigger<Topic>(&mut self, topic: Topic) -> Input<()>
        where Topic: Into<String> {
        return self.endpoint(topic, move |_| { Some(()) });
    }

    pub fn value<T, Topic>(&mut self, topic: Topic) -> Input<T>
        where T: FromStr + InputValue,
              Topic: Into<String> {
        return self.endpoint(topic, move |s| {
            return T::from_str(&s).ok();
        });
    }

    pub fn start(self) -> Result<MqttHandle, Error> {
        let opts = MqttOptions::new(self.id, self.host, self.port)
            .set_clean_session(true)
            .set_reconnect_opts(ReconnectOptions::Always(5));

        let (mut client, notifications) = MqttClient::start(opts)
            .map_err(|err| Error::msg(err.to_string()))?; // TODO: Better error conversion

        for endpoint in self.endpoints.keys() {
            client.subscribe(endpoint.to_owned(), QoS::AtLeastOnce)
                .expect("Failed to subscribe");
        }

        let mut endpoints = self.endpoints;
        let worker = thread::spawn(move || {
            for notification in notifications {
                if let Notification::Publish(publish) = notification {
                    if let Some(endpoint) = endpoints.get_mut(&publish.topic_name) {
                        let message = String::from_utf8(publish.payload.to_vec()).unwrap(); // TODO: Error handling
                        endpoint(message);
                    }
                }
            }
        });

        return Ok(MqttHandle {
            client,
            worker,
        });
    }
}

pub struct MqttHandle {
    client: MqttClient,
    worker: thread::JoinHandle<()>,
}

impl Drop for MqttHandle {
    fn drop(&mut self) {
        // TODO: Close down the client...
    }
}
