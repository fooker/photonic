use palette::rgb::Rgb;
use palette::{Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Luv, Okhsl, Okhsv, Okhwb, Oklab, Oklch, Xyz, Yxy};
use photonic::color::rgbw::Rgbw;

macro_rules! color_attrs {
    ($($value:ty { $($channel:ident: $typ:ty[$lb:literal .. $ub:literal],)* } as $name:ident;)*) => {
        $( ::paste::paste!{
            pub mod [<$name>] {
                use super::*;

                pub struct [<$value ColorAttr>]
                <
                    $(
                        [<$channel:snake:upper>],
                    )*
                >
                {
                    $(
                        [<$channel>]: [<$channel:snake:upper>],
                    )*
                }

                impl
                <Result,
                    $(
                        [<$channel:snake:upper>],
                    )*
                >
                ::photonic::Attr<Result> for [<$value ColorAttr>]
                <
                    $(
                        [<$channel:snake:upper>],
                    )*
                >
                where
                    Result: ::photonic::AttrValue + ::palette::FromColor<$value>,
                $(
                    [<$channel:snake:upper>]: ::photonic::Attr<$typ>,
                )*
                {
                    fn update(&mut self, ctx: &::photonic::scene::RenderContext) -> Result {
                        return Result::from_color($value::new(
                            $(
                                self.[<$channel>].update(ctx),
                            )*
                        ));
                    }
                }

                pub struct [<$value Color>]
                <
                    $(
                        [<$channel:snake:upper>],
                    )*
                >
                {
                    $(
                        pub [<$channel>]: [<$channel:snake:upper>],
                    )*
                }

                impl
                <Result,
                    $(
                        [<$channel:snake:upper>],
                    )*
                >
                ::photonic::FreeAttrDecl<Result> for [<$value Color>]
                <
                    $(
                        [<$channel:snake:upper>],
                    )*
                >
                where
                    Result: ::photonic::AttrValue + ::palette::FromColor<$value>,
                $(
                    [<$channel:snake:upper>]: ::photonic::BoundAttrDecl<$typ>,
                )*
                {
                    const KIND: &'static str = stringify!([<$name:camel>]);

                    type Attr = [<$value ColorAttr>] <
                        $(
                            [<$channel:snake:upper>]::Attr,
                        )*
                    >;

                    fn materialize(self, builder: &mut ::photonic::AttrBuilder) -> ::anyhow::Result<Self::Attr> {
                        return ::anyhow::Ok(Self::Attr {
                            $(
                                [<$channel>]: builder.bound_attr(
                                    stringify!($channel),
                                    self.[<$channel>],
                                    ($lb, $ub),
                                )?,
                            )*
                        });
                    }
                }

                #[cfg(feature = "dynamic")]
                pub mod dynamic {
                    use ::photonic::{input, AttrValue};
                    use ::photonic::boxed::{BoxedBoundAttrDecl, DynFreeAttrDecl};
                    use ::photonic_dynamic::{builder, config};
                    use ::photonic_dynamic::factory::Producible;
                    use ::photonic_dynamic::registry::Registry;

                    use super::*;

                    #[derive(::serde::Deserialize, Debug)]
                    pub struct Config {
                        $(
                            [<$channel>]: config::Attr<$typ>,
                        )*
                    }

                    type BoxedColor = [<$value Color>]
                    <
                        $(
                            BoxedBoundAttrDecl<$typ>,
                        )*
                    >;

                    impl<Result> Producible<dyn DynFreeAttrDecl<Result>> for Config
                        where Result: AttrValue + input::Coerced + ::serde::de::DeserializeOwned
                    {
                        default type Product = !;

                        default fn produce<Reg: Registry>(_config: Self, _builder: builder::AttrBuilder<'_, Reg>) -> ::anyhow::Result<Self::Product> {
                            ::anyhow::bail!("Attribute '{}' no available for value type {}", stringify!($name), std::any::type_name::<Result>());
                        }
                    }

                    impl<Result> Producible<dyn DynFreeAttrDecl<Result>> for Config
                        where Result: AttrValue + input::Coerced + ::serde::de::DeserializeOwned + ::palette::FromColor<$value>
                    {
                        type Product = BoxedColor;

                        fn produce<Reg: Registry>(config: Self, mut builder: builder::AttrBuilder<'_, Reg>) -> ::anyhow::Result<Self::Product> {
                            return Ok(Self::Product {
                                $(
                                    [<$channel>]: builder.bound_attr(stringify!($channel), config.[<$channel>])?,
                                )*
                            });
                        }
                    }
                }
            }
        } )*

        #[cfg(feature = "dynamic")]
        pub fn free_attr<Reg: ::photonic_dynamic::registry::Registry, V>(kind: &str) -> Option<::photonic_dynamic::factory::FreeAttrFactory<Reg, V>>
        where V: ::photonic::AttrValue + ::serde::de::DeserializeOwned + ::photonic::input::Coerced {
            return Some(match kind {
                $(
                    concat!("color:", stringify!($name)) => ::photonic_dynamic::factory::factory::<super::$name::dynamic::Config>(),
                )*
                _ => return None,
            });
        }
    }
}

color_attrs! {
    Rgb {
        red: f32[0.0..1.0],
        green: f32[0.0..1.0],
        blue: f32[0.0..1.0],
    } as rgb;

    Rgbw {
        red: f32[0.0..1.0],
        green: f32[0.0..1.0],
        blue: f32[0.0..1.0],
        white: f32[0.0..1.0],
    } as rgbw;

    Hsl {
        hue: f32[0.0..360.0],
        saturation: f32[0.0..1.0],
        lightness: f32[0.0..1.0],
    } as hsl;

    Hsluv {
        hue: f32[0.0..360.0],
        saturation: f32[0.0..1.0],
        lightness: f32[0.0..1.0],
    } as hsl_luv;

    Hsv {
        hue: f32[0.0..360.0],
        saturation: f32[0.0..1.0],
        value: f32[0.0..1.0],
    } as hsv;

    Hwb {
        hue: f32[0.0..360.0],
        whiteness: f32[0.0..1.0],
        blackness: f32[0.0..1.0],
    } as hwb;

    Lab {
        l: f32[0.0..1.0],
        a: f32[0.0..1.0],
        b: f32[0.0..1.0],
    } as lab;

    Lch {
        l: f32[0.0..1.0],
        chroma: f32[0.0..1.0],
        hue: f32[0.0..360.0],
    } as lch;

    Lchuv {
        l: f32[0.0..1.0],
        chroma: f32[0.0..1.0],
        hue: f32[0.0..360.0],
    } as lch_uv;

    Luv {
        l: f32[0.0..1.0],
        u: f32[0.0..1.0],
        v: f32[0.0..1.0],
    } as luv;

    Okhsl {
        hue: f32[0.0..360.0],
        saturation: f32[0.0..1.0],
        lightness: f32[0.0..1.0],
    } as ok_hsl;

    Okhsv {
        hue: f32[0.0..360.0],
        saturation: f32[0.0..1.0],
        value: f32[0.0..1.0],
    } as ok_hsv;

    Okhwb {
        hue: f32[0.0..360.0],
        whiteness: f32[0.0..1.0],
        blackness: f32[0.0..1.0],
    } as ok_hwb;

    Oklab {
        l: f32[0.0..1.0],
        a: f32[0.0..1.0],
        b: f32[0.0..1.0],
    } as ok_lab;

    Oklch {
        l: f32[0.0..1.0],
        chroma: f32[0.0..1.0],
        hue: f32[0.0..360.0],
    } as ok_lch;

    Xyz {
        x: f32[0.0..1.0],
        y: f32[0.0..1.0],
        z: f32[0.0..1.0],
    } as xyz;

    Yxy {
        y: f32[0.0..1.0],
        x: f32[0.0..1.0],
        luma: f32[0.0..1.0],
    } as yxy;
}
