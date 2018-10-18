#[derive(Serialize, Deserialize)]
pub enum EasingFuncConfig {}


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DynamicValueConfig {
    Fader {
        speed: f64,
        easing: EasingFuncConfig,
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueConfig {
    Fixed(f64),
    Dynamic(DynamicValueConfig),
}

#[derive(Serialize, Deserialize)]
pub struct ValueRangeConfig {
    pub min: Box<ValueConfig>,
    pub max: Box<ValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct BlackoutNodeConfig {
    pub source: Box<NodeConfig>,

    pub range: Option<(usize, usize)>,
    pub value: Box<ValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ColorwheelNodeConfig {
    pub offset: f64,
    pub delta: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct RaindropsNodeConfig {
    pub rate: Box<ValueConfig>,

    pub hue: Box<ValueRangeConfig>,
    pub saturation: Box<ValueRangeConfig>,
    pub lightness: Box<ValueRangeConfig>,

    pub decay: Box<ValueRangeConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct RotationNodeConfig {
    pub source: Box<NodeConfig>,

    pub speed: Box<ValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct LarsonNodeConfig {
    pub hue: Box<ValueConfig>,
    pub speed: Box<ValueConfig>,
    pub width: Box<ValueConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeImplConfig {
    Blackout(BlackoutNodeConfig),
    Colorwheel(ColorwheelNodeConfig),
    Rotation(RotationNodeConfig),
    Raindrops(RaindropsNodeConfig),
    Larson(LarsonNodeConfig),
}

#[derive(Serialize, Deserialize)]
pub struct NodeConfig {
    pub name: String,

    #[serde(flatten)]
    pub config: NodeImplConfig,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub size: usize,

    pub node: Box<NodeConfig>,
}
