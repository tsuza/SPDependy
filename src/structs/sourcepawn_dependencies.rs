#[derive(serde::Deserialize)]
pub struct DependencyConfigFile {
    pub dependency: Vec<Dependency>,
}

#[derive(serde::Deserialize)]
pub struct Dependency {
    #[serde(default)]
    pub path: String,

    pub url: String,
}
