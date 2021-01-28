use std::process::{Child, Command, Stdio};
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use failure::{Error, format_err};
use shared_memory::{SharedMemRaw, SharedMemCast, ReadRaw};

use photonic_core::color::*;
use photonic_core::core::*;
use palette::Component;
use std::io::Write;

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

    fn get(&self, index: usize) -> Self::Element {
        let element = &unsafe { self.shm.get_raw_slice::<Element>() }[index];
        return Self::Element::from_components((element.r.convert(), element.g.convert(), element.b.convert()));
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

    fn materialize(self, size: usize, _builder: &mut NodeBuilder) -> Result<Self::Target, Error> {
        let command = shlex::split(&self.command)
            .ok_or(format_err!("Invalid command: {}", &self.command))?;
        let (command, args) = command.split_first()
            .ok_or(format_err!("Empty command"))?;

        // Create shared memory region for color buffer
        let shm = SharedMemRaw::create(&format!("photonic-{}-{}", std::process::id(), ID.fetch_add(1, Ordering::SeqCst)), size * 3)
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

        return Ok(Self::Target {
            child,
            shm,
        });
    }
}

impl<'a> RenderType<'a> for ExecNode {
    type Element = RGBColor;
    type Render = ExecRenderer<'a>;
}

impl Node for ExecNode {
    const KIND: &'static str = "exec";

    fn update(&mut self, duration: &Duration) {
        let stdin = self.child.stdin.as_mut()
            .expect("StdIn missing");

        let stdout = self.child.stdout.as_mut()
            .expect("StdOut missing");

        // Send the duration to the child process
        stdin.write_u64::<BigEndian>(duration.as_millis() as u64)
            .expect("Failed to write to child process");
        stdin.flush()
            .expect("Failed to flush to child process");

        // Wait for any char from the child to signal it's ready
        stdout.read_u8()
            .expect("Failed to write to child process");
    }

    fn render<'a>(&'a self, _renderer: &Renderer) -> <Self as RenderType<'a>>::Render {
        return ExecRenderer {
            shm: &self.shm,
        };
    }
}
