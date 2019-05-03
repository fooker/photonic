use failure::Error;

use photonic::core::{Output, OutputDecl, Render};

pub type Kind = rs_ws281x::StripType;

pub struct StripDecl {
    pub pin: u8,
    pub kind: Kind,
    pub brightness: f64,
}

pub struct Strip {
    size: usize,
    controller: rs_ws281x::Controller,
}

impl OutputDecl for StripDecl {
    type Output = Strip;

    fn new(self, size: usize) -> Result<Self::Output, Error> {
        let controller = rs_ws281x::ControllerBuilder::new()
            .freq(800_000)
            .channel(0, rs_ws281x::ChannelBuilder::new()
                .pin(self.pin as i32)
                .count(size as i32)
                .strip_type(self.kind)
                .brightness((self.brightness * 255.0) as u8)
                .build())
            .render_wait_time(0)
            .build()?;

        return Ok(Self::Output {
            size,
            controller,
        });
    }
}

impl Output for Strip {
    fn render(&mut self, render: &Render) {
        let leds = self.controller.leds_mut(0);

        for i in 0..self.size {
            let (r, g, b) = render.get(i).int_rgb_tup();
            leds[i] = [r, g, b, 0];
        }

        self.controller.render()
            .expect("WS281x render error");
    }
}
