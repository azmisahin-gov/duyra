//! Test-only model of a fail-closed write-policy transaction.
//!
//! This module is deliberately not connected to the Windows registry or the UI.
//! A future operating-system adapter must implement the compare-and-set/remove
//! contract atomically before this model can be used for real policy changes.
//! In particular, a read followed by an ordinary registry write is not atomic.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PolicySnapshot {
    value: Option<u32>,
    revision: u64,
}

trait PolicyStore {
    fn read(&mut self) -> Result<PolicySnapshot, StoreError>;

    /// Atomically installs a value only if the current snapshot still equals
    /// expected. A successful return identifies the exact version installed.
    fn compare_and_set(
        &mut self,
        expected: PolicySnapshot,
        value: u32,
    ) -> Result<PolicySnapshot, StoreError>;

    /// Atomically removes the value only if the current snapshot still equals
    /// expected. A successful return identifies the exact post-remove version.
    fn compare_and_remove(
        &mut self,
        expected: PolicySnapshot,
    ) -> Result<PolicySnapshot, StoreError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StoreError {
    Conflict(PolicySnapshot),
    Failed(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PolicyChangeReceipt {
    before: PolicySnapshot,
    applied: PolicySnapshot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ApplyError {
    ReadBefore(&'static str),
    ExistingPolicy(u32),
    Conflict(PolicySnapshot),
    InstallOutcomeUnknown(&'static str),
    InvalidInstallReceipt(PolicySnapshot),
    ReadAfterInstall(&'static str),
    InstallVerificationFailed {
        expected: PolicySnapshot,
        observed: PolicySnapshot,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RollbackError {
    ReadCurrent(&'static str),
    InvalidReceipt,
    Conflict {
        expected: PolicySnapshot,
        observed: PolicySnapshot,
    },
    RemoveOutcomeUnknown(&'static str),
    InvalidRemoveReceipt(PolicySnapshot),
    ReadAfterRemove(&'static str),
    RemoveVerificationFailed {
        expected: PolicySnapshot,
        observed: PolicySnapshot,
    },
}

const WRITE_PROTECTION_ENABLED: u32 = 1;

fn apply_write_protection<S: PolicyStore>(
    store: &mut S,
) -> Result<PolicyChangeReceipt, ApplyError> {
    let before = store.read().map_err(|error| match error {
        StoreError::Failed(message) => ApplyError::ReadBefore(message),
        StoreError::Conflict(observed) => ApplyError::Conflict(observed),
    })?;

    // Never overwrite an existing value, regardless of whether it is 0, 1,
    // or an unexpected DWORD. Its owner and intended semantics are unknown.
    if let Some(value) = before.value {
        return Err(ApplyError::ExistingPolicy(value));
    }

    let applied = match store.compare_and_set(before, WRITE_PROTECTION_ENABLED) {
        Ok(snapshot) => snapshot,
        Err(StoreError::Conflict(observed)) => {
            return Err(ApplyError::Conflict(observed));
        }
        // An OS/provider error can leave the outcome ambiguous. Do not attempt
        // a compensating write; a caller must re-read and resolve explicitly.
        Err(StoreError::Failed(message)) => {
            return Err(ApplyError::InstallOutcomeUnknown(message));
        }
    };

    if applied.value != Some(WRITE_PROTECTION_ENABLED) || applied.revision == before.revision {
        return Err(ApplyError::InvalidInstallReceipt(applied));
    }

    let observed = store.read().map_err(|error| match error {
        StoreError::Failed(message) => ApplyError::ReadAfterInstall(message),
        StoreError::Conflict(snapshot) => ApplyError::Conflict(snapshot),
    })?;

    if observed != applied {
        return Err(ApplyError::InstallVerificationFailed {
            expected: applied,
            observed,
        });
    }

    Ok(PolicyChangeReceipt { before, applied })
}

fn rollback_write_protection<S: PolicyStore>(
    store: &mut S,
    receipt: PolicyChangeReceipt,
) -> Result<PolicySnapshot, RollbackError> {
    // This first model only accepts a receipt for a value that was originally
    // absent. It must never restore over a pre-existing policy.
    if receipt.before.value.is_some()
        || receipt.applied.value != Some(WRITE_PROTECTION_ENABLED)
        || receipt.applied.revision == receipt.before.revision
    {
        return Err(RollbackError::InvalidReceipt);
    }

    let current = store.read().map_err(|error| match error {
        StoreError::Failed(message) => RollbackError::ReadCurrent(message),
        StoreError::Conflict(observed) => RollbackError::Conflict {
            expected: receipt.applied,
            observed,
        },
    })?;

    // Compare both data and revision. Even a same-value write by another
    // policy source is a conflict and must not be deleted by rollback.
    if current != receipt.applied {
        return Err(RollbackError::Conflict {
            expected: receipt.applied,
            observed: current,
        });
    }

    let removed = match store.compare_and_remove(receipt.applied) {
        Ok(snapshot) => snapshot,
        Err(StoreError::Conflict(observed)) => {
            return Err(RollbackError::Conflict {
                expected: receipt.applied,
                observed,
            });
        }
        Err(StoreError::Failed(message)) => {
            return Err(RollbackError::RemoveOutcomeUnknown(message));
        }
    };

    if removed.value != receipt.before.value || removed.revision == receipt.applied.revision {
        return Err(RollbackError::InvalidRemoveReceipt(removed));
    }

    let observed = store.read().map_err(|error| match error {
        StoreError::Failed(message) => RollbackError::ReadAfterRemove(message),
        StoreError::Conflict(snapshot) => RollbackError::RemoveVerificationFailed {
            expected: removed,
            observed: snapshot,
        },
    })?;

    if observed != removed {
        return Err(RollbackError::RemoveVerificationFailed {
            expected: removed,
            observed,
        });
    }

    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FakeStore {
        snapshot: PolicySnapshot,
        install_calls: usize,
        remove_calls: usize,
        mutate_before_install: Option<Option<u32>>,
        mutate_after_install: Option<Option<u32>>,
        mutate_before_remove: Option<Option<u32>>,
        fail_install_after_commit: bool,
        fail_remove_after_commit: bool,
        mutate_after_remove: Option<Option<u32>>,
    }

    impl FakeStore {
        fn new(value: Option<u32>) -> Self {
            Self {
                snapshot: PolicySnapshot { value, revision: 1 },
                install_calls: 0,
                remove_calls: 0,
                mutate_before_install: None,
                mutate_after_install: None,
                mutate_before_remove: None,
                fail_install_after_commit: false,
                fail_remove_after_commit: false,
                mutate_after_remove: None,
            }
        }

        fn external_set(&mut self, value: Option<u32>) {
            self.snapshot.value = value;
            self.snapshot.revision += 1;
        }
    }

    impl PolicyStore for FakeStore {
        fn read(&mut self) -> Result<PolicySnapshot, StoreError> {
            Ok(self.snapshot)
        }

        fn compare_and_set(
            &mut self,
            expected: PolicySnapshot,
            value: u32,
        ) -> Result<PolicySnapshot, StoreError> {
            self.install_calls += 1;

            if let Some(value) = self.mutate_before_install.take() {
                self.external_set(value);
            }

            if self.snapshot != expected {
                return Err(StoreError::Conflict(self.snapshot));
            }

            self.external_set(Some(value));
            let installed = self.snapshot;

            if let Some(value) = self.mutate_after_install.take() {
                self.external_set(value);
            }

            if self.fail_install_after_commit {
                return Err(StoreError::Failed("install response lost after commit"));
            }

            Ok(installed)
        }

        fn compare_and_remove(
            &mut self,
            expected: PolicySnapshot,
        ) -> Result<PolicySnapshot, StoreError> {
            self.remove_calls += 1;

            if let Some(value) = self.mutate_before_remove.take() {
                self.external_set(value);
            }

            if self.snapshot != expected {
                return Err(StoreError::Conflict(self.snapshot));
            }

            self.external_set(None);
            let removed = self.snapshot;

            if let Some(value) = self.mutate_after_remove.take() {
                self.external_set(value);
            }

            if self.fail_remove_after_commit {
                return Err(StoreError::Failed("remove response lost after commit"));
            }

            Ok(removed)
        }
    }

    #[test]
    fn pre_existing_policy_values_are_never_overwritten() {
        for value in [0, 1, 9] {
            let mut store = FakeStore::new(Some(value));

            assert_eq!(
                apply_write_protection(&mut store),
                Err(ApplyError::ExistingPolicy(value))
            );
            assert_eq!(store.install_calls, 0);
            assert_eq!(store.snapshot.value, Some(value));
        }
    }

    #[test]
    fn concurrent_change_before_install_is_a_conflict() {
        let mut store = FakeStore::new(None);
        store.mutate_before_install = Some(Some(0));

        let result = apply_write_protection(&mut store);

        assert!(matches!(result, Err(ApplyError::Conflict(_))));
        assert_eq!(store.snapshot.value, Some(0));
        assert_eq!(store.install_calls, 1);
    }

    #[test]
    fn successful_install_is_read_back_and_receipted() {
        let mut store = FakeStore::new(None);
        let receipt = apply_write_protection(&mut store).expect("install should succeed");

        assert_eq!(receipt.before.value, None);
        assert_eq!(receipt.applied.value, Some(WRITE_PROTECTION_ENABLED));
        assert_ne!(receipt.before.revision, receipt.applied.revision);
        assert_eq!(store.snapshot, receipt.applied);
    }

    #[test]
    fn rollback_removes_only_the_exact_version_installed_by_duyra() {
        let mut store = FakeStore::new(None);
        let receipt = apply_write_protection(&mut store).expect("install should succeed");

        let restored = rollback_write_protection(&mut store, receipt)
            .expect("unchanged DUYRA value should be removed");

        assert_eq!(restored.value, None);
        assert_eq!(store.snapshot, restored);
        assert_eq!(store.remove_calls, 1);
    }

    #[test]
    fn rollback_refuses_a_different_external_value() {
        let mut store = FakeStore::new(None);
        let receipt = apply_write_protection(&mut store).expect("install should succeed");
        store.external_set(Some(0));

        let result = rollback_write_protection(&mut store, receipt);

        assert!(matches!(result, Err(RollbackError::Conflict { .. })));
        assert_eq!(store.snapshot.value, Some(0));
        assert_eq!(store.remove_calls, 0);
    }

    #[test]
    fn rollback_detects_a_race_between_read_and_conditional_remove() {
        let mut store = FakeStore::new(None);
        let receipt = apply_write_protection(&mut store).expect("install should succeed");
        store.mutate_before_remove = Some(Some(0));

        let result = rollback_write_protection(&mut store, receipt);

        assert!(matches!(result, Err(RollbackError::Conflict { .. })));
        assert_eq!(store.snapshot.value, Some(0));
        assert_eq!(store.remove_calls, 1);
    }

    #[test]
    fn rollback_refuses_the_same_value_written_again_by_another_source() {
        let mut store = FakeStore::new(None);
        let receipt = apply_write_protection(&mut store).expect("install should succeed");
        store.external_set(Some(WRITE_PROTECTION_ENABLED));

        let result = rollback_write_protection(&mut store, receipt);

        assert!(matches!(result, Err(RollbackError::Conflict { .. })));
        assert_eq!(store.snapshot.value, Some(WRITE_PROTECTION_ENABLED));
        assert_eq!(store.remove_calls, 0);
    }

    #[test]
    fn install_error_with_ambiguous_commit_does_not_trigger_automatic_rollback() {
        let mut store = FakeStore::new(None);
        store.fail_install_after_commit = true;

        let result = apply_write_protection(&mut store);

        assert_eq!(
            result,
            Err(ApplyError::InstallOutcomeUnknown(
                "install response lost after commit"
            ))
        );
        assert_eq!(store.snapshot.value, Some(WRITE_PROTECTION_ENABLED));
        assert_eq!(store.remove_calls, 0);
    }

    #[test]
    fn post_install_race_is_detected_before_a_receipt_is_returned() {
        let mut store = FakeStore::new(None);
        store.mutate_after_install = Some(Some(0));

        let result = apply_write_protection(&mut store);

        assert!(matches!(
            result,
            Err(ApplyError::InstallVerificationFailed { .. })
        ));
        assert_eq!(store.snapshot.value, Some(0));
        assert_eq!(store.remove_calls, 0);
    }

    #[test]
    fn remove_error_with_ambiguous_commit_is_not_reported_as_success() {
        let mut store = FakeStore::new(None);
        let receipt = apply_write_protection(&mut store).expect("install should succeed");
        store.fail_remove_after_commit = true;

        let result = rollback_write_protection(&mut store, receipt);

        assert_eq!(
            result,
            Err(RollbackError::RemoveOutcomeUnknown(
                "remove response lost after commit"
            ))
        );
        assert_eq!(store.snapshot.value, None);
    }

    #[test]
    fn post_remove_race_is_reported_without_overwriting_the_external_value() {
        let mut store = FakeStore::new(None);
        let receipt = apply_write_protection(&mut store).expect("install should succeed");
        store.mutate_after_remove = Some(Some(0));

        let result = rollback_write_protection(&mut store, receipt);

        assert!(matches!(
            result,
            Err(RollbackError::RemoveVerificationFailed { .. })
        ));
        assert_eq!(store.snapshot.value, Some(0));
    }
}
