use std::sync::Arc;

use anyhow::Result;
use palette::Srgb;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use photonic::attr::Range;

use photonic::input::InputSink;
use photonic::interface::Introspection;

pub mod stdio;
pub mod telnet;

pub(self) async fn run(i: impl AsyncRead + Unpin, o: impl AsyncWrite + Unpin, introspection: Arc<Introspection>) -> Result<()> {
    let i = BufReader::new(i);
    let mut o = BufWriter::new(o);


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
                            o.write_all(format!("    {} : {} = [{}]\n", name, info.value_type, info.kind).as_bytes()).await?;
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
                            match &input.sink {
                                InputSink::Trigger(sink) => {
                                    if "trigger" == value {
                                        sink.send(());
                                    } else {
                                        o.write_all(format!("Invalid trigger value: '{}'", value).as_bytes()).await?;
                                    }
                                }
                                InputSink::Boolean(sink) => {
                                    if let Ok(value) = value.parse::<bool>() {
                                        sink.send(value);
                                    } else {
                                        o.write_all(format!("Invalid boolean value: '{}'", value).as_bytes()).await?;
                                    }
                                }
                                InputSink::Integer(sink) => {
                                    if let Ok(value) = value.parse::<i64>() {
                                        sink.send(value);
                                    } else {
                                        o.write_all(format!("Invalid integer value: '{}'", value).as_bytes()).await?;
                                    }
                                }
                                InputSink::Decimal(sink) => {
                                    if let Ok(value) = value.parse::<f64>() {
                                        sink.send(value);
                                    } else {
                                        o.write_all(format!("Invalid decimal value: '{}'", value).as_bytes()).await?;
                                    }
                                }
                                InputSink::Color(sink) => {
                                    if let Ok(value) = value.parse::<Srgb<u8>>() {
                                        sink.send(value.into_format());
                                    } else {
                                        o.write_all(format!("Invalid color value: '{}'", value).as_bytes()).await?;
                                    }
                                }
                                InputSink::IntegerRange(sink) => {
                                    if let Some((v1, v2)) = value.split_once("..") {
                                        if let (Ok(v1), Ok(v2)) = (v1.parse::<i64>(), v2.parse::<i64>()) {
                                            sink.send(Range(v1, v2));
                                        } else {
                                            o.write_all(format!("Invalid integer range value: '{}'", value).as_bytes()).await?;
                                        }
                                    } else {
                                        o.write_all(format!("Invalid integer range value: '{}'", value).as_bytes()).await?;
                                    }
                                }
                                InputSink::DecimalRange(sink) => {
                                    if let Some((v1, v2)) = value.split_once("..") {
                                        if let (Ok(v1), Ok(v2)) = (v1.parse::<f64>(), v2.parse::<f64>()) {
                                            sink.send(Range(v1, v2));
                                        } else {
                                            o.write_all(format!("Invalid decimal range value: '{}'", value).as_bytes()).await?;
                                        }
                                    } else {
                                        o.write_all(format!("Invalid integer range value: '{}'", value).as_bytes()).await?;
                                    }
                                }
                                InputSink::ColorRange(sink) => {
                                    if let Some((v1, v2)) = value.split_once("..") {
                                        if let (Ok(v1), Ok(v2)) = (v1.parse::<Srgb<u8>>(), v2.parse::<Srgb<u8>>()) {
                                            sink.send(Range(v1.into_format(), v2.into_format()));
                                        } else {
                                            o.write_all(format!("Invalid color range value: '{}'", value).as_bytes()).await?;
                                        }
                                    } else {
                                        o.write_all(format!("Invalid color range value: '{}'", value).as_bytes()).await?;
                                    }
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



