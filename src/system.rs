//! İşletim sisteminin mevcut grafik bağdaştırıcılarını listeler.
//! Bu liste, uygulamanın o anda çizim yaptığı GPU bilgisinden ayrıdır.

#[cfg(target_os = "windows")]
mod windows {
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::ptr;

    const DIGCF_PRESENT: u32 = 0x0000_0002;
    const SPDRP_DEVICEDESC: u32 = 0x0000_0000;
    const SPDRP_FRIENDLYNAME: u32 = 0x0000_000C;

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

    const GUID_DEVCLASS_DISPLAY: Guid = Guid {
        data1: 0x4d36_e968,
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
        ) -> *mut c_void;

        fn SetupDiEnumDeviceInfo(
            device_info_set: *mut c_void,
            member_index: u32,
            device_info_data: *mut SpDevInfoData,
        ) -> i32;

        fn SetupDiGetDeviceRegistryPropertyW(
            device_info_set: *mut c_void,
            device_info_data: *mut SpDevInfoData,
            property: u32,
            property_reg_data_type: *mut u32,
            property_buffer: *mut u8,
            property_buffer_size: u32,
            required_size: *mut u32,
        ) -> i32;

        fn SetupDiDestroyDeviceInfoList(device_info_set: *mut c_void) -> i32;
    }

    fn read_device_name(
        device_info_set: *mut c_void,
        device_info_data: &mut SpDevInfoData,
        property: u32,
    ) -> Option<String> {
        let mut buffer = [0u16; 512];
        let mut required_size = 0u32;

        let success = unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                device_info_data,
                property,
                ptr::null_mut(),
                buffer.as_mut_ptr().cast::<u8>(),
                (size_of::<u16>() * buffer.len()) as u32,
                &mut required_size,
            )
        };

        if success == 0 {
            return None;
        }

        let end = buffer.iter().position(|unit| *unit == 0).unwrap_or(buffer.len());
        let name = String::from_utf16_lossy(&buffer[..end]);
        let name = name.trim().to_owned();

        (!name.is_empty()).then_some(name)
    }

    pub fn video_adapters() -> Vec<String> {
        let device_info_set = unsafe {
            SetupDiGetClassDevsW(
                &GUID_DEVCLASS_DISPLAY,
                ptr::null(),
                ptr::null_mut(),
                DIGCF_PRESENT,
            )
        };

        if device_info_set.is_null() || device_info_set == (-1isize as *mut c_void) {
            return Vec::new();
        }

        let mut adapters = Vec::new();

        for member_index in 0..256 {
            let mut device_info_data = SpDevInfoData {
                cb_size: size_of::<SpDevInfoData>() as u32,
                class_guid: Guid {
                    data1: 0,
                    data2: 0,
                    data3: 0,
                    data4: [0; 8],
                },
                dev_inst: 0,
                reserved: 0,
            };

            let found = unsafe {
                SetupDiEnumDeviceInfo(
                    device_info_set,
                    member_index,
                    &mut device_info_data,
                )
            };

            if found == 0 {
                break;
            }

            let name = read_device_name(
                device_info_set,
                &mut device_info_data,
                SPDRP_FRIENDLYNAME,
            )
            .or_else(|| {
                read_device_name(
                    device_info_set,
                    &mut device_info_data,
                    SPDRP_DEVICEDESC,
                )
            });

            if let Some(name) = name
                && !adapters.iter().any(|existing: &String| existing.eq_ignore_ascii_case(&name))
            {
                adapters.push(name);
            }
        }

        unsafe {
            SetupDiDestroyDeviceInfoList(device_info_set);
        }

        adapters
    }
}

#[cfg(target_os = "windows")]
pub fn video_adapters() -> Vec<String> {
    windows::video_adapters()
}

#[cfg(not(target_os = "windows"))]
pub fn video_adapters() -> Vec<String> {
    Vec::new()
}
