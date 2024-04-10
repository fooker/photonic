use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use palette::rgb::Rgb;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};

use photonic::attr::Range;
use photonic::input::InputValueParser;
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
                            match input.sink.send_from_str::<CLIInputValueParser>(&value) {
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

struct CLIInputValueParser;

impl CLIInputValueParser {
    fn parse_range<V, E>(s: &str, parse: impl Fn(&str) -> Result<V, E>) -> Result<Range<V>>
    where
        V: Clone,
        E: Into<anyhow::Error>,
    {
        return Ok(if let Some((a, b)) = s.split_once("..") {
            Range::new(parse(a).map_err(Into::into)?, parse(b).map_err(Into::into)?)
        } else {
            Range::point(parse(s).map_err(Into::into)?)
        });
    }
}

impl InputValueParser for CLIInputValueParser {
    fn parse_trigger(_: &str) -> Result<()> {
        return Ok(());
    }

    fn parse_boolean(s: &str) -> Result<bool> {
        return Ok(s.parse()?);
    }

    fn parse_integer(s: &str) -> Result<i64> {
        return Ok(s.parse()?);
    }

    fn parse_decimal(s: &str) -> Result<f32> {
        return Ok(s.parse()?);
    }

    fn parse_color(s: &str) -> Result<Rgb> {
        return Ok(s.parse::<Rgb<_, u8>>()?.into_format());
    }

    fn parse_integer_range(s: &str) -> Result<Range<i64>> {
        return Self::parse_range(s, FromStr::from_str);
    }

    fn parse_decimal_range(s: &str) -> Result<Range<f32>> {
        return Self::parse_range(s, FromStr::from_str);
    }

    fn parse_color_range(s: &str) -> Result<Range<Rgb>> {
        return Self::parse_range(s, Self::parse_color);
    }
}
