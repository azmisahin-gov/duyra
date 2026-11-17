//! Kaynak aygıtı keşfi için gerçek sistem erişimi içermeyen durum modeli.
//!
//! Bu modül yalnızca test derlemesine eklenir. PnP/SetupAPI, disk tanıtıcısı,
//! DeviceIoControl, BLKROGET, mount veya Registry çağrısı yapmaz.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeviceClass {
    Removable,
    Fixed,
    Other,
    Unknown,
}

#[derive(Clone, PartialEq, Eq)]
struct ProviderCandidate {
    display_name: String,
    class: DeviceClass,
}

trait PresentDeviceProvider {
    fn enumerate_present(&self) -> Result<Vec<ProviderCandidate>, EnumerationError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnumerationError {
    Unsupported,
    AccessDenied,
    Unavailable,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiscoveryState {
    Unsupported,
    NoCandidates,
    CandidatesFound,
    Unknown,
}

#[derive(Clone, PartialEq, Eq)]
struct DeviceCandidate {
    /// Yalnızca o envanter görüntüsünde geçerli geçici satır numarasıdır.
    /// Kalıcı kimlik, aygıt yolu, PnP instance ID veya seri numarası değildir.
    row: usize,
    display_name: String,
    class: DeviceClass,
}

#[derive(Clone, PartialEq, Eq)]
struct DeviceInventory {
    state: DiscoveryState,
    candidates: Vec<DeviceCandidate>,
    diagnostic_code: &'static str,
}

impl DeviceInventory {
    fn enumerate(provider: &impl PresentDeviceProvider) -> Self {
        match provider.enumerate_present() {
            Ok(items) if items.is_empty() => Self {
                state: DiscoveryState::NoCandidates,
                candidates: Vec::new(),
                diagnostic_code: "device_inventory_no_candidates",
            },
            Ok(items) => {
                let candidates = items
                    .into_iter()
                    .enumerate()
                    .map(|(row, item)| DeviceCandidate {
                        row,
                        display_name: item.display_name,
                        class: item.class,
                    })
                    .collect();

                Self {
                    state: DiscoveryState::CandidatesFound,
                    candidates,
                    diagnostic_code: "device_inventory_candidates_found",
                }
            }
            Err(EnumerationError::Unsupported) => Self {
                state: DiscoveryState::Unsupported,
                candidates: Vec::new(),
                diagnostic_code: "device_inventory_unsupported",
            },
            Err(_) => Self {
                state: DiscoveryState::Unknown,
                candidates: Vec::new(),
                diagnostic_code: "device_inventory_unknown",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReadOnlyObservation {
    ReportedReadOnly,
    ReportedWritable,
    Unsupported,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WindowsWriteabilityResult {
    WriteProtected,
    Writable,
    Unsupported,
    AccessDenied,
    Failed,
}

fn observe_windows_writeability(result: WindowsWriteabilityResult) -> ReadOnlyObservation {
    match result {
        WindowsWriteabilityResult::WriteProtected => ReadOnlyObservation::ReportedReadOnly,
        WindowsWriteabilityResult::Writable => ReadOnlyObservation::ReportedWritable,
        WindowsWriteabilityResult::Unsupported => ReadOnlyObservation::Unsupported,
        WindowsWriteabilityResult::AccessDenied | WindowsWriteabilityResult::Failed => {
            ReadOnlyObservation::Unknown
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LinuxBlockReadOnlyResult {
    ReadOnly,
    Writable,
    Unsupported,
    Failed,
}

fn observe_linux_block_read_only(result: LinuxBlockReadOnlyResult) -> ReadOnlyObservation {
    match result {
        LinuxBlockReadOnlyResult::ReadOnly => ReadOnlyObservation::ReportedReadOnly,
        LinuxBlockReadOnlyResult::Writable => ReadOnlyObservation::ReportedWritable,
        LinuxBlockReadOnlyResult::Unsupported => ReadOnlyObservation::Unsupported,
        LinuxBlockReadOnlyResult::Failed => ReadOnlyObservation::Unknown,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FilesystemMountObservation {
    NotMounted,
    MountedReadOnly,
    MountedWritable,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeProvider {
        result: Result<Vec<ProviderCandidate>, EnumerationError>,
    }

    impl PresentDeviceProvider for FakeProvider {
        fn enumerate_present(&self) -> Result<Vec<ProviderCandidate>, EnumerationError> {
            self.result.clone()
        }
    }

    fn candidate(name: &str, class: DeviceClass) -> ProviderCandidate {
        ProviderCandidate {
            display_name: name.to_owned(),
            class,
        }
    }

    #[test]
    fn empty_inventory_is_not_reported_as_unsupported() {
        let provider = FakeProvider {
            result: Ok(Vec::new()),
        };
        let inventory = DeviceInventory::enumerate(&provider);

        assert_eq!(inventory.state, DiscoveryState::NoCandidates);
        assert!(inventory.candidates.is_empty());
        assert_eq!(inventory.diagnostic_code, "device_inventory_no_candidates");
    }

    #[test]
    fn present_candidates_are_listed_without_automatic_selection() {
        let provider = FakeProvider {
            result: Ok(vec![
                candidate("USB bellek", DeviceClass::Removable),
                candidate("USB disk", DeviceClass::Fixed),
                candidate("Diğer aygıt", DeviceClass::Other),
            ]),
        };
        let inventory = DeviceInventory::enumerate(&provider);

        assert_eq!(inventory.state, DiscoveryState::CandidatesFound);
        assert_eq!(inventory.candidates.len(), 3);
        assert_eq!(inventory.candidates[0].display_name, "USB bellek");
        assert_eq!(inventory.candidates[0].row, 0);
        assert_eq!(inventory.candidates[1].row, 1);
        assert_eq!(inventory.candidates[0].class, DeviceClass::Removable);
        assert_eq!(inventory.candidates[1].class, DeviceClass::Fixed);
        assert_eq!(inventory.candidates[2].class, DeviceClass::Other);
    }

    #[test]
    fn unknown_device_class_is_preserved_instead_of_guessed() {
        let provider = FakeProvider {
            result: Ok(vec![candidate("Bilinmeyen aygıt", DeviceClass::Unknown)]),
        };
        let inventory = DeviceInventory::enumerate(&provider);

        assert_eq!(inventory.state, DiscoveryState::CandidatesFound);
        assert_eq!(inventory.candidates[0].class, DeviceClass::Unknown);
    }

    #[test]
    fn unsupported_enumeration_is_not_reported_as_an_empty_inventory() {
        let provider = FakeProvider {
            result: Err(EnumerationError::Unsupported),
        };
        let inventory = DeviceInventory::enumerate(&provider);

        assert_eq!(inventory.state, DiscoveryState::Unsupported);
        assert!(inventory.candidates.is_empty());
        assert_eq!(inventory.diagnostic_code, "device_inventory_unsupported");
    }

    #[test]
    fn enumeration_errors_fail_closed_as_unknown() {
        for error in [
            EnumerationError::AccessDenied,
            EnumerationError::Unavailable,
            EnumerationError::Failed,
        ] {
            let provider = FakeProvider { result: Err(error) };
            let inventory = DeviceInventory::enumerate(&provider);

            assert_eq!(inventory.state, DiscoveryState::Unknown);
            assert!(inventory.candidates.is_empty());
            assert_eq!(inventory.diagnostic_code, "device_inventory_unknown");
        }
    }

    #[test]
    fn windows_write_protected_result_is_reported_as_observation_only() {
        assert_eq!(
            observe_windows_writeability(WindowsWriteabilityResult::WriteProtected),
            ReadOnlyObservation::ReportedReadOnly
        );
    }

    #[test]
    fn windows_access_or_query_errors_never_become_read_only() {
        assert_eq!(
            observe_windows_writeability(WindowsWriteabilityResult::AccessDenied),
            ReadOnlyObservation::Unknown
        );
        assert_eq!(
            observe_windows_writeability(WindowsWriteabilityResult::Failed),
            ReadOnlyObservation::Unknown
        );
        assert_eq!(
            observe_windows_writeability(WindowsWriteabilityResult::Unsupported),
            ReadOnlyObservation::Unsupported
        );
    }

    #[test]
    fn linux_block_flag_is_mapped_separately_from_mount_state() {
        let block_state = observe_linux_block_read_only(LinuxBlockReadOnlyResult::ReadOnly);
        let mount_state = FilesystemMountObservation::MountedWritable;

        assert_eq!(block_state, ReadOnlyObservation::ReportedReadOnly);
        assert_eq!(mount_state, FilesystemMountObservation::MountedWritable);
    }

    #[test]
    fn unsupported_and_failed_block_queries_remain_distinct() {
        assert_eq!(
            observe_linux_block_read_only(LinuxBlockReadOnlyResult::Unsupported),
            ReadOnlyObservation::Unsupported
        );
        assert_eq!(
            observe_linux_block_read_only(LinuxBlockReadOnlyResult::Failed),
            ReadOnlyObservation::Unknown
        );
    }

    #[test]
    fn writable_observations_are_not_converted_to_read_only() {
        assert_eq!(
            observe_windows_writeability(WindowsWriteabilityResult::Writable),
            ReadOnlyObservation::ReportedWritable
        );
        assert_eq!(
            observe_linux_block_read_only(LinuxBlockReadOnlyResult::Writable),
            ReadOnlyObservation::ReportedWritable
        );
    }

    #[test]
    fn mount_state_is_retained_as_an_independent_observation() {
        let states = [
            FilesystemMountObservation::NotMounted,
            FilesystemMountObservation::MountedReadOnly,
            FilesystemMountObservation::MountedWritable,
            FilesystemMountObservation::Unknown,
        ];

        assert_eq!(states.len(), 4);
        assert_ne!(
            FilesystemMountObservation::MountedReadOnly,
            FilesystemMountObservation::MountedWritable
        );
    }
}
