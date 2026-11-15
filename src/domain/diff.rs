use crate::domain::manifest::{ContentEntry, Manifest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeKind {
    Added,
    Removed,
    ContentChanged,
    MetadataChanged,
    // TODO unchanged 是否还有必要记录？
    Unchanged,
}

#[derive(Debug, Clone)]
// 资源变化记录
pub struct ManifestChange {
    pub resource_id: String,
    pub kind: ChangeKind,

    pub old: Option<ContentEntry>,
    pub new: Option<ContentEntry>,
}

#[derive(Debug, Clone)]
pub struct ManifestDiff {
    pub changes: Vec<ManifestChange>,
}

pub struct ManifestDiffService;

impl ManifestDiffService {
    pub fn compare(old: Option<&Manifest>, new: &Manifest) -> ManifestDiff {
        println!("{:?}\n{:?}", old, new);
        // TODO
        todo!();
    }
}
