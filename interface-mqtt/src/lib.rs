use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use photonic::interface::{Interface, Introspection};

struct Realm<'a>(&'a str);

impl<'a> Realm<'a> {
    pub fn from(value: &'a str) -> Self {
        return Self(if value.ends_with('/') { &value[0..value.len() - 1] } else { value });
    }

    pub fn topic(&self, suffix: &str) -> String {
        return format!("{}/{}", self.0, suffix);
    }
}

pub struct MQTT<'s> {
    pub mqtt_options: MqttOptions,

    pub realm: &'s str,
}

impl <'s> MQTT<'s> {
    pub fn new(url: impl Into<String>) -> Result<Self> {
        let mqtt_options = MqttOptions::parse_url(url)?;
        return Ok(Self {
            mqtt_options,
            realm: "photonic", // TODO: Extract realm from URL
        });
    }

    pub fn with_realm(mut self, realm: &'s str) -> Self {
        self.realm = realm;
        return self;
    }
}

impl Interface for MQTT<'_> {
    async fn listen(self, introspection: Arc<Introspection>) -> Result<()> {
        let realm = Realm::from(&self.realm);

        let (client, mut event_loop) = AsyncClient::new(self.mqtt_options, 10);

        let mut topics = HashMap::new();

        for input in introspection.inputs.values() {
            let topic = realm.topic(&input.name);
            client.subscribe(&topic, QoS::AtLeastOnce).await?;

            eprintln!("⇄ Subscribed to '{}' for input '{}' with type {}", topic, input.name, input.value_type);

            topics.insert(topic, input);
        }

        while let Ok(event) = event_loop.poll().await {
            if let Event::Incoming(Incoming::Publish(publish)) = event {
                let input = topics.get(&publish.topic).expect("Got notification for unknown topic");

                let payload = match String::from_utf8(publish.payload.to_vec()) {
                    Ok(payload) => payload,
                    Err(err) => {
                        eprintln!("⇄ Invalid value on '{}' = {:?}: {}", publish.topic, publish.payload, err);
                        continue;
                    }
                };

                match input.sink.send_str(&payload) {
                    Ok(()) => {}
                    Err(err) => {
                        eprintln!("⇄ Invalid value on '{}' = {:?}: {}", publish.topic, payload, err);
                        continue;
                    }
                }
            }
        }

        return Ok(());
    }
}
