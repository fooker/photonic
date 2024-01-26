use std::io::Write;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use anyhow::{Context, Result};
use palette::Srgb;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use photonic::{BufferReader, Output, OutputDecl};

pub struct Terminal {
    pub waterfall: bool,
    pub path: Option<PathBuf>,
}

impl Terminal {
    pub fn stdout() -> Self {
        return Self {
            waterfall: false,
            path: None,
        };
    }

    pub fn with_path(path: impl AsRef<Path>) -> Self {
        return Self {
            waterfall: false,
            path: Some(path.as_ref().to_path_buf()),
        }
    }

    pub fn with_waterfall(mut self, waterfall: bool) -> Self {
        self.waterfall = waterfall;
        return self;
    }
}

pub struct TerminalOutput {
    waterfall: bool,
    out: Pin<Box<dyn AsyncWrite>>,
}


impl OutputDecl for Terminal
{
    type Output = TerminalOutput;

    async fn materialize(self, _size: usize) -> Result<Self::Output>
        where Self::Output: Sized,
    {
        let out: Pin<Box<dyn AsyncWrite>> = if let Some(path) = self.path {
            let _ = nix::unistd::unlink(&path);
            nix::unistd::mkfifo(&path, nix::sys::stat::Mode::S_IRWXU)
                .with_context(|| format!("Failed to create output fifo: '{}'", path.display()))?;

            Box::pin(tokio::net::unix::pipe::OpenOptions::new()
                .read_write(true)
                .open_sender(&path)
                .with_context(|| format!("Failed to open output fifo: '{}'", path.display()))?)
        } else {
            Box::pin(tokio::io::stdout())
        };

        return Ok(Self::Output {
            waterfall: self.waterfall,
            out,
        });
    }
}

impl Output for TerminalOutput
{
    const KIND: &'static str = "terminal";

    type Element = Srgb;

    async fn render(&mut self, out: impl BufferReader<Element=Self::Element>) -> Result<()> {
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
}
