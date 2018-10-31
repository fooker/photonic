use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub enum EasingFuncConfig {
    #[serde(rename = "linear")]
    Linear,

    #[serde(rename = "in_quad")]
    InQuad,

    #[serde(rename = "out_quad")]
    OutQuad,

    #[serde(rename = "quad")]
    Quad,

    #[serde(rename = "in_cubic")]
    InCubic,

    #[serde(rename = "out_cubic")]
    OutCubic,

    #[serde(rename = "cubic")]
    Cubic,

    #[serde(rename = "in_quart")]
    InQuart,

    #[serde(rename = "out_quart")]
    OutQuart,

    #[serde(rename = "quart")]
    Quart,

    #[serde(rename = "in_quint")]
    InQuint,

    #[serde(rename = "out_quint")]
    OutQuint,

    #[serde(rename = "quint")]
    Quint,

    #[serde(rename = "in_sine")]
    InSine,

    #[serde(rename = "out_sine")]
    OutSine,

    #[serde(rename = "sine")]
    Sine,

    #[serde(rename = "in_expo")]
    InExpo,

    #[serde(rename = "out_expo")]
    OutExpo,

    #[serde(rename = "expo")]
    Expo,

    #[serde(rename = "in_elastic")]
    InElastic,

    #[serde(rename = "out_elastic")]
    OutElastic,

    #[serde(rename = "elastic")]
    Elastic,

    #[serde(rename = "in_back")]
    InBack,

    #[serde(rename = "out_back")]
    OutBack,

    #[serde(rename = "back")]
    Back,

    #[serde(rename = "in_bounce")]
    InBounce,

    #[serde(rename = "out_bounce")]
    OutBounce,

    #[serde(rename = "bounce")]
    Bounce,
}

#[derive(Serialize, Deserialize)]
pub struct EasingConfig {
    pub speed: f64,
    pub func: EasingFuncConfig,
}

#[derive(Serialize, Deserialize)]
pub struct FaderConfig {
    pub initial_value: f64,

    pub easing: Option<EasingConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonConfig {
    pub value_released: f64,
    pub value_pressed: f64,

    pub hold_time: f64,

    pub easing_pressed: Option<EasingConfig>,
    pub easing_released: Option<EasingConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct SequenceConfig {
    pub values: Vec<f64>, // FIXME: Other value sets...

    pub duration: f64,

    pub easing: Option<EasingConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BehaviorConfig {
    #[serde(rename = "fader")]
    Fader(FaderConfig),

    #[serde(rename = "button")]
    Button(ButtonConfig),

    #[serde(rename = "sequence")]
    Sequence(SequenceConfig),
}


#[derive(Serialize, Deserialize)]
pub struct DynamicValueConfig {
    pub name: String,

    #[serde(flatten)]
    pub behavior: BehaviorConfig,
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
pub struct OverlayNodeConfig {
    pub base: Box<NodeConfig>,
    pub overlay: Box<NodeConfig>,
    pub blend: Box<ValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct SwitchNodeConfig {
    pub sources: Vec<Box<NodeConfig>>,
    pub position: Box<ValueConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeImplConfig {
    #[serde(rename = "blackout")]
    Blackout(BlackoutNodeConfig),

    #[serde(rename = "colorwheel")]
    Colorwheel(ColorwheelNodeConfig),

    #[serde(rename = "rotation")]
    Rotation(RotationNodeConfig),

    #[serde(rename = "raindrops")]
    Raindrops(RaindropsNodeConfig),

    #[serde(rename = "larson")]
    Larson(LarsonNodeConfig),

    #[serde(rename = "overlay")]
    Overlay(OverlayNodeConfig),

    #[serde(rename = "switch")]
    Switch(SwitchNodeConfig),
}

#[derive(Serialize, Deserialize)]
pub struct NodeConfig {
    pub name: String,

    #[serde(flatten)]
    pub config: NodeImplConfig,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub node: Box<NodeConfig>,
}
