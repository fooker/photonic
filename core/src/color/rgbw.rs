use std::ops::{BitAnd, Deref, DerefMut};

use palette::bool_mask::HasBoolMask;
use palette::encoding::Srgb;
use palette::num::{PartialCmp, Zero};
use palette::rgb::Rgb;
use palette::stimulus::{FromStimulus, Stimulus};
use palette::{num, Clamp, ClampAssign, Darken, FromColor, IsWithinBounds, SrgbLuma};

/// An white component wrapper for colors.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Rgbw<S = Srgb, T = f32> {
    /// The color.
    pub color: Rgb<S, T>,

    /// The white component. 0.0 is fully off and 1.0 is fully on.
    pub white: T,
}

impl<S, T> Rgbw<S, T> {
    pub fn into_format<U>(self) -> Rgbw<S, U>
    where U: FromStimulus<T> {
        return Rgbw {
            color: self.color.into_format(),
            white: U::from_stimulus(self.white),
        };
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Rgbw<S, U>) -> Self
    where T: FromStimulus<U> {
        return color.into_format();
    }

    /// Convert to a `(red, green, blue, white)` tuple.
    pub fn into_components(self) -> (T, T, T, T) {
        let (r, g, b) = self.color.into_components();
        return (r, g, b, self.white);
    }

    /// Convert from a `(red, green, blue, white)` tuple.
    pub fn from_components((red, green, blue, white): (T, T, T, T)) -> Self {
        return Self {
            color: Rgb::new(red, green, blue),
            white,
        };
    }
}

impl<S, T: Stimulus> Rgbw<S, T> {
    /// Return the `white` value minimum.
    pub fn min_white() -> T {
        return T::zero();
    }

    /// Return the `white` value maximum.
    pub fn max_white() -> T {
        return T::max_intensity();
    }
}

impl<S, T> PartialEq for Rgbw<S, T>
where
    T: PartialEq,
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.white == other.white
    }
}

impl<S, T> Eq for Rgbw<S, T>
where
    T: Eq,
    S: Eq,
{
}

impl<S, T> Deref for Rgbw<S, T> {
    type Target = Rgb<S, T>;

    fn deref(&self) -> &Self::Target {
        &self.color
    }
}

impl<S, T> DerefMut for Rgbw<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.color
    }
}

impl<S, T> IsWithinBounds for Rgbw<S, T>
where
    Rgb<S, T>: IsWithinBounds,
    T: Stimulus + PartialCmp + IsWithinBounds<Mask = <Rgb<S, T> as HasBoolMask>::Mask>,
    <Rgb<S, T> as HasBoolMask>::Mask: BitAnd<Output = <Rgb<S, T> as HasBoolMask>::Mask>,
{
    #[inline]
    fn is_within_bounds(&self) -> <Rgb<S, T> as HasBoolMask>::Mask {
        self.color.is_within_bounds() & self.white.gt_eq(&Self::min_white()) & self.white.lt_eq(&Self::max_white())
    }
}

impl<S, T> Clamp for Rgbw<S, T>
where
    Rgb<S, T>: Clamp,
    T: Stimulus + num::Clamp,
{
    #[inline]
    fn clamp(self) -> Self {
        return Rgbw {
            color: self.color.clamp(),
            white: self.white.clamp(Self::min_white(), Self::max_white()),
        };
    }
}

impl<S, T> ClampAssign for Rgbw<S, T>
where
    Rgb<S, T>: ClampAssign,
    T: Stimulus + num::ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        self.color.clamp_assign();
        self.white.clamp_assign(Self::min_white(), Self::max_white());
    }
}

impl<S, T> HasBoolMask for Rgbw<S, T>
where
    Rgb<S, T>: HasBoolMask,
    T: HasBoolMask<Mask = <Rgb<S, T> as HasBoolMask>::Mask>,
{
    type Mask = <Rgb<S, T> as HasBoolMask>::Mask;
}

impl<S, T> Default for Rgbw<S, T>
where T: Stimulus
{
    fn default() -> Rgbw<S, T> {
        return Rgbw {
            color: Rgb::default(),
            white: Self::max_white(),
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WhiteMode {
    /// No auto white calculation is performed.
    None,

    /// This mode sets the calculated white value from the RGB
    /// channels. This adds additional brightness but keeps
    /// "RGB-White" resulting in a brighter white as both, the white
    /// channel and the RGB channels being utilized for pure white.
    Brighter,

    /// This mode subtracts the calculated white value from the RGB
    /// channels. This gets rid of the "RGB-white" but means that the
    /// light is less bright with only the white channel and not the
    /// RGB channels being utilized for pure white.
    Accurate,
}

impl Default for WhiteMode {
    fn default() -> Self {
        return Self::None;
    }
}

impl WhiteMode {
    /// Transforms the color into a color having an extra white
    /// channel using the given white mode. If `color` already has a
    /// white channel, it will be overwritten.
    pub fn apply<C, W>(&self, color: C) -> C::WithWhite
    where
        C: WithWhite<W> + Darken<Scalar = W> + Copy,
        W: Stimulus + Copy,
        SrgbLuma<W>: FromColor<C>,
    {
        let w = SrgbLuma::<W>::from_color(color);
        return match self {
            Self::None => color.black(),
            Self::Brighter => color.with_white(w.luma),
            Self::Accurate => color.darken_fixed(w.luma).with_white(w.luma),
        };
    }
}

pub trait WithWhite<W>: Sized
where W: Stimulus
{
    /// The opaque color type, without any white channel.
    ///
    /// This is typically `Self`.
    type Color;

    /// The color type with white channel applied.
    ///
    /// This is typically `White<Self::Color, W>`.
    type WithWhite: WithWhite<W, Color = Self::Color, WithWhite = Self::WithWhite>;

    /// Transforms the color into a color having an extra white
    /// channel with the provided white value. If `Self` already has
    /// a white channel, it is overwritten.
    #[must_use]
    fn with_white(self, white: W) -> Self::WithWhite;

    /// Removes the white channel from the color. If `Self::Color` has
    /// an internal white channel field, that field will be set to
    /// `W::min_intensity()`.
    #[must_use]
    fn without_white(self) -> Self::Color;

    /// Splits the color into separate color and white values.
    ///
    /// A color without any white channel field will return
    /// `W::min_intensity()` instead. If `Self::Color` has an internal
    /// white channel field, that field will be set to
    /// `W::min_intensity()`.
    #[must_use]
    fn split(self) -> (Self::Color, W);

    /// Transforms the color into a non white color with a white channel
    /// field. If `Self` already has a white channel, it is overwritten.
    #[must_use]
    #[inline]
    fn full(self) -> Self::WithWhite
    where W: Stimulus {
        self.with_white(W::max_intensity())
    }

    /// Transforms the color into a fully white color. If `Self`
    /// already has a white channel, it is overwritten.
    #[must_use]
    #[inline]
    fn black(self) -> Self::WithWhite
    where W: Zero {
        self.with_white(W::zero())
    }
}

impl<S, T> WithWhite<T> for Rgbw<S, T>
where T: Stimulus
{
    type Color = Rgb<S, T>;
    type WithWhite = Self;

    fn with_white(mut self, white: T) -> Self::WithWhite {
        self.white = white;
        return self;
    }

    fn without_white(self) -> Self::Color {
        return self.color;
    }

    fn split(self) -> (Self::Color, T) {
        return (self.color, self.white);
    }
}

impl<S, T> WithWhite<T> for Rgb<S, T>
where T: Stimulus
{
    type Color = Self;
    type WithWhite = Rgbw<S, T>;

    fn with_white(self, white: T) -> Self::WithWhite {
        return Self::WithWhite {
            color: self,
            white,
        };
    }

    fn without_white(self) -> Self::Color {
        return self;
    }

    fn split(self) -> (Self::Color, T) {
        return (self, T::zero());
    }
}
