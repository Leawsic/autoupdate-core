use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InstanceContext {
    pub game_directory: PathBuf,
    pub game_version: String,
    pub loader: String,
    pub loader_version: String,
}
