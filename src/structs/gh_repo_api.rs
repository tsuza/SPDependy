use serde::{Deserialize, Deserializer};

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum RepositoryLayout {
    Array(Vec<FileInfo>),
    Single(FileInfo),
}

#[derive(serde::Deserialize)]
pub struct FileInfo {
    pub name: String,

    #[serde(deserialize_with = "deserialize_null_default")]
    pub download_url: String,

    #[serde(rename = "type", default)]
    pub file_type: String,

    #[serde(rename = "_links")]
    pub links: TemporaryName,
}

#[derive(serde::Deserialize)]
pub struct TemporaryName {
    #[serde(rename = "self")]
    pub link: String,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

impl RepositoryLayout {
    pub fn parse(self) -> Vec<FileInfo> {
        match self {
            RepositoryLayout::Array(t) => t,
            RepositoryLayout::Single(r) => vec![r],
        }
    }
}
