use crate::domain::resource::ResourceType;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct UpdateAction {
    pub resource_id: String,
    pub action_type: ActionType,
    pub resource_type: ResourceType,
    // 相对于游戏 instance 根目录
    pub target_path: PathBuf,
    pub download_url: Option<String>,
    // 目标文件预期哈希
    pub expected_sha256: Option<String>,
    pub expected_size: Option<u64>,
    // 留作 Update 和 Remove 用
    pub old_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct UpdatePlan {
    pub pack_id: String,
    pub from_version: Option<String>,
    pub to_version: String,
    pub actions: Vec<UpdateAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    Add,
    Update,
    Remove,
    Keep,
    Conflict,
}
