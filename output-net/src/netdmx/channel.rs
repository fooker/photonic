use palette::rgb::Rgb;
use photonic::math::clamp;
use photonic::Rgbw;
use std::marker::PhantomData;

pub trait Channel<E: ?Sized> {
    fn extract(&self, pixel: &E) -> u8;

    #[inline]
    fn calibrate(self, scale: f32) -> Calibration<Self>
    where Self: Sized {
        return Calibration {
            inner: self,
            scale,
        };
    }
}

impl<E, F> Channel<E> for F
where F: Fn(&E) -> u8
{
    fn extract(&self, pixel: &E) -> u8 {
        return self(pixel);
    }
}

pub struct Calibration<C> {
    inner: C,
    scale: f32,
}

impl<E, C> Channel<E> for Calibration<C>
where
    E: ?Sized,
    C: Channel<E>,
{
    fn extract(&self, pixel: &E) -> u8 {
        return clamp(self.inner.extract(pixel) as f32 * self.scale, (u8::MIN as f32, u8::MAX as f32)) as u8;
    }
}

pub struct Channels<E: ?Sized>(PhantomData<E>);

impl Channels<Rgb> {
    pub fn red() -> impl Channel<Rgb> {
        return |pixel: &Rgb| pixel.into_format::<u8>().red;
    }

    pub fn green() -> impl Channel<Rgb> {
        return |pixel: &Rgb| pixel.into_format::<u8>().green;
    }

    pub fn blue() -> impl Channel<Rgb> {
        return |pixel: &Rgb| pixel.into_format::<u8>().blue;
    }
}

impl Channels<Rgbw> {
    pub fn red() -> impl Channel<Rgbw> {
        return |pixel: &Rgbw| pixel.into_format::<u8>().red;
    }

    pub fn green() -> impl Channel<Rgbw> {
        return |pixel: &Rgbw| pixel.into_format::<u8>().green;
    }

    pub fn blue() -> impl Channel<Rgbw> {
        return |pixel: &Rgbw| pixel.into_format::<u8>().blue;
    }

    pub fn white() -> impl Channel<Rgbw> {
        return |pixel: &Rgbw| pixel.into_format::<u8>().white;
    }
}
