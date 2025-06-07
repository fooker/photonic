use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use bytes::Bytes;
use futures::StreamExt;
use palette::rgb::Rgb;
use photonic::attr::Range;
use photonic::input::{AnyInputValue, InputSink, Trigger};
use rumqttc::{AsyncClient, Event, Incoming, LastWill, MqttOptions, QoS};
use tokio_stream::StreamMap;

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

pub struct MQTT {
    pub mqtt_options: MqttOptions,

    pub realm: String,
}

impl MQTT {
    pub fn with_url(url: impl Into<String>) -> Result<Self> {
        let mut mqtt_options = MqttOptions::parse_url(url)?;
        mqtt_options.set_keep_alive(Duration::from_secs(5));
        mqtt_options.set_clean_session(true);

        return Ok(Self {
            mqtt_options,
            realm: "photonic".into(), // TODO: Extract realm from URL
        });
    }

    pub fn with_realm(mut self, realm: impl Into<String>) -> Self {
        self.realm = realm.into();
        return self;
    }
}

impl Interface for MQTT {
    async fn listen(mut self, introspection: Arc<Introspection>) -> Result<()> {
        let realm = Realm::from(&self.realm);

        self.mqtt_options.set_last_will(LastWill {
            topic: realm.topic("status"),
            message: Bytes::from("offline"),
            qos: QoS::AtLeastOnce,
            retain: true,
        });

        self.mqtt_options.set_keep_alive(Duration::from_secs(5));

        let (client, mut event_loop) = AsyncClient::new(self.mqtt_options.clone(), 10);

        let mut inputs = introspection
            .inputs
            .iter()
            .map(|(name, input)| (realm.topic(format!("input/{name}")), input.subscribe()))
            .collect::<StreamMap<_, _>>();

        loop {
            tokio::select! {
                Some((topic, value)) = inputs.next() => {
                    let value = match value {
                        AnyInputValue::Trigger => String::new(),
                        AnyInputValue::Boolean(value) => value.to_string(),
                        AnyInputValue::Integer(value) => value.to_string(),
                        AnyInputValue::Decimal(value) => value.to_string(),
                        AnyInputValue::Color(value) => format!("#{:06x}", value.into_format::<u8>()),
                        AnyInputValue::IntegerRange(value) => value.to_string(),
                        AnyInputValue::DecimalRange(value) => value.to_string(),
                        AnyInputValue::ColorRange(value) => value.map(|value| format!("#{:06x}", value.into_format::<u8>())).to_string(),
                    };
                    client.publish(topic, QoS::AtLeastOnce, false, value).await?;
                }

                event = event_loop.poll() => match event {
                    Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                        // Subscribe to all input topics
                        client.subscribe(realm.topic("input/+/set"), QoS::AtLeastOnce).await?;

                        // Report online status
                        client.publish_bytes(realm.topic("status"), QoS::AtLeastOnce, true, Bytes::from("online")).await?;
                    }

                    Ok(Event::Incoming(Incoming::Publish(publish))) => {
                        let input = introspection.inputs.iter()
                            .find_map(|(name, input)| (realm.topic(format!("input/{name}/set")) == publish.topic).then_some(input));

                        let input = match input {
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

                        let res: Result<()> = (async { match input.sink() {
                            InputSink::Trigger(sink) => sink.send(Trigger::next()).await,
                            InputSink::Boolean(sink) => sink.send(payload.parse()?).await,
                            InputSink::Integer(sink) => sink.send(payload.parse()?).await,
                            InputSink::Decimal(sink) => sink.send(payload.parse()?).await,
                            InputSink::Color(sink) => sink.send(payload.parse::<Rgb<_, u8>>()?.into_format()).await,
                            InputSink::IntegerRange(sink) => sink.send(payload.parse()?).await,
                            InputSink::DecimalRange(sink) => sink.send(payload.parse()?).await,
                            InputSink::ColorRange(sink) => sink.send(payload.parse::<Range<Rgb<_, u8>>>()?.map(Rgb::into_format)).await,
                        }}).await;

                        match res {
                            Ok(()) => {}
                            Err(err) => {
                                eprintln!("⇄ Invalid value on '{}' = {:?}: {}", publish.topic, payload, err);
                                continue;
                            }
                        }
                    }

                    Ok(_) => {}

                    Err(err) => {
                        eprintln!("MQTT error: {err}");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        }
    }
}
