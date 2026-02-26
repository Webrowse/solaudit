use crate::models::types::AccountSnapshot;
use serde::Serialize;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub enum RetrySafety {
    Safe,
    Unsafe,
}

pub fn classify(diff: &SnapshotDiff) -> Classification {
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

#[derive(Debug, Serialize)]
pub struct Classification {
    pub safety: RetrySafety,
    pub reasons: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub before: AccountSnapshot,
    pub after: AccountSnapshot,
    pub diff: SnapshotDiff,
    pub classification: Classification,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub simulation_logs: Vec<String>,
}

pub fn analyse(before: AccountSnapshot, after: AccountSnapshot, simulation_logs: Vec<String>) -> AnalysisResult {
    let diff = SnapshotDiff::diff(&before, &after);
    let classification = classify(&diff);

    AnalysisResult { before, after, diff, classification, simulation_logs }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    fn base_snapshot() -> AccountSnapshot {
        AccountSnapshot {
            pubkey: Pubkey::default(),
            lamports: 1_000_000,
            owner: Pubkey::default(),
            executable: false,
            data_len: 128,
            rent_epoch: 0,
        }
    }

    // ── SnapshotDiff::diff ────────────────────────────────────────────────

    #[test]
    fn diff_no_changes() {
        let s = base_snapshot();
        let d = SnapshotDiff::diff(&s, &s);
        assert!(!d.lamports_changed);
        assert!(!d.owner_changed);
        assert!(!d.executable_changed);
        assert!(!d.data_len_changed);
    }

    #[test]
    fn diff_lamports_changed() {
        let before = base_snapshot();
        let mut after = base_snapshot();
        after.lamports = 999;
        let d = SnapshotDiff::diff(&before, &after);
        assert!(d.lamports_changed);
        assert!(!d.owner_changed);
        assert!(!d.executable_changed);
        assert!(!d.data_len_changed);
    }

    #[test]
    fn diff_owner_changed() {
        let before = base_snapshot();
        let mut after = base_snapshot();
        after.owner = Pubkey::new_unique();
        let d = SnapshotDiff::diff(&before, &after);
        assert!(!d.lamports_changed);
        assert!(d.owner_changed);
    }

    #[test]
    fn diff_executable_changed() {
        let before = base_snapshot();
        let mut after = base_snapshot();
        after.executable = true;
        let d = SnapshotDiff::diff(&before, &after);
        assert!(d.executable_changed);
    }

    #[test]
    fn diff_data_len_changed() {
        let before = base_snapshot();
        let mut after = base_snapshot();
        after.data_len = 256;
        let d = SnapshotDiff::diff(&before, &after);
        assert!(d.data_len_changed);
    }

    #[test]
    fn diff_all_fields_changed() {
        let before = base_snapshot();
        let after = AccountSnapshot {
            pubkey: Pubkey::default(),
            lamports: 0,
            owner: Pubkey::new_unique(),
            executable: true,
            data_len: 0,
            rent_epoch: 99,
        };
        let d = SnapshotDiff::diff(&before, &after);
        assert!(d.lamports_changed);
        assert!(d.owner_changed);
        assert!(d.executable_changed);
        assert!(d.data_len_changed);
    }

    // ── classify ─────────────────────────────────────────────────────────

    #[test]
    fn classify_no_changes_is_safe() {
        let diff = SnapshotDiff {
            lamports_changed: false,
            owner_changed: false,
            executable_changed: false,
            data_len_changed: false,
        };
        let c = classify(&diff);
        assert!(matches!(c.safety, RetrySafety::Safe));
        assert!(c.reasons.is_empty());
    }

    #[test]
    fn classify_lamports_change_is_unsafe() {
        let diff = SnapshotDiff {
            lamports_changed: true,
            owner_changed: false,
            executable_changed: false,
            data_len_changed: false,
        };
        let c = classify(&diff);
        assert!(matches!(c.safety, RetrySafety::Unsafe));
        assert_eq!(c.reasons, vec!["Lamports changed"]);
    }

    #[test]
    fn classify_owner_change_is_unsafe() {
        let diff = SnapshotDiff {
            lamports_changed: false,
            owner_changed: true,
            executable_changed: false,
            data_len_changed: false,
        };
        let c = classify(&diff);
        assert!(matches!(c.safety, RetrySafety::Unsafe));
        assert_eq!(c.reasons, vec!["Owner changed"]);
    }

    #[test]
    fn classify_executable_change_is_unsafe() {
        let diff = SnapshotDiff {
            lamports_changed: false,
            owner_changed: false,
            executable_changed: true,
            data_len_changed: false,
        };
        let c = classify(&diff);
        assert!(matches!(c.safety, RetrySafety::Unsafe));
        assert_eq!(c.reasons, vec!["Executable flag changed"]);
    }

    #[test]
    fn classify_data_len_change_is_unsafe() {
        let diff = SnapshotDiff {
            lamports_changed: false,
            owner_changed: false,
            executable_changed: false,
            data_len_changed: true,
        };
        let c = classify(&diff);
        assert!(matches!(c.safety, RetrySafety::Unsafe));
        assert_eq!(c.reasons, vec!["Account data size changed"]);
    }

    #[test]
    fn classify_all_changes_emits_all_reasons() {
        let diff = SnapshotDiff {
            lamports_changed: true,
            owner_changed: true,
            executable_changed: true,
            data_len_changed: true,
        };
        let c = classify(&diff);
        assert!(matches!(c.safety, RetrySafety::Unsafe));
        assert_eq!(c.reasons.len(), 4);
        assert!(c.reasons.contains(&"Lamports changed".to_string()));
        assert!(c.reasons.contains(&"Owner changed".to_string()));
        assert!(c.reasons.contains(&"Executable flag changed".to_string()));
        assert!(c.reasons.contains(&"Account data size changed".to_string()));
    }
}