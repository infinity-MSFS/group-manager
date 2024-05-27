use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    pub owner: String,
    pub repoName: String,
    pub version: String,
    pub fileName: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub date: String,
    pub changelog: String,
    pub overview: String,
    pub description: String,
    pub background: String,
    pub pageBackground: Option<String>,
    pub variants: Option<Vec<String>>,
    pub package: Option<Package>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Palette {
    pub primary: String,
    pub secondary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BetaProject {
    pub background: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupData {
    pub name: String,
    pub projects: Vec<Project>,
    pub beta: BetaProject,
    pub logo: String,
    pub update: Option<bool>,
    pub path: String,
    pub palette: Palette,
}
