use std::sync::Arc;

use anyhow::Result;
use palette::rgb::Rgb;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};

use photonic::attr::Range;
use photonic::input::{InputSink, Trigger};
use photonic::interface::Introspection;

pub mod stdio;
pub mod telnet;

async fn run(i: impl AsyncRead + Unpin, o: impl AsyncWrite + Unpin, introspection: Arc<Introspection>) -> Result<()> {
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

        match line.first().map(String::as_str) {
            Some("exit") => {
                break;
            }

            Some("node") => {
                if let Some(node) = line.get(1) {
                    if let Some(node) = introspection.nodes.get(node) {
                        o.write_all(format!("Node '{}':\n", node.name()).as_bytes()).await?;
                        o.write_all(format!("  Kind: {}\n", node.kind()).as_bytes()).await?;
                        o.write_all(format!("  Nodes: {}\n", node.kind()).as_bytes()).await?;
                        for (name, info) in node.nodes().iter() {
                            o.write_all(format!("    {} = [{}]\n", name, info.kind()).as_bytes()).await?;
                        }
                        o.write_all(format!("  Attributes: {}\n", node.kind()).as_bytes()).await?;
                        for (name, info) in node.attrs().iter() {
                            o.write_all(
                                format!("    {} : {} = [{}]\n", name, info.value_type(), info.kind()).as_bytes(),
                            )
                            .await?;
                            // TODO: Recurse into attrs
                            // TODO: Show attached inputs
                        }
                    } else {
                        o.write_all(format!("No such node: '{node}'\n").as_bytes()).await?;
                    }
                } else {
                    for (name, info) in introspection.nodes.iter() {
                        o.write_all(format!("{} = [{}]\n", name, info.kind()).as_bytes()).await?;
                    }
                }
            }

            Some("input") => {
                if let Some(input) = line.get(1) {
                    if let Some(input) = introspection.inputs.get(input) {
                        if let Some(value) = line.get(2) {
                            let res: Result<()> = (async {
                                match &input.sink() {
                                    InputSink::Trigger(sink) => sink.send(Trigger::next()).await,
                                    InputSink::Boolean(sink) => sink.send(value.parse()?).await,
                                    InputSink::Integer(sink) => sink.send(value.parse()?).await,
                                    InputSink::Decimal(sink) => sink.send(value.parse()?).await,
                                    InputSink::Color(sink) => {
                                        sink.send(value.parse::<Rgb<_, u8>>()?.into_format()).await
                                    }
                                    InputSink::IntegerRange(sink) => sink.send(value.parse()?).await,
                                    InputSink::DecimalRange(sink) => sink.send(value.parse()?).await,
                                    InputSink::ColorRange(sink) => {
                                        sink.send(value.parse::<Range<Rgb<_, u8>>>()?.map(Rgb::into_format)).await
                                    }
                                }
                            })
                            .await;

                            match res {
                                Ok(()) => {}
                                Err(err) => {
                                    o.write_all(
                                        format!("Invalid value: '{}' for {}: {}", value, input.sink(), err).as_bytes(),
                                    )
                                    .await?;
                                    continue;
                                }
                            }
                        } else {
                            o.write_all(format!("Input '{}':\n", input.name()).as_bytes()).await?;
                            o.write_all(format!("  Value: {}\n", input.value_type()).as_bytes()).await?;
                        }
                    } else {
                        o.write_all(format!("No such input: '{input}'\n").as_bytes()).await?;
                    }
                } else {
                    for (name, info) in introspection.inputs.iter() {
                        o.write_all(format!("{} : {}\n", name, info.value_type()).as_bytes()).await?;
                    }
                }
            }

            Some(unknown) => {
                o.write_all(format!("Unknown command: '{unknown}'\n").as_bytes()).await?;
                continue;
            }
            None => {
                continue;
            }
        }
    }

    return Ok(());
}
