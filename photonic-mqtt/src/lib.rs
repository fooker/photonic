use std::collections::HashMap;
use std::str::FromStr;
use std::thread;

use failure::Error;
use rumqtt::{MqttClient, MqttOptions, Notification, ReconnectOptions};

use photonic::input::Input;

pub struct MqttHandleBuilder {
    id: String,
    host: String,
    port: u16,

    realm: Option<String>,

    endpoints: HashMap<String, Box<FnMut(String) + Send>>,
}

impl MqttHandleBuilder {
    pub fn new(id: String, host: String, port: u16) -> Self {
        return Self {
            id,
            host,
            port,
            realm: None,
            endpoints: HashMap::new(),
        };
    }

    pub fn with_realm(mut self, realm: String) -> Self {
        self.realm = Some(realm);
        return self;
    }

    pub fn endpoint<T, F>(&mut self, topic: String, f: F) -> Input<T>
        where T: Send + 'static,
              F: Fn(String) -> Option<T> + Send + 'static {
        let (input, mut sink) = Input::new();

        let mut topic = topic;
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

    pub fn trigger(&mut self, topic: String) -> Input<()> {
        return self.endpoint(topic, move |_| { Some(()) });
    }

    pub fn value<T>(&mut self, topic: String) -> Input<T>
        where T: FromStr + Send + 'static {
        return self.endpoint(topic, move |s| {
            return T::from_str(&s).ok();
        });
    }

    pub fn start(self) -> Result<MqttHandle, Error> {
        let opts = MqttOptions::new(self.id, self.host, self.port)
            .set_clean_session(true)
            .set_reconnect_opts(ReconnectOptions::Always(5));

        let (client, notifications) = MqttClient::start(opts)?;

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
