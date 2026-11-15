use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalState {
    // 对应此 state 时 pack 的 manifest version
    pub state_version: u32,
    pub pack_id: String,
    pub release_id: Option<String>,
    pub manifest_hash: String,
    // 应用 pack 的时间戳
    pub applied_at: String,
}
