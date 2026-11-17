//! Windows üzerinde bağlı disk aygıtlarını yalnızca PnP özelliklerinden listeler.
//!
//! Bu modül hiçbir fiziksel disk/volume tanıtıcısı açmaz. Dosya sistemi,
//! bölüm veya sektör okumaz; aygıta ya da Windows politikasına yazmaz.
//! Benzersiz aygıt kimlikleri, yollar ve seri numaraları bu modele alınmaz.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryState {
    NotScanned,
    // Windows disk taraması desteklenir; bu durum yalnızca diğer platformlarda üretilir.
    #[cfg_attr(target_os = "windows", allow(dead_code))]
    Unsupported,
    NoCandidates,
    CandidatesFound,
    Unknown,
}

/// PnP aygıt düğümünün kendi üst düğümünden ayrılabilme işareti.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PnpNodeRemovability {
    RemovableFromParent,
    NotMarkedRemovableFromParent,
    Unknown,
}

/// Windows'un PnP aygıt örneği için bildirdiği çıkarma politikası.
/// Bu değer tek başına takılı medyanın türünü veya yazılabilirliğini kanıtlamaz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceRemovalPolicy {
    NoRemovalExpected,
    OrderlyRemovalExpected,
    SurpriseRemovalExpected,
    Unknown,
}

#[cfg(any(target_os = "windows", test))]
fn pnp_node_removability_from_capabilities(capabilities: Option<u32>) -> PnpNodeRemovability {
    match capabilities {
        Some(flags) if flags & 0x0000_0004 != 0 => PnpNodeRemovability::RemovableFromParent,
        Some(_) => PnpNodeRemovability::NotMarkedRemovableFromParent,
        None => PnpNodeRemovability::Unknown,
    }
}

#[cfg(any(target_os = "windows", test))]
fn device_removal_policy_from_value(value: Option<u32>) -> DeviceRemovalPolicy {
    match value {
        Some(1) => DeviceRemovalPolicy::NoRemovalExpected,
        Some(2) => DeviceRemovalPolicy::OrderlyRemovalExpected,
        Some(3) => DeviceRemovalPolicy::SurpriseRemovalExpected,
        _ => DeviceRemovalPolicy::Unknown,
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct DiskCandidate {
    /// Yalnızca ekrandaki geçici satır numarasıdır; kalıcı aygıt kimliği değildir.
    pub row: usize,
    /// Windows dost ad/aygıt açıklaması; yerel ekranda gösterilir, kaydedilmez.
    pub display_name: String,
    /// PnP düğümünün üst düğümünden ayrılma işareti.
    pub node_removability: PnpNodeRemovability,
    /// Windows'un ayrı olarak bildirdiği aygıt çıkarma politikası.
    pub removal_policy: DeviceRemovalPolicy,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceInventory {
    pub state: InventoryState,
    pub candidates: Vec<DiskCandidate>,
}

impl DeviceInventory {
    pub fn not_scanned() -> Self {
        Self {
            state: InventoryState::NotScanned,
            candidates: Vec::new(),
        }
    }

    pub fn discover_present_disks() -> Self {
        #[cfg(target_os = "windows")]
        {
            windows::discover_present_disks()
        }

        #[cfg(not(target_os = "windows"))]
        {
            Self {
                state: InventoryState::Unsupported,
                candidates: Vec::new(),
            }
        }
    }

    #[cfg(any(target_os = "windows", test))]
    fn from_provider(provider: &impl PresentDiskProvider) -> Self {
        match provider.enumerate_present() {
            Ok(items) if items.is_empty() => Self {
                state: InventoryState::NoCandidates,
                candidates: Vec::new(),
            },
            Ok(items) => {
                let candidates = items
                    .into_iter()
                    .enumerate()
                    .map(|(row, item)| DiskCandidate {
                        row,
                        display_name: item.display_name,
                        node_removability: item.node_removability,
                        removal_policy: item.removal_policy,
                    })
                    .collect();

                Self {
                    state: InventoryState::CandidatesFound,
                    candidates,
                }
            }
            Err(_) => Self {
                state: InventoryState::Unknown,
                candidates: Vec::new(),
            },
        }
    }

    pub fn state_label(&self) -> &'static str {
        match self.state {
            InventoryState::NotScanned => "Henüz tarama yapılmadı",
            InventoryState::Unsupported => "Bu işletim sisteminde desteklenmiyor",
            InventoryState::NoCandidates => "Bağlı disk aygıtı bulunamadı",
            InventoryState::CandidatesFound => "Bağlı disk aygıtları listelendi",
            InventoryState::Unknown => "Aygıt listesi alınamadı",
        }
    }
}

#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Copy, PartialEq, Eq)]
enum EnumerationError {
    Failed,
}

#[cfg(any(target_os = "windows", test))]
struct ProviderCandidate {
    display_name: String,
    node_removability: PnpNodeRemovability,
    removal_policy: DeviceRemovalPolicy,
}

#[cfg(any(target_os = "windows", test))]
trait PresentDiskProvider {
    fn enumerate_present(&self) -> Result<Vec<ProviderCandidate>, EnumerationError>;
}

#[cfg(target_os = "windows")]
mod windows {
    use super::{
        DeviceInventory, DeviceRemovalPolicy, EnumerationError, PnpNodeRemovability,
        PresentDiskProvider, ProviderCandidate, device_removal_policy_from_value,
        pnp_node_removability_from_capabilities,
    };
    use std::{ffi::c_void, mem::size_of, ptr};

    const DIGCF_PRESENT: u32 = 0x0000_0002;
    const SPDRP_DEVICEDESC: u32 = 0x0000_0000;
    const SPDRP_FRIENDLYNAME: u32 = 0x0000_000C;
    const SPDRP_CAPABILITIES: u32 = 0x0000_000F;
    const SPDRP_REMOVAL_POLICY: u32 = 0x0000_001F;
    const REG_SZ: u32 = 1;
    const REG_EXPAND_SZ: u32 = 2;
    const REG_DWORD: u32 = 4;
    const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: u32 = 1;
    const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: u32 = 2;
    const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: u32 = 3;
    const ERROR_NO_MORE_ITEMS: u32 = 259;
    const MAX_PRESENT_DISK_DEVICES: u32 = 256;

    type DeviceInfoSet = *mut c_void;

    #[repr(C)]
    struct Guid {
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    }

    #[repr(C)]
    struct SpDevInfoData {
        cb_size: u32,
        class_guid: Guid,
        dev_inst: u32,
        reserved: usize,
    }

    const GUID_DEVCLASS_DISKDRIVE: Guid = Guid {
        data1: 0x4d36_e967,
        data2: 0xe325,
        data3: 0x11ce,
        data4: [0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18],
    };

    #[link(name = "setupapi")]
    unsafe extern "system" {
        fn SetupDiGetClassDevsW(
            class_guid: *const Guid,
            enumerator: *const u16,
            hwnd_parent: *mut c_void,
            flags: u32,
        ) -> DeviceInfoSet;

        fn SetupDiEnumDeviceInfo(
            device_info_set: DeviceInfoSet,
            member_index: u32,
            device_info_data: *mut SpDevInfoData,
        ) -> i32;

        fn SetupDiGetDeviceRegistryPropertyW(
            device_info_set: DeviceInfoSet,
            device_info_data: *mut SpDevInfoData,
            property: u32,
            property_reg_data_type: *mut u32,
            property_buffer: *mut u8,
            property_buffer_size: u32,
            required_size: *mut u32,
        ) -> i32;

        fn SetupDiDestroyDeviceInfoList(device_info_set: DeviceInfoSet) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLastError() -> u32;
    }

    fn empty_device_info_data() -> SpDevInfoData {
        SpDevInfoData {
            cb_size: size_of::<SpDevInfoData>() as u32,
            class_guid: Guid {
                data1: 0,
                data2: 0,
                data3: 0,
                data4: [0; 8],
            },
            dev_inst: 0,
            reserved: 0,
        }
    }

    fn read_string_property(
        device_info_set: DeviceInfoSet,
        device_info_data: &mut SpDevInfoData,
        property: u32,
    ) -> Option<String> {
        let mut buffer = [0u16; 512];
        let mut data_type = 0u32;
        let mut required_size = 0u32;

        let success = unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                device_info_data,
                property,
                &mut data_type,
                buffer.as_mut_ptr().cast::<u8>(),
                (size_of::<u16>() * buffer.len()) as u32,
                &mut required_size,
            )
        };

        if success == 0 || (data_type != REG_SZ && data_type != REG_EXPAND_SZ) {
            return None;
        }

        let end = buffer
            .iter()
            .position(|unit| *unit == 0)
            .unwrap_or(buffer.len());
        let value = String::from_utf16_lossy(&buffer[..end]);
        let value = value
            .chars()
            .filter(|character| !character.is_control())
            .take(128)
            .collect::<String>();
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");

        (!value.is_empty()).then_some(value)
    }

    fn read_dword_property(
        device_info_set: DeviceInfoSet,
        device_info_data: &mut SpDevInfoData,
        property: u32,
    ) -> Option<u32> {
        let mut buffer = [0u8; size_of::<u32>()];
        let mut data_type = 0u32;
        let mut required_size = 0u32;

        let success = unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                device_info_data,
                property,
                &mut data_type,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                &mut required_size,
            )
        };

        if success == 0 || data_type != REG_DWORD || required_size != size_of::<u32>() as u32 {
            return None;
        }

        Some(u32::from_le_bytes(buffer))
    }

    fn enumerate_device_set(
        device_info_set: DeviceInfoSet,
    ) -> Result<Vec<ProviderCandidate>, EnumerationError> {
        let mut candidates: Vec<ProviderCandidate> = Vec::new();

        for member_index in 0..MAX_PRESENT_DISK_DEVICES {
            let mut device_info_data = empty_device_info_data();
            let found = unsafe {
                SetupDiEnumDeviceInfo(device_info_set, member_index, &mut device_info_data)
            };

            if found == 0 {
                let error = unsafe { GetLastError() };
                if error == ERROR_NO_MORE_ITEMS {
                    break;
                }
                return Err(EnumerationError::Failed);
            }

            let display_name =
                read_string_property(device_info_set, &mut device_info_data, SPDRP_FRIENDLYNAME)
                    .or_else(|| {
                        read_string_property(
                            device_info_set,
                            &mut device_info_data,
                            SPDRP_DEVICEDESC,
                        )
                    })
                    .unwrap_or_else(|| "Disk aygıtı (adı alınamadı)".to_owned());

            let node_capabilities =
                read_dword_property(device_info_set, &mut device_info_data, SPDRP_CAPABILITIES);
            let node_removability =
                pnp_node_removability_from_capabilities(node_capabilities);

            let removal_policy_value =
                read_dword_property(device_info_set, &mut device_info_data, SPDRP_REMOVAL_POLICY);
            let removal_policy = device_removal_policy_from_value(removal_policy_value);

            candidates.push(ProviderCandidate {
                display_name,
                node_removability,
                removal_policy,
            });
        }

        if candidates.len() == MAX_PRESENT_DISK_DEVICES as usize {
            return Err(EnumerationError::Failed);
        }

        Ok(candidates)
    }

    struct WindowsDiskProvider;

    impl PresentDiskProvider for WindowsDiskProvider {
        fn enumerate_present(&self) -> Result<Vec<ProviderCandidate>, EnumerationError> {
            let device_info_set = unsafe {
                SetupDiGetClassDevsW(
                    &GUID_DEVCLASS_DISKDRIVE,
                    ptr::null(),
                    ptr::null_mut(),
                    DIGCF_PRESENT,
                )
            };

            if device_info_set.is_null() || device_info_set == (-1isize as DeviceInfoSet) {
                return Err(EnumerationError::Failed);
            }

            let result = enumerate_device_set(device_info_set);
            unsafe {
                SetupDiDestroyDeviceInfoList(device_info_set);
            }
            result
        }
    }

    pub(super) fn discover_present_disks() -> DeviceInventory {
        DeviceInventory::from_provider(&WindowsDiskProvider)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeProvider {
        result: Result<Vec<ProviderCandidate>, EnumerationError>,
    }

    impl PresentDiskProvider for FakeProvider {
        fn enumerate_present(&self) -> Result<Vec<ProviderCandidate>, EnumerationError> {
            match &self.result {
                Ok(items) => Ok(items
                    .iter()
                    .map(|item| ProviderCandidate {
                        display_name: item.display_name.clone(),
                        node_removability: item.node_removability,
                        removal_policy: item.removal_policy,
                    })
                    .collect()),
                Err(error) => Err(*error),
            }
        }
    }

    fn candidate(
        name: &str,
        node_removability: PnpNodeRemovability,
        removal_policy: DeviceRemovalPolicy,
    ) -> ProviderCandidate {
        ProviderCandidate {
            display_name: name.to_owned(),
            node_removability,
            removal_policy,
        }
    }

    #[test]
    fn pnp_node_removability_is_not_conflated_with_media_removability() {
        assert_eq!(
            pnp_node_removability_from_capabilities(Some(0x0000_0004)),
            PnpNodeRemovability::RemovableFromParent
        );
        assert_eq!(
            pnp_node_removability_from_capabilities(Some(0)),
            PnpNodeRemovability::NotMarkedRemovableFromParent
        );
        assert_eq!(
            pnp_node_removability_from_capabilities(None),
            PnpNodeRemovability::Unknown
        );
    }

    #[test]
    fn windows_removal_policy_values_are_mapped_independently() {
        assert_eq!(
            device_removal_policy_from_value(Some(CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL)),
            DeviceRemovalPolicy::NoRemovalExpected
        );
        assert_eq!(
            device_removal_policy_from_value(Some(CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL)),
            DeviceRemovalPolicy::OrderlyRemovalExpected
        );
        assert_eq!(
            device_removal_policy_from_value(Some(CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL)),
            DeviceRemovalPolicy::SurpriseRemovalExpected
        );
        assert_eq!(
            device_removal_policy_from_value(Some(99)),
            DeviceRemovalPolicy::Unknown
        );
        assert_eq!(
            device_removal_policy_from_value(None),
            DeviceRemovalPolicy::Unknown
        );
    }

    #[test]
    fn not_scanned_state_is_distinct() {
        let inventory = DeviceInventory::not_scanned();
        assert_eq!(inventory.state, InventoryState::NotScanned);
        assert!(inventory.candidates.is_empty());
    }

    #[test]
    fn empty_inventory_is_not_reported_as_unsupported() {
        let provider = FakeProvider {
            result: Ok(Vec::new()),
        };
        let inventory = DeviceInventory::from_provider(&provider);

        assert_eq!(inventory.state, InventoryState::NoCandidates);
        assert!(inventory.candidates.is_empty());
    }

    #[test]
    fn candidates_keep_temporary_rows_and_independent_pnp_signals() {
        let provider = FakeProvider {
            result: Ok(vec![
                candidate(
                    "USB bellek",
                    PnpNodeRemovability::NotMarkedRemovableFromParent,
                    DeviceRemovalPolicy::SurpriseRemovalExpected,
                ),
                candidate(
                    "SD kart",
                    PnpNodeRemovability::RemovableFromParent,
                    DeviceRemovalPolicy::OrderlyRemovalExpected,
                ),
            ]),
        };
        let inventory = DeviceInventory::from_provider(&provider);

        assert_eq!(inventory.state, InventoryState::CandidatesFound);
        assert_eq!(inventory.candidates.len(), 2);
        assert_eq!(inventory.candidates[0].row, 0);
        assert_eq!(inventory.candidates[0].display_name, "USB bellek");
        assert_eq!(
            inventory.candidates[0].node_removability,
            PnpNodeRemovability::NotMarkedRemovableFromParent
        );
        assert_eq!(
            inventory.candidates[0].removal_policy,
            DeviceRemovalPolicy::SurpriseRemovalExpected
        );
        assert_eq!(
            inventory.candidates[1].node_removability,
            PnpNodeRemovability::RemovableFromParent
        );
        assert_eq!(
            inventory.candidates[1].removal_policy,
            DeviceRemovalPolicy::OrderlyRemovalExpected
        );
    }

    #[test]
    fn enumeration_errors_fail_closed_as_unknown() {
        let failed = FakeProvider {
            result: Err(EnumerationError::Failed),
        };

        assert_eq!(
            DeviceInventory::from_provider(&failed).state,
            InventoryState::Unknown
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_present_disk_inventory_can_be_queried_without_opening_a_disk() {
        let inventory = DeviceInventory::discover_present_disks();
        assert_ne!(inventory.state, InventoryState::Unknown);
        assert_ne!(inventory.state, InventoryState::NotScanned);
        assert!(inventory.candidates.iter().all(|candidate| candidate.row < 256));
    }
}
