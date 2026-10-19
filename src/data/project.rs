use std::collections::HashMap;
use std::{path::PathBuf, sync::LazyLock};

use potassium_shot_plugin::AxumResult;

use crate::data::loc_string::LocString;

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("projects"));

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Project {
    pub name: LocString,
    pub tags: Vec<String>,
    pub short_description: LocString,
    pub images: Vec<String>,
    pub description: LocString,
    #[serde(default)]
    pub order: i32,
}

impl Project {
    pub fn load_all() -> AxumResult<HashMap<String, Self>> {
        super::load_all_in_dir(&PATH)
    }
}
