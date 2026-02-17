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

#[derive(Debug)]
pub enum RetrySafety {
    Safe,
    Unsafe,
}

pub fn Classify(diff: &SnapshotDiff) -> Classification {
    let mut reasons = Vec::new();
    if diff.lamports_changed {
        reasons.push("Lamports changed".into());
    }
    if diff.owner_changed {
        reasons.push("Owner changed".into());
    }
    if diff.executable_changed {
        reasons.push("Executable flag changed".into());
    }
    if diff.data_len_changed {
        reasons.push("Account data size changed".into());
    }

    let safety = if reasons.is_empty() {
        RetrySafety::Safe
    } else {
        RetrySafety::Unsafe
    };

    Classification { safety, reasons }
}

#[derive(Debug)]
pub struct Classification {
    pub safety: RetrySafety,
    pub reasons: Vec<String>,
}