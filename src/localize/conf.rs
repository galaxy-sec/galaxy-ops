use crate::predule::*;
#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct TemplateConfig {
    origin: (String, String),
    target: (String, String),
}

impl TemplateConfig {
    pub fn example() -> Self {
        TemplateConfig {
            origin: ("[[".into(), "]]".into()),
            target: ("{{".into(), "}}".into()),
        }
    }
    pub fn new(origin: (String, String), target: (String, String)) -> Self {
        Self { origin, target }
    }
}
