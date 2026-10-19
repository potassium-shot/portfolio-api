use anyhow::Context;
use potassium_shot_plugin::AxumResult;

use crate::data::loc_string::LocString;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct GlobalConfig {
    pub home_message: LocString,
    pub about_me: LocString,
    pub phone: String,
    pub email: String,
}

impl GlobalConfig {
    pub fn load() -> AxumResult<Self> {
        let str = std::fs::read_to_string(super::CONFIG_PATH.join("global_config.ron"))
            .context("Reading global portfolio config")?;
        Ok(ron::from_str(str.as_str()).context("Parsing global portfolio config")?)
    }
}
