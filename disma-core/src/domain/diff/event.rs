use std::sync::Arc;

use super::base::EntityChange;

pub trait DiffEventListener {
    fn after_change_executed(&self, change: EntityChange);
}
pub type DiffEventListenerRef = Arc<dyn DiffEventListener>;

pub struct NullDiffEventListener {}

impl DiffEventListener for NullDiffEventListener {
    fn after_change_executed(&self, _: EntityChange) {}
}
