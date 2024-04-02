use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use bytes::Bytes;
use rumqttc::{AsyncClient, Event, Incoming, LastWill, MqttOptions, QoS};

use photonic::interface::{Interface, Introspection};

struct Realm<'a>(&'a str);

impl<'a> Realm<'a> {
    pub fn from(value: &'a str) -> Self {
        return Self(if value.ends_with('/') { &value[0..value.len() - 1] } else { value });
    }

    pub fn topic(&self, suffix: impl AsRef<str>) -> String {
        return format!("{}/{}", self.0, suffix.as_ref());
    }
}

pub struct MQTT<'s> {
    pub mqtt_options: MqttOptions,

    pub realm: &'s str,
}

impl <'s> MQTT<'s> {
    pub fn with_url(url: impl Into<String>) -> Result<Self> {
        let mut mqtt_options = MqttOptions::parse_url(url)?;
        mqtt_options.set_keep_alive(Duration::from_secs(5));
        mqtt_options.set_clean_session(true);

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
    async fn listen(mut self, introspection: Arc<Introspection>) -> Result<()> {
        let realm = Realm::from(&self.realm);

        self.mqtt_options.set_last_will(LastWill {
            topic: realm.topic("status"),
            message: Bytes::from("offline"),
            qos: QoS::AtLeastOnce,
            retain: true,
        });

        let (client, mut event_loop) = AsyncClient::new(self.mqtt_options, 10);

        client.publish_bytes(realm.topic("status"),
                             QoS::AtLeastOnce,
                             true,
                             Bytes::from("online")).await?;

        let mut topics = HashMap::new();

        for input in introspection.inputs.values() {
            let topic = realm.topic(format!("input/{}/set", input.name));
            
            client.subscribe(topic.clone(), QoS::AtLeastOnce).await?;
            eprintln!("⇄ Subscribed to '{}' for input '{}' with type {}", topic, input.name, input.value_type);
            
            topics.insert(topic, input);
        }

        while let Ok(event) = event_loop.poll().await {
            match event {
                Event::Incoming(Incoming::Connect(_)) => {
                    for (topic, input) in &topics {
                        client.subscribe(topic.clone(), QoS::AtLeastOnce).await?;
                        eprintln!("⇄ Subscribed to '{}' for input '{}' with type {}", topic, input.name, input.value_type);
                    }
                }

                Event::Incoming(Incoming::Publish(publish)) => {
                    let input = match topics.get(&publish.topic) {
                        Some(input) => input,
                        None => {
                            eprintln!("Got notification for unknown topic: {}", publish.topic);
                            continue;
                        }
                    };

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

                _ => {}
            }
        }

        return Ok(());
    }
}
