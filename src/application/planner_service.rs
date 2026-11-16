use crate::{
    application::scanner_service::ScanResult,
    domain::{diff::ManifestDiff, error::CoreError, manifest::Manifest, plan::UpdatePlan},
};

pub struct PlannerService;

impl PlannerService {
    pub fn create_plan(
        old_manifest: Option<&Manifest>,
        new_manifest: &Manifest,
        diff: &ManifestDiff,
        scan: &ScanResult,
    ) -> Result<UpdatePlan, CoreError> {
        // TODO
        todo!();
    }
}
