use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    Create(ChangeEntity, ChangeEntityName),
    Delete(ChangeEntity, ChangeEntityName),
    Update(ChangeEntity, ChangeEntityName),
}

type ChangeEntityName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeEntity {
    Role,
    Category,
    Channel,
}

#[derive(Debug, PartialEq)]
pub enum ChangeEvent {
    Success(Change),
    Error(Change, String),
}

#[cfg_attr(test, mock_it::mock_it)]
pub trait ChangeEventListener {
    fn handle(&self, event: ChangeEvent);
}
pub type ChangeEventListenerRef = Arc<dyn ChangeEventListener>;
