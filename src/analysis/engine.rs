use crate::models::types::AccountSnapshot;

#[derive(Debug)]
pub struct SnapshotDiff {
    pub lamports_changed: bool,
    pub owner_changed: bool,
    pub executable_changed: bool,
    pub data_len_changed: bool,
}

impl SnapshotDiff {
    pub fn diff(before: &AccountSnapshot, after: &AccountSnapshot) -> Self {
        Self {
            lamports_changed: before.lamports != after.lamports,
            owner_changed: before.owner != after.owner,
            executable_changed: before.executable != after.executable,
            data_len_changed: before.data_len != after.data_len,
        }
    }
}
