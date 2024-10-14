use std::io::Write;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use anyhow::{Context, Result};
use palette::Srgb;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use photonic::{BufferReader, Output, OutputDecl};

pub struct Terminal {
    pub size: usize,
    pub path: Option<PathBuf>,
    pub waterfall: bool,
}

impl Terminal {
    pub fn new(size: usize) -> Self {
        return Self {
            size,
            path: None,
            waterfall: false,
        };
    }

    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        return self;
    }

    pub fn with_waterfall(mut self, waterfall: bool) -> Self {
        self.waterfall = waterfall;
        return self;
    }
}

pub struct TerminalOutput {
    size: usize,
    waterfall: bool,

    out: Pin<Box<dyn AsyncWrite>>,
}

impl OutputDecl for Terminal {
    const KIND: &'static str = "terminal";

    type Output = TerminalOutput;

    async fn materialize(self) -> Result<Self::Output>
    where Self::Output: Sized {
        let out: Pin<Box<dyn AsyncWrite>> = if let Some(path) = self.path {
            let _ = nix::unistd::unlink(&path);
            nix::unistd::mkfifo(&path, nix::sys::stat::Mode::S_IRWXU)
                .with_context(|| format!("Failed to create output fifo: '{}'", path.display()))?;

            Box::pin(
                tokio::net::unix::pipe::OpenOptions::new()
                    .read_write(true)
                    .open_sender(&path)
                    .with_context(|| format!("Failed to open output fifo: '{}'", path.display()))?,
            )
        } else {
            Box::pin(tokio::io::stdout())
        };

        return Ok(Self::Output {
            size: self.size,
            waterfall: self.waterfall,
            out,
        });
    }
}

impl Output for TerminalOutput {
    const KIND: &'static str = "terminal";

    type Element = Srgb;

    async fn render(&mut self, out: impl BufferReader<Element = Self::Element>) -> Result<()> {
        // TODO: Maybe with inline replacement?
        let mut buf = Vec::with_capacity(out.size() * 20 + 5);

        for rgb in out.iter() {
            let rgb = rgb.into_format::<u8>();
            let (r, g, b) = rgb.into_components();

            write!(&mut buf, "\x1b[48;2;{:03};{:03};{:03}m ", r, g, b)?;
        }

        write!(&mut buf, "\x1b[0m")?;
        write!(&mut buf, "{}", if self.waterfall { "\n" } else { "\r" })?;

        self.out.write_all(&buf).await?;
        self.out.flush().await?;

        return Ok(());
    }

    fn size(&self) -> usize {
        return self.size;
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use std::path::PathBuf;

    use anyhow::Result;
    use serde::Deserialize;

    use photonic::boxed::DynOutputDecl;
    use photonic_dynamic::factory::{factory, OutputFactory, Producible};
    use photonic_dynamic::{builder, registry};

    use crate::Terminal;

    #[derive(Deserialize)]
    pub struct Config {
        size: usize,
        path: Option<PathBuf>,
        waterfall: bool,
    }

    impl Producible<dyn DynOutputDecl> for Config {
        type Product = Terminal;

        fn produce<Reg: registry::Registry>(
            config: Self,
            _builder: builder::OutputBuilder<'_, Reg>,
        ) -> Result<Self::Product> {
            return Ok(Terminal {
                size: config.size,
                path: config.path,
                waterfall: config.waterfall,
            });
        }
    }

    pub struct Registry;

    impl registry::Registry for Registry {
        fn output<Reg: registry::Registry>(kind: &str) -> Option<OutputFactory<Reg>> {
            return match kind {
                "terminal" => Some(factory::<Config>()),
                _ => return None,
            };
        }
    }
}
