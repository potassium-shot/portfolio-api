use std::collections::HashMap;
use std::{path::PathBuf, sync::LazyLock};

use potassium_shot_plugin::AxumResult;

use crate::data::color::PortfolioColor;
use crate::data::loc_string::LocString;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("tags"));

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Tag {
    pub name: LocString,
    pub color: PortfolioColor,
    pub order: i32,
}

impl Tag {
    pub fn load_all() -> AxumResult<HashMap<String, Self>> {
        super::load_all_in_dir(&PATH)
    }
}
