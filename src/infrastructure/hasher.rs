use std::path::Path;

use crate::domain::error::CoreError;

pub struct Hasher;

impl Hasher {
    pub fn sha256_file(path: &Path) -> Result<String, CoreError> {
        // TODO
        todo!();
    }
}
