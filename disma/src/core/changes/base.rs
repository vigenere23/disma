use crate::core::diffs::Diff;

#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    Create(ChangeEntity, ChangeEntityName),
    Delete(ChangeEntity, ChangeEntityName),
    Update(ChangeEntity, ChangeEntityName, Vec<Diff>),
}

type ChangeEntityName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeEntity {
    Role,
    Category,
    Channel,
}
