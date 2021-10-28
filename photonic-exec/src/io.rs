use std::io::{Read, Write};
use std::mem;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use anyhow::{format_err, Context, Result};
use byteorder::{BigEndian, WriteBytesExt};

use photonic_core::element::RGBColor;
use photonic_core::node::{MapRender, Render, RenderType};
use photonic_core::scene::NodeBuilder;
use photonic_core::{Buffer, Node, NodeDecl};

use super::Element;

#[cfg_attr(feature = "dyn", derive(serde::Deserialize))]
pub struct IOExecNodeDecl {
    pub command: String,
}

pub struct IOExecNode {
    buffer: Buffer<Element>,
    child: Child,
}

impl NodeDecl for IOExecNodeDecl {
    type Element = RGBColor;
    type Target = IOExecNode;

    fn materialize(self, size: usize, _builder: &mut NodeBuilder) -> Result<Self::Target> {
        let command = shlex::split(&self.command)
            .ok_or_else(|| format_err!("Invalid command: {}", &self.command))?;
        let (command, args) = command.split_first().ok_or_else(|| format_err!("Empty command"))?;

        let buffer = Buffer::new(size);

        // Spawn a child process
        let child = Command::new(command)
            .args(args)
            .env("PHOTONIC_SIZE", format!("{}", size))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        return Ok(Self::Target {
            buffer,
            child,
        });
    }
}

impl<'a> RenderType<'a, Self> for IOExecNode {
    type Render = MapRender<'a, &'a Buffer<Element>, RGBColor>;
}

impl Node for IOExecNode {
    const KIND: &'static str = "exec";

    type Element = RGBColor;

    fn update(&mut self, duration: Duration) -> Result<()> {
        let stdin = self.child.stdin.as_mut().context("StdIn missing")?;
        let stdout = self.child.stdout.as_mut().context("StdOut missing")?;

        // Send the duration to the child process
        stdin
            .write_u64::<BigEndian>(duration.as_millis() as u64)
            .context("Failed to write to child")?;
        stdin.flush().context("Failed to write to child")?;

        // Read the buffer content from child
        stdout
            .read_exact(bytemuck::cast_slice_mut(self.buffer.as_mut()))
            .context("Failed to read from child process")?;

        return Ok(());
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(Render::map(&self.buffer, &RGBColor::from_format));
    }
}

impl Drop for IOExecNode {
    fn drop(&mut self) {
        mem::drop(self.child.stdout.take());
        mem::drop(self.child.stdin.take());
    }
}
