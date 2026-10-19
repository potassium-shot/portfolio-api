#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum LocString {
    Loc { en_us: String, fr_fr: String },
    Unloc(String),
}

impl Default for LocString {
    fn default() -> Self {
        Self::Unloc(String::default())
    }
}
