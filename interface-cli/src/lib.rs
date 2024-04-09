use std::pin::Pin;
use std::sync::Arc;

use anyhow::Result;
use futures::{Stream, TryStreamExt};
use photonic::input::InputSink;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamMap;

use photonic::interface::Introspection;

pub mod stdio;
pub mod telnet;

pub(self) async fn run(
    i: impl AsyncRead + Unpin,
    o: impl AsyncWrite + Unpin,
    introspection: Arc<Introspection>,
) -> Result<()> {
    let i = BufReader::new(i);
    let mut o = BufWriter::new(o);

    let inputs = introspection
        .inputs
        .iter()
        .map(|(name, info)| {
            (name.clone(), match &info.sink {
                InputSink::Trigger(sink) => {
                    Box::pin(BroadcastStream::new(sink.subscribe()).map_ok(|()| "()".to_string()))
                        as Pin<Box<dyn Stream<Item = Result<String, BroadcastStreamRecvError>> + Send>>
                }
                InputSink::Boolean(sink) => {
                    Box::pin(BroadcastStream::new(sink.subscribe()).map_ok(|value| value.to_string()))
                }
                InputSink::Integer(sink) => {
                    Box::pin(BroadcastStream::new(sink.subscribe()).map_ok(|value| value.to_string()))
                }
                InputSink::Decimal(sink) => {
                    Box::pin(BroadcastStream::new(sink.subscribe()).map_ok(|value| value.to_string()))
                }
                InputSink::Color(sink) => Box::pin(
                    BroadcastStream::new(sink.subscribe()).map_ok(|value| format!("{:02x}", value.into_format::<u8>())),
                ),
                InputSink::IntegerRange(sink) => {
                    Box::pin(BroadcastStream::new(sink.subscribe()).map_ok(|value| value.to_string()))
                }
                InputSink::DecimalRange(sink) => {
                    Box::pin(BroadcastStream::new(sink.subscribe()).map_ok(|value| value.to_string()))
                }
                InputSink::ColorRange(sink) => Box::pin(BroadcastStream::new(sink.subscribe()).map_ok(|value| {
                    value.map(|component| format!("{:02x}", component.into_format::<u8>())).to_string()
                })),
            })
        })
        //.map(|(name, stream)| stream.map_ok(|value| format!("↯ {}: {}", name, value)))
        .collect::<StreamMap<String, _>>();

    let mut lines = i.lines();
    loop {
        o.write_all("✨ » ".as_bytes()).await?;
        o.flush().await?;

        let line = match lines.next_line().await? {
            Some(line) => line,
            None => {
                break;
            }
        };

        let line = match shlex::split(&line) {
            Some(line) => line,
            None => {
                o.write_all("Invalid input\n".as_bytes()).await?;
                continue;
            }
        };

        match line.get(0).map(String::as_str) {
            Some("exit") => {
                break;
            }

            Some("node") => {
                if let Some(node) = line.get(1) {
                    if let Some(node) = introspection.nodes.get(node) {
                        o.write_all(format!("Node '{}':\n", node.name).as_bytes()).await?;
                        o.write_all(format!("  Kind: {}\n", node.kind).as_bytes()).await?;
                        o.write_all(format!("  Nodes: {}\n", node.kind).as_bytes()).await?;
                        for (name, info) in node.nodes.iter() {
                            o.write_all(format!("    {} = [{}]\n", name, info.kind).as_bytes()).await?;
                        }
                        o.write_all(format!("  Attributes: {}\n", node.kind).as_bytes()).await?;
                        for (name, info) in node.attrs.iter() {
                            o.write_all(format!("    {} : {} = [{}]\n", name, info.value_type, info.kind).as_bytes())
                                .await?;
                            // TODO: Recurse into attrs
                            // TODO: Show attached inputs
                        }
                    } else {
                        o.write_all(format!("No such node: '{}'\n", node).as_bytes()).await?;
                    }
                } else {
                    for (name, info) in introspection.nodes.iter() {
                        o.write_all(format!("{} = [{}]\n", name, info.kind).as_bytes()).await?;
                    }
                }
            }

            Some("input") => {
                if let Some(input) = line.get(1) {
                    if let Some(input) = introspection.inputs.get(input) {
                        if let Some(value) = line.get(2) {
                            match input.sink.send_str(&value) {
                                Ok(()) => {}
                                Err(err) => {
                                    o.write_all(
                                        format!("Invalid value: '{}' for {}: {}", value, input.sink, err).as_bytes(),
                                    )
                                    .await?;
                                    continue;
                                }
                            }
                        } else {
                            o.write_all(format!("Input '{}':\n", input.name).as_bytes()).await?;
                            o.write_all(format!("  Value: {}\n", input.value_type).as_bytes()).await?;
                        }
                    } else {
                        o.write_all(format!("No such input: '{}'\n", input).as_bytes()).await?;
                    }
                } else {
                    for (name, info) in introspection.inputs.iter() {
                        o.write_all(format!("{} : {}\n", name, info.value_type).as_bytes()).await?;
                    }
                }
            }

            Some(unknown) => {
                o.write_all(format!("Unknown command: '{}'\n", unknown).as_bytes()).await?;
                continue;
            }
            None => {
                continue;
            }
        }
    }

    return Ok(());
}
