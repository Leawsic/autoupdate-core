use serde::{Deserialize, Serialize};

use crate::domain::resource::ResourceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO Manifest 简易字段合规性校验（主要校验部分交由 exporter）
pub struct Manifest {
    pub schema_version: u32,
    pub pack: PackInfo,
    pub environment: Environment,
    pub contents: Vec<ContentEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackInfo {
    pub id: String,
    pub name: String,

    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub game_version: String,
    pub loader: String,
    pub loader_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentEntry {
    pub id: String,

    #[serde(rename = "type")]
    pub resource_type: ResourceType,

    pub version: String,

    pub file: FileInfo,

    #[serde(default)]
    pub source: Option<SourceInfo>,

    #[serde(default = "default_required")]
    pub required: bool,

    #[serde(default)]
    pub side: Option<EnvironmentSide>,

    #[serde(default)]
    pub dependencies: Vec<String>,

    #[serde(default)]
    pub order: Option<i32>,

    #[serde(default)]
    pub shader_loader: Option<String>,

    #[serde(default)]
    pub matching_version: Option<String>,
}

fn default_required() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub sha256: String,
    pub url: String,

    #[serde(default)]
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfo {
    pub platform: String,
    pub project_id: Option<String>,
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnvironmentSide {
    Client,
    Server,
    Both,
}
