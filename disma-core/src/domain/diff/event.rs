use std::sync::Arc;

use super::base::EntityChange;

pub trait DiffEventListener {
    fn before_change_executed(&self, change: EntityChange);
    fn after_change_executed(&self, change: EntityChange);
}
pub type DiffEventListenerRef = Arc<dyn DiffEventListener>;

pub struct NullDiffEventListener {}

impl DiffEventListener for NullDiffEventListener {
    fn before_change_executed(&self, _: EntityChange) {}
    fn after_change_executed(&self, _: EntityChange) {}
}
