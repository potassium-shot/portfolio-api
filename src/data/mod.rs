use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Context;
use potassium_shot_plugin::AxumResult;

mod color;
pub mod global_config;
mod image;
mod loc_string;
pub mod project;
pub mod tag;

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("PORTFOLIO_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/usr/share/portfolio"))
});

fn load_all_in_dir<T: serde::de::DeserializeOwned>(path: &Path) -> AxumResult<HashMap<String, T>> {
    Ok(std::fs::read_dir(path)
        .context("While checking contents of portfolio data directory")?
        .filter_map(|element| element.ok())
        .filter(|element| element.file_type().is_ok_and(|ty| ty.is_file()))
        .filter_map(|file| {
            std::fs::read_to_string(file.path()).ok().map(|text| {
                (
                    file.path()
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    text,
                )
            })
        })
        .filter_map(|(id, text)| ron::from_str::<T>(text.as_str()).ok().map(|t| (id, t)))
        .collect())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct PortfolioData {
    pub projects: HashMap<String, project::Project>,
    pub tags: HashMap<String, tag::Tag>,
    pub global_config: global_config::GlobalConfig,
}

impl PortfolioData {
    pub fn load() -> AxumResult<Self> {
        Ok(Self {
            projects: project::Project::load_all()?,
            tags: tag::Tag::load_all()?,
            global_config: global_config::GlobalConfig::load()?,
        })
    }
}
