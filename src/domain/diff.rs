use std::collections::HashMap;

use crate::domain::manifest::{ContentEntry, Manifest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeKind {
    Added,
    Removed,
    ContentChanged,
    // TODO 仅文件名、元数据变化，哈希不变的情况交由 Planner 处理
    Unchanged,
}

#[derive(Debug, Clone)]
// 资源变化记录
pub struct ManifestChange {
    pub kind: ChangeKind,

    pub old: Option<ContentEntry>,
    pub new: Option<ContentEntry>,
}

impl ManifestChange {
    pub fn resource_id(&self) -> &str {
        self.old.as_ref().or(self.new.as_ref()).unwrap().id.as_str()
    }
}

impl ManifestChange {
    pub fn added(new: ContentEntry) -> ManifestChange {
        ManifestChange {
            kind: ChangeKind::Added,
            old: None,
            new: Some(new),
        }
    }
    pub fn removed(old: ContentEntry) -> ManifestChange {
        ManifestChange {
            kind: ChangeKind::Removed,
            old: Some(old),
            new: None,
        }
    }
    pub fn both_exist(old: ContentEntry, new: ContentEntry, kind: ChangeKind) -> ManifestChange {
        ManifestChange {
            kind,
            old: Some(old),
            new: Some(new),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManifestDiff {
    pub changes: Vec<ManifestChange>,
}

pub struct ManifestDiffService;

impl ManifestDiffService {
    pub fn compare(old: Option<&Manifest>, new: &Manifest) -> ManifestDiff {
        let mut res = ManifestDiff {
            changes: Vec::new(),
        };

        match old {
            None => {
                return all_new_diff(new);
            }
            Some(ctnt) => {
                let old_map: HashMap<&str, &ContentEntry> =
                    ctnt.contents.iter().map(|c| (c.id.as_str(), c)).collect();
                let new_map: HashMap<&str, &ContentEntry> =
                    new.contents.iter().map(|c| (c.id.as_str(), c)).collect();
                for entry in &ctnt.contents {
                    match new_map.get(entry.id.as_str()) {
                        Some(e) => {
                            let k = if entry.file.sha256 != e.file.sha256 {
                                ChangeKind::ContentChanged
                            } else {
                                ChangeKind::Unchanged
                            };
                            res.changes.push(ManifestChange::both_exist(
                                entry.clone(),
                                (*e).clone(),
                                k,
                            ));
                        }
                        None => {
                            res.changes.push(ManifestChange::removed(entry.clone()));
                        }
                    }
                }
                for new_entry in &new.contents {
                    if let None = old_map.get(new_entry.id.as_str()) {
                        res.changes.push(ManifestChange::added(new_entry.clone()));
                    }
                }
            }
        }
        res
    }
}

fn all_new_diff(new: &Manifest) -> ManifestDiff {
    let mut res = ManifestDiff {
        changes: Vec::new(),
    };
    for entry in new.contents.iter() {
        res.changes.push(ManifestChange::added(entry.clone()));
    }
    res
}
