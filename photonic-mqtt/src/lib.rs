#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use mqtt_async_client::client::{Client, KeepAlive, QoS, Subscribe, SubscribeTopic};

use photonic_core::input::InputSender;
use photonic_core::interface::Interface;
use photonic_core::Introspection;

pub struct MqttInterface {
    client: Client,
    realm: String,
}

impl MqttInterface {
    pub fn connect(host: String, port: u16, realm: String) -> Result<Self, Error> {
        let client = Client::builder()
            .set_automatic_connect(true)
            .set_host(host)
            .set_port(port)
            .set_keep_alive(KeepAlive::from_secs(1))
            .build()?;

        return Ok(Self { client, realm });
    }
}

#[async_trait]
impl Interface for MqttInterface {
    async fn listen(mut self, introspection: Arc<Introspection>) -> Result<(), Error> {
        self.client.connect().await?;

        let topics = introspection
            .inputs
            .iter()
            .map(|(name, input)| (format!("{}/{}/set", self.realm, name), input.clone()))
            .collect::<HashMap<_, _>>();

        self.client
            .subscribe(Subscribe::new(
                topics
                    .keys()
                    .map(|topic| SubscribeTopic {
                        topic_path: topic.clone(),
                        qos: QoS::AtLeastOnce,
                    })
                    .collect(),
            ))
            .await?
            .any_failures()?;

        loop {
            let read = self.client.read_subscriptions().await?;

            if let Some(input) = topics.get(read.topic()) {
                match &input.sender {
                    InputSender::Trigger(sink) => sink.send(()),

                    InputSender::Boolean(sink) => {
                        if let Ok(val) = serde_json::from_slice(read.payload()) {
                            sink.send(val);
                        }
                    }

                    InputSender::Integer(sink) => {
                        if let Ok(val) = serde_json::from_slice(read.payload()) {
                            sink.send(val);
                        }
                    }

                    InputSender::Decimal(sink) => {
                        if let Ok(val) = serde_json::from_slice(read.payload()) {
                            sink.send(val);
                        }
                    }
                }
            }
        }
    }
}
