use crate::domain::{error::CoreError, instance::InstanceContext, resource::InstalledResource};

pub struct ScannerService;

impl ScannerService {
    pub fn scan(context: &InstanceContext) -> Result<ScanResult, CoreError> {
        // TODO
        todo!()
    }
}

pub struct ScanResult {
    pub resources: Vec<InstalledResource>,
}
