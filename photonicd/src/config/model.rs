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
pub struct FaderFloatValueConfig {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,

    pub easing: Option<EasingConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonFloatValueConfig {
    pub value_released: Option<f64>,
    pub value_pressed: Option<f64>,

    pub hold_time: f64,

    pub auto_trigger: Option<f64>,

    pub easing_pressed: Option<EasingConfig>,
    pub easing_released: Option<EasingConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct SequenceFloatValueConfig {
    pub values: Vec<f64>, // FIXME: Other value sets...

    pub auto_trigger: Option<f64>,

    pub easing: Option<EasingConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct RandomFloatValueConfig {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,

    pub auto_trigger: Option<f64>,

    pub easing: Option<EasingConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DynamicFloatValueDetailsConfig {
    #[serde(rename = "fader")]
    Fader(FaderFloatValueConfig),

    #[serde(rename = "button")]
    Button(ButtonFloatValueConfig),

    #[serde(rename = "sequence")]
    Sequence(SequenceFloatValueConfig),

    #[serde(rename = "random")]
    Random(RandomFloatValueConfig),
}

#[derive(Serialize, Deserialize)]
pub struct DynamicFloatValueConfig {
    pub name: Option<String>,

    #[serde(flatten)]
    pub details: DynamicFloatValueDetailsConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FloatValueConfig {
    Fixed(f64),
    Dynamic(DynamicFloatValueConfig),
}

#[derive(Serialize, Deserialize)]
pub struct ManualIntValueConfig {
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct LoopIntValueConfig {
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,

    pub step: Option<i64>,

    pub auto_trigger: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct SequenceIntValueConfig {
    pub values: Vec<i64>,

    pub auto_trigger: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct RandomIntValueConfig {
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,

    pub auto_trigger: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DynamicIntValueDetailsConfig {
    #[serde(rename = "manual")]
    Manual(ManualIntValueConfig),

    #[serde(rename = "loop")]
    Loop(LoopIntValueConfig),

    #[serde(rename = "sequence")]
    Sequence(SequenceIntValueConfig),

    #[serde(rename = "random")]
    Random(RandomIntValueConfig),
}

#[derive(Serialize, Deserialize)]
pub struct DynamicIntValueConfig {
    pub name: Option<String>,

    #[serde(flatten)]
    pub details: DynamicIntValueDetailsConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntValueConfig {
    Fixed(i64),
    Dynamic(DynamicIntValueConfig),
}

#[derive(Serialize, Deserialize)]
pub struct ValueRangeConfig {
    pub min: Box<FloatValueConfig>,
    pub max: Box<FloatValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct BlackoutNodeConfig {
    pub source: Box<NodeConfig>,

    pub range: Option<(usize, usize)>,
    pub value: Box<FloatValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ColorwheelNodeConfig {
    pub offset: f64,
    pub delta: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct RaindropsNodeConfig {
    pub rate: Box<FloatValueConfig>,

    pub hue: Box<ValueRangeConfig>,
    pub saturation: Box<ValueRangeConfig>,
    pub lightness: Box<ValueRangeConfig>,

    pub decay: Box<ValueRangeConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct RotationNodeConfig {
    pub source: Box<NodeConfig>,

    pub speed: Box<FloatValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct LarsonNodeConfig {
    pub hue: Box<FloatValueConfig>,
    pub speed: Box<FloatValueConfig>,
    pub width: Box<FloatValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct OverlayNodeConfig {
    pub base: Box<NodeConfig>,
    pub overlay: Box<NodeConfig>,
    pub blend: Box<FloatValueConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct SwitchNodeConfig {
    pub sources: Vec<Box<NodeConfig>>,
    pub position: Box<IntValueConfig>,
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
