use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};

fn main() -> Result<()> {
    let size: u64 = std::env::var("PHOTONIC_SIZE").unwrap().parse()?;

    let mut reader = std::io::stdin();
    let mut writer = std::io::stdout();

    eprintln!("Here we go - Mario!");

    let mut x = 0.0f64;

    loop {
        // Read duration since last loop from master
        let duration = reader.read_u64::<BigEndian>()?;
        let duration = Duration::from_millis(duration);

        x += duration.as_secs_f64();

        // Fill the buffer with random colors
        for i in 0..size {
            let j = i as f64 + x * 10.0;
            writer.write_all(&[
                ((f64::sin(j / 10.0) * 0.5 + 0.5) * 0xFF as f64) as u8,
                ((f64::sin(j / 20.0) * 0.5 + 0.5) * 0xFF as f64) as u8,
                ((f64::sin(j / 30.0) * 0.5 + 0.5) * 0xFF as f64) as u8,
            ])?;
        }
    }
}
