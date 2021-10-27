use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Write;
use std::time::Duration;
use shared_memory::ShmemConf;

#[repr(C, packed)]
struct Element {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let size = std::env::var("PHOTONIC_SIZE").unwrap().parse().unwrap();
    let path = std::env::var("PHOTONIC_PATH").unwrap();

    let mut reader = std::io::stdin();
    let mut writer = std::io::stdout();

    let mut shm = ShmemConf::new().open()::open(&path).unwrap();

    eprintln!("Here we go - Mario!");

    let mut x = 0.0f64;

    loop {
        // Read duration since last loop from master
        let duration = reader.read_u64::<BigEndian>().unwrap();
        let duration = Duration::from_millis(duration);

        x += duration.as_secs_f64();

        // Fill the buffer with random colors
        let slice = unsafe { shm.get_raw_slice_mut::<Element>() };
        for i in 0..size {
            let j = i as f64 + x * 10.0;
            slice[i].r = ((f64::sin(j / 10.0) * 0.5 + 0.5) * 0xFF as f64) as u8;
            slice[i].g = ((f64::sin(j / 20.0) * 0.5 + 0.5) * 0xFF as f64) as u8;
            slice[i].b = ((f64::sin(j / 30.0) * 0.5 + 0.5) * 0xFF as f64) as u8;
        }

        // Send signal to master that buffer is filled
        writer.write_u8(0).unwrap();
        writer.flush().unwrap();
    }
}
