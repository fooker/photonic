use std::collections::HashMap;
use std::path::PathBuf;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use photonic::attr::Range;
use serde::{Deserialize, Serialize};
use tokio_stream::{StreamExt, StreamMap};

use photonic::color::palette::rgb::Rgb;
use photonic::input::{AnyInputValue, InputSink};
use photonic::interface::{Interface, Introspection};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum InputValue {
    Boolean(bool),
    Integer(i64),
    Decimal(f32),
    Color(Rgb),
    IntegerRange(i64, i64),
    DecimalRange(f32, f32),
    ColorRange(Rgb, Rgb),
}

pub struct Restore {
    pub path: PathBuf,

    pub write_threshold: usize,
    pub write_timeout: Duration,
}

impl Restore {}

impl Interface for Restore {
    async fn listen(self, introspection: Arc<Introspection>) -> Result<()> {
        // Read existing restore data, if possible
        let mut data = if let Ok(data) = tokio::fs::read(&self.path).await {
            serde_json::from_slice(&data).with_context(|| format!("Invalid restore data: {}", self.path.display()))?
        } else {
            HashMap::new()
        };

        // Try to find and restore all inputs from data
        for (name, value) in data.iter() {
            let input = if let Some(input) = introspection.inputs.get(name) {
                input
            } else {
                continue;
            };
            let result = match (&input.sink(), value) {
                (InputSink::Boolean(sink), InputValue::Boolean(value)) => sink.send(*value).await,
                (InputSink::Integer(sink), InputValue::Integer(value)) => sink.send(*value).await,
                (InputSink::Decimal(sink), InputValue::Decimal(value)) => sink.send(*value).await,
                (InputSink::Color(sink), InputValue::Color(value)) => sink.send(*value).await,
                (InputSink::IntegerRange(sink), InputValue::IntegerRange(a, b)) => sink.send(Range::new(*a, *b)).await,
                (InputSink::DecimalRange(sink), InputValue::DecimalRange(a, b)) => sink.send(Range::new(*a, *b)).await,
                (InputSink::ColorRange(sink), InputValue::ColorRange(a, b)) => sink.send(Range::new(*a, *b)).await,
                (_, _) => Err(anyhow!("Restore data type mismatch: {} - ignoring", name)),
            };

            if let Err(err) = result {
                eprintln!("Failed to restore input value: {err}");
            }
        }

        // Merge all inputs into a stream of (name, value)
        let inputs = introspection
            .inputs
            .iter()
            .map(|(name, input)| (name.clone(), input.subscribe()))
            .collect::<StreamMap<_, _>>();

        // Form chunks by size and timeout
        let mut inputs = pin!(inputs.chunks_timeout(self.write_threshold, self.write_timeout));

        loop {
            let values = inputs.next().await.expect("Inputs never close");

            // Persist the values in the aggregated view
            for (name, value) in values {
                let value = match value {
                    AnyInputValue::Trigger => continue, // Skip triggers
                    AnyInputValue::Boolean(value) => InputValue::Boolean(value),
                    AnyInputValue::Integer(value) => InputValue::Integer(value),
                    AnyInputValue::Decimal(value) => InputValue::Decimal(value),
                    AnyInputValue::Color(value) => InputValue::Color(value),
                    AnyInputValue::IntegerRange(value) => InputValue::IntegerRange(value.0, value.1),
                    AnyInputValue::DecimalRange(value) => InputValue::DecimalRange(value.0, value.1),
                    AnyInputValue::ColorRange(value) => InputValue::ColorRange(value.0, value.1),
                };

                data.insert(name, value);
            }

            let data = serde_json::to_vec(&data).context("Failed to serialize restore data")?;

            tokio::fs::write(&self.path, data)
                .await
                .with_context(|| format!("Failed to persist restore data: {}", self.path.display()))?;
        }
    }
}
