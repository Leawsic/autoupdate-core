use crate::domain::manifest::{ContentEntry, Manifest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeKind {
    Added,
    Removed,
    ContentChanged,
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

impl ManifestChange {
    pub fn added(new: &ContentEntry) -> ManifestChange {
        ManifestChange {
            resource_id: new.id.clone(),
            kind: ChangeKind::Added,
            old: None,
            new: Some(new.clone()),
        }
    }
    pub fn removed(old: &ContentEntry) -> ManifestChange {
        ManifestChange {
            resource_id: old.id.clone(),
            kind: ChangeKind::Removed,
            old: Some(old.clone()),
            new: None,
        }
    }
    pub fn both_exist(old: &ContentEntry, new: &ContentEntry, kind: ChangeKind) -> ManifestChange {
        ManifestChange {
            resource_id: old.id.clone(),
            kind: kind,
            old: Some(old.clone()),
            new: Some(new.clone()),
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
            Some(content) => {
                for entry in content.contents.clone() {
                    // TODO 频繁比较是否会有性能问题
                    if !new.contents.iter().any(|x| x.id == entry.id) {
                        res.changes.push(ManifestChange::removed(&entry));
                    } else {
                        let n = new.contents.iter().find(|&x| x.id == entry.id).unwrap();
                        if n.file.sha256 != entry.file.sha256 {
                            res.changes.push(ManifestChange::both_exist(
                                &entry,
                                n,
                                ChangeKind::ContentChanged,
                            ));
                            continue;
                        } else {
                            res.changes.push(ManifestChange::both_exist(
                                &entry,
                                n,
                                ChangeKind::Unchanged,
                            ));
                        }
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
    for entry in new.contents.clone() {
        res.changes.push(ManifestChange::added(&entry));
    }
    res
}
