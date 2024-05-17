pub use alert::Alert;
pub use blackout::Blackout;
pub use brightness::Brightness;
pub use color_wheel::ColorWheel;
pub use larson::Larson;
pub use noise::Noise;
pub use overlay::Overlay;
pub use raindrops::Raindrops;
pub use solid::Solid;
pub use splice::Splice;
pub use switch::Switch;

#[cfg(feature = "dynamic")]
use photonic_dynamic::{BoxedNodeDecl, NodeBuilder, NodeFactory, Registry};

mod alert;
mod blackout;
mod brightness;
mod color_wheel;
mod larson;
mod noise;
mod overlay;
mod raindrops;
mod solid;
mod splice;
mod switch;

#[cfg(feature = "dynamic")]
pub struct NodeRegistry;

#[cfg(feature = "dynamic")]
impl<B> Registry<BoxedNodeDecl, B> for NodeRegistry
where B: NodeBuilder
{
    fn lookup(kind: &str) -> Option<Box<NodeFactory<B>>> {
        return Some(match kind {
            "alert" => alert::factory(),
            "blackout" => blackout::factory(),
            "brightness" => brightness::factory(),
            "color-wheel" => color_wheel::factory(),
            "larson" => larson::factory(),
            "noise" => noise::factory(),
            "overlay" => overlay::factory(),
            "raindrops" => raindrops::factory(),
            "solid" => solid::factory(),
            "splice" => splice::factory(),
            "switch" => switch::factory(),
            _ => return None,
        });
    }
}
