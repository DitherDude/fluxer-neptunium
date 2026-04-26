use crate::id::{Id, marker::EmojiMarker};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Emoji {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<EmojiMarker>>,
    #[serde(default)]
    pub animated: bool,
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.id.is_some() {
            f.write_str(":")?;
            f.write_str(&self.name)?;
            f.write_str(":")
        } else {
            f.write_str(&self.name)
        }
    }
}

impl From<&str> for Emoji {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<String> for Emoji {
    fn from(value: String) -> Self {
        Self {
            name: value,
            id: None,
            animated: false,
        }
    }
}
