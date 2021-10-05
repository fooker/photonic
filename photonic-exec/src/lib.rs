#![allow(clippy::needless_return)]

use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use anyhow::{Result, format_err, Context};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use shared_memory::{ReadRaw, SharedMemCast, SharedMemRaw};

use photonic_core::color::*;
use photonic_core::color::palette::LinSrgb;
use photonic_core::node::{Node, NodeDecl, Render, RenderType};
use photonic_core::scene::*;

#[repr(C, packed)]
struct Element {
    r: u8,
    g: u8,
    b: u8,
}

unsafe impl SharedMemCast for Element {}

pub struct ExecRenderer<'a> {
    shm: &'a SharedMemRaw,
}

impl<'a> Render for ExecRenderer<'a> {
    type Element = RGBColor;

    fn get(&self, index: usize) -> Result<Self::Element> {
        let element = &unsafe { self.shm.get_raw_slice::<Element>() }[index];
        return Ok(LinSrgb::<u8>::from_components((element.r, element.g, element.b)).into_format());
    }
}

pub struct ExecNodeDecl {
    pub command: String,
}

pub struct ExecNode {
    child: Child,
    shm: SharedMemRaw,
}

static ID: AtomicUsize = AtomicUsize::new(0);

impl NodeDecl for ExecNodeDecl {
    type Element = RGBColor;
    type Target = ExecNode;

    fn materialize(self, size: usize, _builder: &mut NodeBuilder) -> Result<Self::Target> {
        let command = shlex::split(&self.command)
            .ok_or_else(|| format_err!("Invalid command: {}", &self.command))?;
        let (command, args) = command
            .split_first()
            .ok_or_else(|| format_err!("Empty command"))?;

        // Create shared memory region for color buffer
        let shm = SharedMemRaw::create(
            &format!(
                "photonic-{}-{}",
                std::process::id(),
                ID.fetch_add(1, Ordering::SeqCst)
            ),
            size * 3,
        )
            .expect("Failed to create SHM");

        // Spawn a child process
        let child = Command::new(command)
            .args(args)
            .env("PHOTONIC_SIZE", format!("{}", size))
            .env("PHOTONIC_PATH", shm.get_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        return Ok(Self::Target { child, shm });
    }
}

impl<'a> RenderType<'a, Self> for ExecNode {
    type Render = ExecRenderer<'a>;
}

impl Node for ExecNode {
    const KIND: &'static str = "exec";

    type Element = RGBColor;

    fn update(&mut self, duration: Duration) -> Result<()> {
        let stdin = self.child.stdin.as_mut()
            .context("StdIn missing")?;

        let stdout = self.child.stdout.as_mut()
            .context("StdOut missing")?;

        // Send the duration to the child process
        stdin
            .write_u64::<BigEndian>(duration.as_millis() as u64)
            .context("Failed to write to child process")?;
        stdin.flush().context("Failed to flush to child process")?;

        // Wait for any char from the child to signal it's ready
        stdout.read_u8().context("Failed to write to child process")?;

        return Ok(());
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(ExecRenderer { shm: &self.shm });
    }
}
