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
pub struct RotationNodeConfig {
    pub source: Box<NodeConfig>,

    pub speed: Box<ValueConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeImplConfig {
    Blackout(BlackoutNodeConfig),
    Colorwheel(ColorwheelNodeConfig),
    Rotation(RotationNodeConfig),
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
