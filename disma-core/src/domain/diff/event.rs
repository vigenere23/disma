use std::sync::Arc;

use super::base::Diff;

pub trait DiffEventListener {
    fn after_diff_executed(&self, diff: Diff);
}
pub type DiffEventListenerRef = Arc<dyn DiffEventListener>;

pub struct NullDiffEventListener {}

impl DiffEventListener for NullDiffEventListener {
    fn after_diff_executed(&self, _diff: Diff) {}
}
