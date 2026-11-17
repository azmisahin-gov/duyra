//! Windows üzerinde bağlı disk aygıtlarını yalnızca PnP özelliklerinden listeler.
//!
//! Bu modül hiçbir fiziksel disk/volume tanıtıcısı açmaz. Dosya sistemi,
//! bölüm veya sektör okumaz; aygıta ya da Windows politikasına yazmaz.
//! Benzersiz aygıt kimlikleri, yollar ve seri numaraları bu modele alınmaz.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryState {
    NotScanned,
    Unsupported,
    NoCandidates,
    CandidatesFound,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PnpRemovability {
    ReportedRemovable,
    NotConfirmed,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DiskCandidate {
    /// Yalnızca ekrandaki geçici satır numarasıdır; kalıcı aygıt kimliği değildir.
    pub row: usize,
    /// Windows dost ad/aygıt açıklaması; yerel ekranda gösterilir, kaydedilmez.
    pub display_name: String,
    /// PnP devnode yeteneğidir; çıkarılabilir disk politikası kapsamını kanıtlamaz.
    pub removability: PnpRemovability,
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
                        removability: item.removability,
                    })
                    .collect();

                Self {
                    state: InventoryState::CandidatesFound,
                    candidates,
                }
            }
            Err(EnumerationError::Unsupported) => Self {
                state: InventoryState::Unsupported,
                candidates: Vec::new(),
            },
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
    Unsupported,
    Failed,
}

#[cfg(any(target_os = "windows", test))]
struct ProviderCandidate {
    display_name: String,
    removability: PnpRemovability,
}

#[cfg(any(target_os = "windows", test))]
trait PresentDiskProvider {
    fn enumerate_present(&self) -> Result<Vec<ProviderCandidate>, EnumerationError>;
}

#[cfg(target_os = "windows")]
mod windows {
    use super::{
        DeviceInventory, EnumerationError, PnpRemovability, PresentDiskProvider, ProviderCandidate,
    };
    use std::{ffi::c_void, mem::size_of, ptr};

    const DIGCF_PRESENT: u32 = 0x0000_0002;
    const SPDRP_DEVICEDESC: u32 = 0x0000_0000;
    const SPDRP_FRIENDLYNAME: u32 = 0x0000_000C;
    const SPDRP_CAPABILITIES: u32 = 0x0000_000F;
    const REG_SZ: u32 = 1;
    const REG_EXPAND_SZ: u32 = 2;
    const REG_DWORD: u32 = 4;
    const CM_DEVCAP_REMOVABLE: u32 = 0x0000_0004;
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

    fn read_capabilities(
        device_info_set: DeviceInfoSet,
        device_info_data: &mut SpDevInfoData,
    ) -> Option<u32> {
        let mut buffer = [0u8; size_of::<u32>()];
        let mut data_type = 0u32;
        let mut required_size = 0u32;

        let success = unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                device_info_data,
                SPDRP_CAPABILITIES,
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

            let display_name = read_string_property(
                device_info_set,
                &mut device_info_data,
                SPDRP_FRIENDLYNAME,
            )
            .or_else(|| {
                read_string_property(
                    device_info_set,
                    &mut device_info_data,
                    SPDRP_DEVICEDESC,
                )
            })
            .unwrap_or_else(|| "Disk aygıtı (adı alınamadı)".to_owned());

            let removability = match read_capabilities(device_info_set, &mut device_info_data) {
                Some(capabilities) if capabilities & CM_DEVCAP_REMOVABLE != 0 => {
                    PnpRemovability::ReportedRemovable
                }
                _ => PnpRemovability::NotConfirmed,
            };

            candidates.push(ProviderCandidate {
                display_name,
                removability,
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
                        removability: item.removability,
                    })
                    .collect()),
                Err(error) => Err(*error),
            }
        }
    }

    fn candidate(name: &str, removability: PnpRemovability) -> ProviderCandidate {
        ProviderCandidate {
            display_name: name.to_owned(),
            removability,
        }
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
    fn candidates_keep_temporary_rows_and_display_names() {
        let provider = FakeProvider {
            result: Ok(vec![
                candidate("USB bellek", PnpRemovability::ReportedRemovable),
                candidate("USB disk", PnpRemovability::NotConfirmed),
            ]),
        };
        let inventory = DeviceInventory::from_provider(&provider);

        assert_eq!(inventory.state, InventoryState::CandidatesFound);
        assert_eq!(inventory.candidates.len(), 2);
        assert_eq!(inventory.candidates[0].row, 0);
        assert_eq!(inventory.candidates[0].display_name, "USB bellek");
        assert_eq!(
            inventory.candidates[0].removability,
            PnpRemovability::ReportedRemovable
        );
        assert_eq!(
            inventory.candidates[1].removability,
            PnpRemovability::NotConfirmed
        );
    }

    #[test]
    fn enumeration_errors_fail_closed_as_unknown_or_unsupported() {
        let failed = FakeProvider {
            result: Err(EnumerationError::Failed),
        };
        let unsupported = FakeProvider {
            result: Err(EnumerationError::Unsupported),
        };

        assert_eq!(
            DeviceInventory::from_provider(&failed).state,
            InventoryState::Unknown
        );
        assert_eq!(
            DeviceInventory::from_provider(&unsupported).state,
            InventoryState::Unsupported
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
