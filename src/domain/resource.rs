use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Mod,
    ResourcePack,
    ShaderPack,
}

impl ResourceType {
    pub fn directory(&self) -> &'static str {
        match self {
            Self::Mod => "mods",
            Self::ResourcePack => "resourcepacks",
            Self::ShaderPack => "shaderpacks",
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstalledResource {
    pub resource_type: ResourceType,
    // 相较于 instance 根目录的路径
    pub relative_path: PathBuf,
    pub file_name: String,
    pub size: u64,
    pub sha256: String,
}
