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

        let end = buffer
            .iter()
            .position(|unit| *unit == 0)
            .unwrap_or(buffer.len());
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
                SetupDiEnumDeviceInfo(device_info_set, member_index, &mut device_info_data)
            };

            if found == 0 {
                break;
            }

            let name = read_device_name(device_info_set, &mut device_info_data, SPDRP_FRIENDLYNAME)
                .or_else(|| {
                    read_device_name(device_info_set, &mut device_info_data, SPDRP_DEVICEDESC)
                });

            if let Some(name) = name
                && !adapters
                    .iter()
                    .any(|existing: &String| existing.eq_ignore_ascii_case(&name))
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

#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WritePolicyStatus {
    Enabled,
    Disabled,
    NotConfigured,
    UnexpectedValue(u32),
    Unknown(String),
}

#[cfg(any(target_os = "windows", test))]
fn policy_status_from_value(value: Option<u32>) -> WritePolicyStatus {
    match value {
        Some(1) => WritePolicyStatus::Enabled,
        Some(0) => WritePolicyStatus::Disabled,
        Some(value) => WritePolicyStatus::UnexpectedValue(value),
        None => WritePolicyStatus::NotConfigured,
    }
}

#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriteProtectionSupport {
    Supported,
    UnsupportedEdition(String),
    UnsupportedBuild(u32),
    Unknown,
}

#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowsWriteProtectionPreflight {
    pub product_name: Option<String>,
    pub edition_id: Option<String>,
    pub build_number: Option<u32>,
    pub process_is_elevated: Option<bool>,
    pub support: WriteProtectionSupport,
    pub computer_policy: WritePolicyStatus,
    pub read_warnings: Vec<String>,
}

#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyChangeAssessment {
    Managed,
    Reviewable,
    Unknown,
}

#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyAuthorityCheck {
    pub group_policy_applied: Option<bool>,
    pub mdm_enrollment_detected: Option<bool>,
    pub mdm_policy_present: Option<bool>,
    pub domain_joined: Option<bool>,
    pub current_policy_value_present: Option<bool>,
}

#[cfg(any(target_os = "windows", test))]
/// Politika değiştirme henüz sunulmadan önce gerekli güvenlik ön koşullarını sınıflandırır.
/// Bu kontrol yalnızca değerlendirme yapar; kayıt defterine veya sistem politikasına yazmaz.
#[cfg(any(target_os = "windows", test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyChangeReadiness {
    UnsupportedWindows,
    ManagedByOrganization,
    SourceUnknown,
    ExistingPolicyValue,
    NotElevated,
    PreconditionsMet,
}

#[cfg(any(target_os = "windows", test))]
pub fn assess_policy_change_readiness(
    status: &WindowsWriteProtectionPreflight,
    authority: &PolicyAuthorityCheck,
) -> PolicyChangeReadiness {
    if !matches!(&status.support, WriteProtectionSupport::Supported) {
        return PolicyChangeReadiness::UnsupportedWindows;
    }

    match authority.assessment() {
        PolicyChangeAssessment::Managed => {
            return PolicyChangeReadiness::ManagedByOrganization;
        }
        PolicyChangeAssessment::Unknown => {
            return PolicyChangeReadiness::SourceUnknown;
        }
        PolicyChangeAssessment::Reviewable => {}
    }

    match &status.computer_policy {
        WritePolicyStatus::NotConfigured => {}
        WritePolicyStatus::Unknown(_) => {
            return PolicyChangeReadiness::SourceUnknown;
        }
        WritePolicyStatus::Enabled
        | WritePolicyStatus::Disabled
        | WritePolicyStatus::UnexpectedValue(_) => {
            return PolicyChangeReadiness::ExistingPolicyValue;
        }
    }

    if status.process_is_elevated != Some(true) {
        return PolicyChangeReadiness::NotElevated;
    }

    PolicyChangeReadiness::PreconditionsMet
}

#[cfg(any(target_os = "windows", test))]
fn parse_rsop_policy_marker(success: bool, output: &str) -> Option<bool> {
    if !success {
        return None;
    }

    match output.trim() {
        "DUYRA_RSOP_APPLIED" => Some(true),
        "DUYRA_RSOP_NONE" => Some(false),
        _ => None,
    }
}

#[cfg(any(target_os = "windows", test))]
pub fn windows_write_protection_summary(status: &WindowsWriteProtectionPreflight) -> String {
    let elevated = match status.process_is_elevated {
        Some(true) => "Yükseltilmiş (Yönetici)",
        Some(false) => "Yükseltilmemiş",
        None => "Belirlenemedi",
    };

    let support = match &status.support {
        WriteProtectionSupport::Supported => "Destek listesiyle eşleşiyor".to_owned(),
        WriteProtectionSupport::UnsupportedEdition(edition) => {
            format!("Desteklenmeyen edisyon ({edition})")
        }
        WriteProtectionSupport::UnsupportedBuild(build) => {
            format!("Desteklenmeyen derleme ({build})")
        }
        WriteProtectionSupport::Unknown => "Belirlenemedi".to_owned(),
    };

    let policy = match &status.computer_policy {
        WritePolicyStatus::Enabled => "Etkin görünüyor (1)".to_owned(),
        WritePolicyStatus::Disabled => "Devre dışı (0)".to_owned(),
        WritePolicyStatus::NotConfigured => "Tanımlı değil".to_owned(),
        WritePolicyStatus::UnexpectedValue(value) => {
            format!("Beklenmeyen değer ({value})")
        }
        WritePolicyStatus::Unknown(_) => "Okunamadı".to_owned(),
    };

    format!(
        concat!(
            "DUYRA - Windows Yazma Koruması Ön Kontrol Özeti\n",
            "Windows: {}\n",
            "Edisyon: {}\n",
            "Derleme: {}\n",
            "DUYRA yönetici yetkisi: {}\n",
            "İşletim sistemi destek durumu: {}\n",
            "Bilgisayar Grup İlkesi (HKLM Deny_Write): {}\n",
            "Ek okuma uyarısı sayısı: {}\n",
            "\n",
            "Not: Bu sonuç yalnızca bilgisayar düzeyindeki HKLM ilke kaydını gösterir.\n",
            "Tek başına USB aygıtının fiilen salt okunur olduğunu kanıtlamaz.\n",
            "Bu ön kontrol sistem ayarlarını değiştirmez ve USB aygıtını açmaz."
        ),
        status.product_name.as_deref().unwrap_or("Okunamadı"),
        status.edition_id.as_deref().unwrap_or("Okunamadı"),
        status
            .build_number
            .map(|build| build.to_string())
            .unwrap_or_else(|| "Okunamadı".to_owned()),
        elevated,
        support,
        policy,
        status.read_warnings.len(),
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriteProtectionPreflight {
    #[cfg(target_os = "windows")]
    Windows(WindowsWriteProtectionPreflight),
    #[cfg(not(target_os = "windows"))]
    UnsupportedPlatform,
}

pub fn write_protection_preflight() -> WriteProtectionPreflight {
    #[cfg(target_os = "windows")]
    {
        WriteProtectionPreflight::Windows(windows_write_protection::inspect())
    }

    #[cfg(not(target_os = "windows"))]
    {
        WriteProtectionPreflight::UnsupportedPlatform
    }
}

#[cfg(any(target_os = "windows", test))]
fn is_supported_windows_edition(edition_id: &str) -> bool {
    let edition_id = edition_id.to_ascii_lowercase();

    edition_id.starts_with("professional")
        || edition_id.starts_with("enterprise")
        || edition_id.starts_with("education")
        || edition_id.starts_with("iotenterprise")
}

#[cfg(any(target_os = "windows", test))]
fn assess_write_protection_support(
    edition_id: Option<&str>,
    build_number: Option<u32>,
) -> WriteProtectionSupport {
    let Some(edition_id) = edition_id else {
        return WriteProtectionSupport::Unknown;
    };

    if !is_supported_windows_edition(edition_id) {
        return WriteProtectionSupport::UnsupportedEdition(edition_id.to_owned());
    }

    let Some(build_number) = build_number else {
        return WriteProtectionSupport::Unknown;
    };

    if build_number < 17_763 {
        return WriteProtectionSupport::UnsupportedBuild(build_number);
    }

    WriteProtectionSupport::Supported
}

#[cfg(target_os = "windows")]
mod windows_write_protection {
    use super::{
        PolicyAuthorityCheck, WindowsWriteProtectionPreflight, WritePolicyStatus,
        assess_write_protection_support,
    };
    use std::{
        ffi::c_void,
        mem::size_of,
        os::windows::process::CommandExt,
        path::PathBuf,
        process::{Command, Stdio},
        ptr, thread,
        time::{Duration, Instant},
    };

    type HKey = *mut c_void;
    type Handle = *mut c_void;

    const ERROR_SUCCESS: i32 = 0;
    const ERROR_FILE_NOT_FOUND: i32 = 2;
    const ERROR_PATH_NOT_FOUND: i32 = 3;
    const ERROR_MORE_DATA: i32 = 234;
    const KEY_READ: u32 = 0x0002_0019;
    const REG_SZ: u32 = 1;
    const REG_EXPAND_SZ: u32 = 2;
    const REG_DWORD: u32 = 4;
    const TOKEN_QUERY: u32 = 0x0008;
    const TOKEN_ELEVATION_CLASS: u32 = 20;
    const MAX_REGISTRY_VALUE_BYTES: usize = 64 * 1024;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    const CURRENT_VERSION_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
    const REMOVABLE_DISK_POLICY_KEY: &str = r"SOFTWARE\Policies\Microsoft\Windows\RemovableStorageDevices\{53f5630d-b6bf-11d0-94f2-00a0c91efb8b}";

    #[repr(C)]
    struct TokenElevation {
        token_is_elevated: u32,
    }

    #[link(name = "advapi32")]
    unsafe extern "system" {
        #[link_name = "RegOpenKeyExW"]
        fn reg_open_key_ex_w(
            key: HKey,
            sub_key: *const u16,
            options: u32,
            desired_access: u32,
            result: *mut HKey,
        ) -> i32;

        #[link_name = "RegQueryValueExW"]
        fn reg_query_value_ex_w(
            key: HKey,
            value_name: *const u16,
            reserved: *mut u32,
            value_type: *mut u32,
            data: *mut u8,
            data_size: *mut u32,
        ) -> i32;

        #[link_name = "RegCloseKey"]
        fn reg_close_key(key: HKey) -> i32;

        #[link_name = "RegQueryInfoKeyW"]
        fn reg_query_info_key_w(
            key: HKey,
            class: *mut u16,
            class_length: *mut u32,
            reserved: *mut u32,
            sub_key_count: *mut u32,
            max_sub_key_length: *mut u32,
            max_class_length: *mut u32,
            value_count: *mut u32,
            max_value_name_length: *mut u32,
            max_value_length: *mut u32,
            security_descriptor_length: *mut u32,
            last_write_time: *mut c_void,
        ) -> i32;

        #[link_name = "OpenProcessToken"]
        fn open_process_token(process: Handle, desired_access: u32, token: *mut Handle) -> i32;

        #[link_name = "GetTokenInformation"]
        fn get_token_information(
            token: Handle,
            information_class: u32,
            information: *mut c_void,
            information_length: u32,
            return_length: *mut u32,
        ) -> i32;
    }

    #[link(name = "netapi32")]
    unsafe extern "system" {
        #[link_name = "NetGetJoinInformation"]
        fn net_get_join_information(
            server: *const u16,
            name_buffer: *mut *mut u16,
            buffer_type: *mut u32,
        ) -> u32;

        #[link_name = "NetApiBufferFree"]
        fn net_api_buffer_free(buffer: *mut c_void) -> u32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "GetCurrentProcess"]
        fn get_current_process() -> Handle;

        #[link_name = "CloseHandle"]
        fn close_handle(handle: Handle) -> i32;

        #[link_name = "GetLastError"]
        fn get_last_error() -> u32;
    }

    fn to_wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn win32_error(operation: &str, code: i32) -> String {
        format!("{operation} başarısız (Windows hata kodu: {code}).")
    }

    fn open_registry_key(path: &str) -> Result<Option<HKey>, String> {
        let path = to_wide(path);
        let hkey_local_machine = (-2_147_483_646isize) as HKey;
        let mut key: HKey = ptr::null_mut();

        let result =
            unsafe { reg_open_key_ex_w(hkey_local_machine, path.as_ptr(), 0, KEY_READ, &mut key) };

        match result {
            ERROR_SUCCESS => Ok(Some(key)),
            ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND => Ok(None),
            error => Err(win32_error(
                "Windows kayıt defteri anahtarı okunması",
                error,
            )),
        }
    }

    fn query_registry_value(key: HKey, name: &str) -> Result<Option<(u32, Vec<u8>)>, String> {
        let name = to_wide(name);
        let mut value_type = 0u32;
        let mut data_size = 0u32;

        let first_query = unsafe {
            reg_query_value_ex_w(
                key,
                name.as_ptr(),
                ptr::null_mut(),
                &mut value_type,
                ptr::null_mut(),
                &mut data_size,
            )
        };

        match first_query {
            ERROR_FILE_NOT_FOUND => return Ok(None),
            ERROR_SUCCESS | ERROR_MORE_DATA => {}
            error => return Err(win32_error("Windows kayıt defteri değeri sorgusu", error)),
        }

        let size = data_size as usize;
        if size > MAX_REGISTRY_VALUE_BYTES {
            return Err("Windows kayıt defteri değeri beklenenden büyük.".to_owned());
        }

        let mut data = vec![0u8; size];
        let mut actual_size = data_size;

        let second_query = unsafe {
            reg_query_value_ex_w(
                key,
                name.as_ptr(),
                ptr::null_mut(),
                &mut value_type,
                if data.is_empty() {
                    ptr::null_mut()
                } else {
                    data.as_mut_ptr()
                },
                &mut actual_size,
            )
        };

        match second_query {
            ERROR_FILE_NOT_FOUND => Ok(None),
            ERROR_SUCCESS => {
                data.truncate(actual_size as usize);
                Ok(Some((value_type, data)))
            }
            error => Err(win32_error("Windows kayıt defteri değeri okunması", error)),
        }
    }

    fn read_registry_value(
        key_path: &str,
        value_name: &str,
    ) -> Result<Option<(u32, Vec<u8>)>, String> {
        let Some(key) = open_registry_key(key_path)? else {
            return Ok(None);
        };

        let result = query_registry_value(key, value_name);
        unsafe {
            reg_close_key(key);
        }
        result
    }

    fn read_registry_string(key_path: &str, value_name: &str) -> Result<Option<String>, String> {
        let Some((value_type, data)) = read_registry_value(key_path, value_name)? else {
            return Ok(None);
        };

        if value_type != REG_SZ && value_type != REG_EXPAND_SZ {
            return Err(format!(
                "Windows kayıt defterindeki {value_name} değeri metin türünde değil."
            ));
        }

        if data.len() % size_of::<u16>() != 0 {
            return Err(format!(
                "Windows kayıt defterindeki {value_name} değeri geçersiz biçimde."
            ));
        }

        let (chunks, _remainder) = data.as_chunks::<2>();
        let units = chunks
            .iter()
            .map(|bytes| u16::from_le_bytes(*bytes))
            .take_while(|unit| *unit != 0)
            .collect::<Vec<_>>();
        let value = String::from_utf16_lossy(&units);
        let value = value.trim().to_owned();

        Ok((!value.is_empty()).then_some(value))
    }

    fn read_registry_dword(key_path: &str, value_name: &str) -> Result<Option<u32>, String> {
        let Some((value_type, data)) = read_registry_value(key_path, value_name)? else {
            return Ok(None);
        };

        if value_type != REG_DWORD || data.len() != size_of::<u32>() {
            return Err(format!(
                "Windows kayıt defterindeki {value_name} değeri beklenen DWORD biçiminde değil."
            ));
        }

        Ok(Some(u32::from_le_bytes([
            data[0], data[1], data[2], data[3],
        ])))
    }

    fn process_is_elevated() -> Result<bool, String> {
        let mut token: Handle = ptr::null_mut();
        let opened = unsafe { open_process_token(get_current_process(), TOKEN_QUERY, &mut token) };

        if opened == 0 {
            let code = unsafe { get_last_error() } as i32;
            return Err(win32_error("Uygulama yetki durumu sorgusu", code));
        }

        let mut elevation = TokenElevation {
            token_is_elevated: 0,
        };
        let mut returned_length = 0u32;
        let success = unsafe {
            get_token_information(
                token,
                TOKEN_ELEVATION_CLASS,
                (&mut elevation as *mut TokenElevation).cast::<c_void>(),
                size_of::<TokenElevation>() as u32,
                &mut returned_length,
            )
        };

        if success == 0 {
            let code = unsafe { get_last_error() } as i32;
            unsafe {
                close_handle(token);
            }
            return Err(win32_error("Uygulama yükseltilmiş yetki sorgusu", code));
        }

        unsafe {
            close_handle(token);
        }
        Ok(elevation.token_is_elevated != 0)
    }

    fn read_policy() -> WritePolicyStatus {
        match read_registry_dword(REMOVABLE_DISK_POLICY_KEY, "Deny_Write") {
            Ok(value) => super::policy_status_from_value(value),
            Err(error) => WritePolicyStatus::Unknown(error),
        }
    }

    const MDM_ENROLLMENTS_KEY: &str = r"SOFTWARE\Microsoft\Enrollments";
    const MDM_STORAGE_POLICY_KEY: &str = r"SOFTWARE\Microsoft\PolicyManager\current\device\Storage";

    fn registry_subkey_count(key_path: &str) -> Result<Option<u32>, String> {
        let Some(key) = open_registry_key(key_path)? else {
            return Ok(None);
        };

        let mut sub_key_count = 0u32;
        let result = unsafe {
            reg_query_info_key_w(
                key,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut sub_key_count,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        unsafe {
            reg_close_key(key);
        }

        if result == ERROR_SUCCESS {
            Ok(Some(sub_key_count))
        } else {
            Err(win32_error(
                "Windows kayıt defteri alt anahtar sorgusu",
                result,
            ))
        }
    }

    fn domain_joined() -> Option<bool> {
        const NETSETUP_UNJOINED: u32 = 1;
        const NETSETUP_WORKGROUP_NAME: u32 = 2;
        const NETSETUP_DOMAIN_NAME: u32 = 3;

        let mut name_buffer: *mut u16 = ptr::null_mut();
        let mut join_status = 0u32;
        let result =
            unsafe { net_get_join_information(ptr::null(), &mut name_buffer, &mut join_status) };

        if !name_buffer.is_null() {
            unsafe {
                net_api_buffer_free(name_buffer.cast::<c_void>());
            }
        }

        if result != 0 {
            return None;
        }

        match join_status {
            NETSETUP_DOMAIN_NAME => Some(true),
            NETSETUP_UNJOINED | NETSETUP_WORKGROUP_NAME => Some(false),
            _ => None,
        }
    }

    fn rsop_policy_applied() -> Option<bool> {
        let system_root = std::env::var_os("SystemRoot")?;
        let powershell = PathBuf::from(system_root)
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");

        let script = r#"
$ErrorActionPreference = 'Stop'
$target = 'Software\Policies\Microsoft\Windows\RemovableStorageDevices\{53f5630d-b6bf-11d0-94f2-00a0c91efb8b}'
try {
    $setting = Get-CimInstance -Namespace 'root\RSOP\Computer' -ClassName 'RSOP_RegistryPolicySetting' -ErrorAction Stop |
        Where-Object {
            $_.registryKey -ieq $target -and
            $_.valueName -ieq 'Deny_Write'
        } |
        Select-Object -First 1

    if ($null -eq $setting) {
        [Console]::Out.Write('DUYRA_RSOP_NONE')
    } else {
        [Console]::Out.Write('DUYRA_RSOP_APPLIED')
    }
    exit 0
} catch {
    [Console]::Error.Write('DUYRA_RSOP_UNKNOWN')
    exit 2
}
"#;

        let mut child = Command::new(powershell)
            .args([
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                script,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .ok()?;

        let started = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) if started.elapsed() < Duration::from_secs(8) => {
                    thread::sleep(Duration::from_millis(50));
                }
                Ok(None) | Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
            }
        }

        let output = child.wait_with_output().ok()?;
        let stdout = String::from_utf8(output.stdout).ok()?;
        super::parse_rsop_policy_marker(output.status.success(), &stdout)
    }

    pub fn inspect_authority() -> PolicyAuthorityCheck {
        let group_policy_applied = rsop_policy_applied();

        let mdm_enrollment_detected = match registry_subkey_count(MDM_ENROLLMENTS_KEY) {
            Ok(count) => Some(count.is_some_and(|count| count > 0)),
            Err(_) => None,
        };

        let mdm_policy_present =
            match read_registry_value(MDM_STORAGE_POLICY_KEY, "RemovableDiskDenyWriteAccess") {
                Ok(value) => Some(value.is_some()),
                Err(_) => None,
            };

        let current_policy_value_present = match read_policy() {
            WritePolicyStatus::NotConfigured => Some(false),
            WritePolicyStatus::Enabled
            | WritePolicyStatus::Disabled
            | WritePolicyStatus::UnexpectedValue(_) => Some(true),
            WritePolicyStatus::Unknown(_) => None,
        };

        PolicyAuthorityCheck {
            group_policy_applied,
            mdm_enrollment_detected,
            mdm_policy_present,
            domain_joined: domain_joined(),
            current_policy_value_present,
        }
    }

    pub fn inspect() -> WindowsWriteProtectionPreflight {
        let mut read_warnings = Vec::new();

        let product_name = match read_registry_string(CURRENT_VERSION_KEY, "ProductName") {
            Ok(value) => value,
            Err(error) => {
                read_warnings.push(error);
                None
            }
        };

        let edition_id = match read_registry_string(CURRENT_VERSION_KEY, "EditionID") {
            Ok(value) => value,
            Err(error) => {
                read_warnings.push(error);
                None
            }
        };

        let build_number = match read_registry_string(CURRENT_VERSION_KEY, "CurrentBuildNumber") {
            Ok(Some(value)) => match value.parse::<u32>() {
                Ok(build) => Some(build),
                Err(_) => {
                    read_warnings
                        .push("Windows derleme numarası sayısal olarak okunamadı.".to_owned());
                    None
                }
            },
            Ok(None) => None,
            Err(error) => {
                read_warnings.push(error);
                None
            }
        };

        let process_is_elevated = match process_is_elevated() {
            Ok(value) => Some(value),
            Err(error) => {
                read_warnings.push(error);
                None
            }
        };

        let support = assess_write_protection_support(edition_id.as_deref(), build_number);
        let computer_policy = read_policy();

        WindowsWriteProtectionPreflight {
            product_name,
            edition_id,
            build_number,
            process_is_elevated,
            support,
            computer_policy,
            read_warnings,
        }
    }
}

#[cfg(target_os = "windows")]
pub fn inspect_windows_policy_authority() -> PolicyAuthorityCheck {
    windows_write_protection::inspect_authority()
}

#[cfg(test)]
mod write_protection_tests {
    use super::{
        WritePolicyStatus, WriteProtectionSupport, assess_write_protection_support,
        is_supported_windows_edition, policy_status_from_value,
    };

    #[test]
    fn supported_windows_editions_are_recognized() {
        for edition in [
            "Professional",
            "ProfessionalN",
            "Enterprise",
            "EnterpriseS",
            "Education",
            "IoTEnterprise",
            "IoTEnterpriseS",
        ] {
            assert!(is_supported_windows_edition(edition), "{edition}");
        }
    }

    #[test]
    fn home_and_unknown_editions_are_not_marked_as_supported() {
        assert!(!is_supported_windows_edition("Core"));
        assert!(!is_supported_windows_edition("Home"));
        assert!(!is_supported_windows_edition("ServerStandard"));
    }

    #[test]
    fn windows_build_and_edition_must_both_be_supported() {
        assert_eq!(
            assess_write_protection_support(Some("Professional"), Some(17_763)),
            WriteProtectionSupport::Supported
        );
        assert_eq!(
            assess_write_protection_support(Some("Professional"), Some(17_762)),
            WriteProtectionSupport::UnsupportedBuild(17_762)
        );
        assert_eq!(
            assess_write_protection_support(Some("Core"), Some(26_100)),
            WriteProtectionSupport::UnsupportedEdition("Core".to_owned())
        );
        assert_eq!(
            assess_write_protection_support(None, Some(26_100)),
            WriteProtectionSupport::Unknown
        );
    }

    #[test]
    fn windows_preflight_summary_contains_status_without_raw_error_details() {
        let status = super::WindowsWriteProtectionPreflight {
            product_name: Some("Windows 10 Pro".to_owned()),
            edition_id: Some("Professional".to_owned()),
            build_number: Some(26_300),
            process_is_elevated: Some(true),
            support: WriteProtectionSupport::Supported,
            computer_policy: WritePolicyStatus::Unknown(
                "Registry read failed: C:\\\\Users\\\\Private".to_owned(),
            ),
            read_warnings: vec!["Sensitive path: C:\\\\Users\\\\Private".to_owned()],
        };

        let summary = super::windows_write_protection_summary(&status);

        assert!(summary.contains("Windows: Windows 10 Pro"));
        assert!(summary.lines().count() >= 11);
        assert!(summary.contains("Edisyon: Professional"));
        assert!(summary.contains("Derleme: 26300"));
        assert!(summary.contains("DUYRA yönetici yetkisi: Yükseltilmiş (Yönetici)"));
        assert!(summary.contains("Destek listesiyle eşleşiyor"));
        assert!(summary.contains("HKLM Deny_Write): Okunamadı"));
        assert!(summary.contains("Ek okuma uyarısı sayısı: 1"));
        assert!(summary.contains("fiilen salt okunur olduğunu kanıtlamaz"));
        assert!(!summary.contains("Registry read failed"));
        assert!(!summary.contains("Sensitive path"));
        assert!(!summary.contains("C:\\\\Users"));
    }

    #[test]
    fn windows_preflight_summary_distinguishes_policy_and_elevation_states() {
        let cases = [
            (WritePolicyStatus::Enabled, "Etkin görünüyor (1)"),
            (WritePolicyStatus::Disabled, "Devre dışı (0)"),
            (WritePolicyStatus::NotConfigured, "Tanımlı değil"),
            (
                WritePolicyStatus::UnexpectedValue(9),
                "Beklenmeyen değer (9)",
            ),
        ];

        for (policy, expected) in cases {
            let status = super::WindowsWriteProtectionPreflight {
                product_name: None,
                edition_id: None,
                build_number: None,
                process_is_elevated: Some(false),
                support: WriteProtectionSupport::Unknown,
                computer_policy: policy,
                read_warnings: Vec::new(),
            };
            let summary = super::windows_write_protection_summary(&status);
            assert!(summary.contains("DUYRA yönetici yetkisi: Yükseltilmemiş"));
            assert!(summary.contains(&format!("HKLM Deny_Write): {expected}")));
        }
    }

    #[test]
    fn policy_registry_values_map_to_clear_states() {
        assert_eq!(
            policy_status_from_value(Some(1)),
            WritePolicyStatus::Enabled
        );
        assert_eq!(
            policy_status_from_value(Some(0)),
            WritePolicyStatus::Disabled
        );
        assert_eq!(
            policy_status_from_value(None),
            WritePolicyStatus::NotConfigured
        );
        assert_eq!(
            policy_status_from_value(Some(7)),
            WritePolicyStatus::UnexpectedValue(7)
        );
    }
}

#[cfg(test)]
mod policy_authority_tests {
    use super::{PolicyAuthorityCheck, PolicyChangeAssessment, parse_rsop_policy_marker};

    #[test]
    fn rsop_marker_requires_a_successful_exact_response() {
        assert_eq!(
            parse_rsop_policy_marker(true, "DUYRA_RSOP_APPLIED"),
            Some(true)
        );
        assert_eq!(
            parse_rsop_policy_marker(true, "DUYRA_RSOP_NONE"),
            Some(false)
        );
        assert_eq!(parse_rsop_policy_marker(false, "DUYRA_RSOP_NONE"), None);
        assert_eq!(
            parse_rsop_policy_marker(true, "DUYRA_RSOP_NONE extra"),
            None
        );
        assert_eq!(parse_rsop_policy_marker(true, ""), None);
    }

    #[test]
    fn any_detected_management_source_blocks_policy_changes() {
        let mut check = reviewable_check();
        check.group_policy_applied = Some(true);
        assert_eq!(check.assessment(), PolicyChangeAssessment::Managed);

        let mut check = reviewable_check();
        check.mdm_policy_present = Some(true);
        assert_eq!(check.assessment(), PolicyChangeAssessment::Managed);

        let mut check = reviewable_check();
        check.domain_joined = Some(true);
        assert_eq!(check.assessment(), PolicyChangeAssessment::Managed);
    }

    #[test]
    fn enrollment_marker_alone_is_ambiguous_and_never_reviewable() {
        let mut check = reviewable_check();
        check.mdm_enrollment_detected = Some(true);

        assert_eq!(check.assessment(), PolicyChangeAssessment::Unknown);
    }

    #[test]
    fn reviewable_result_requires_every_probe_to_confirm_no_management() {
        assert_eq!(
            reviewable_check().assessment(),
            PolicyChangeAssessment::Reviewable
        );
        assert_eq!(
            PolicyAuthorityCheck::unknown().assessment(),
            PolicyChangeAssessment::Unknown
        );

        let mut incomplete = reviewable_check();
        incomplete.mdm_enrollment_detected = None;
        assert_eq!(incomplete.assessment(), PolicyChangeAssessment::Unknown);
    }

    #[test]
    fn unattributed_existing_policy_value_never_becomes_reviewable() {
        let mut check = reviewable_check();
        check.current_policy_value_present = Some(true);
        assert_eq!(check.assessment(), PolicyChangeAssessment::Unknown);
    }

    fn reviewable_check() -> PolicyAuthorityCheck {
        PolicyAuthorityCheck {
            group_policy_applied: Some(false),
            mdm_enrollment_detected: Some(false),
            mdm_policy_present: Some(false),
            domain_joined: Some(false),
            current_policy_value_present: Some(false),
        }
    }
}

#[cfg(all(test, target_os = "windows"))]
mod windows_preflight_tests {
    use super::write_protection_preflight;

    #[test]
    fn windows_preflight_reads_system_information_without_changing_it() {
        let first = write_protection_preflight();
        let second = write_protection_preflight();

        assert_eq!(first, second);
        let super::WriteProtectionPreflight::Windows(status) = first;

        assert!(
            status
                .product_name
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        );
        assert!(
            status
                .edition_id
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        );
        assert!(status.build_number.is_some());
    }
}

#[cfg(all(test, target_os = "windows"))]
mod windows_policy_authority_smoke_tests {
    use super::inspect_windows_policy_authority;

    #[test]
    fn windows_policy_authority_probe_reads_join_state() {
        let result = inspect_windows_policy_authority();

        assert!(
            result.domain_joined.is_some(),
            "NetGetJoinInformation should return a known join state on the Windows runner"
        );
    }
}


#[cfg(test)]
mod policy_change_readiness_tests {
    use super::{
        PolicyAuthorityCheck, PolicyChangeReadiness, WindowsWriteProtectionPreflight,
        WritePolicyStatus, WriteProtectionSupport, assess_policy_change_readiness,
    };

    fn supported_status(
        computer_policy: WritePolicyStatus,
        process_is_elevated: Option<bool>,
        support: WriteProtectionSupport,
    ) -> WindowsWriteProtectionPreflight {
        WindowsWriteProtectionPreflight {
            product_name: Some("Windows 11 Pro".to_owned()),
            edition_id: Some("Professional".to_owned()),
            build_number: Some(26_300),
            process_is_elevated,
            support,
            computer_policy,
            read_warnings: Vec::new(),
        }
    }

    fn unmanaged_authority() -> PolicyAuthorityCheck {
        PolicyAuthorityCheck {
            group_policy_applied: Some(false),
            mdm_enrollment_detected: Some(false),
            mdm_policy_present: Some(false),
            domain_joined: Some(false),
            current_policy_value_present: Some(false),
        }
    }

    #[test]
    fn prerequisites_are_met_only_for_supported_unmanaged_elevated_unconfigured_system() {
        let status = supported_status(
            WritePolicyStatus::NotConfigured,
            Some(true),
            WriteProtectionSupport::Supported,
        );

        assert_eq!(
            assess_policy_change_readiness(&status, &unmanaged_authority()),
            PolicyChangeReadiness::PreconditionsMet
        );
    }

    #[test]
    fn unsupported_windows_is_blocked() {
        for support in [
            WriteProtectionSupport::Unknown,
            WriteProtectionSupport::UnsupportedBuild(17_762),
            WriteProtectionSupport::UnsupportedEdition("Core".to_owned()),
        ] {
            let status = supported_status(
                WritePolicyStatus::NotConfigured,
                Some(true),
                support,
            );

            assert_eq!(
                assess_policy_change_readiness(&status, &unmanaged_authority()),
                PolicyChangeReadiness::UnsupportedWindows
            );
        }
    }

    #[test]
    fn managed_and_unknown_authority_are_blocked() {
        let status = supported_status(
            WritePolicyStatus::NotConfigured,
            Some(true),
            WriteProtectionSupport::Supported,
        );

        let mut managed = unmanaged_authority();
        managed.mdm_policy_present = Some(true);
        assert_eq!(
            assess_policy_change_readiness(&status, &managed),
            PolicyChangeReadiness::ManagedByOrganization
        );

        let mut unknown = unmanaged_authority();
        unknown.mdm_enrollment_detected = Some(true);
        assert_eq!(
            assess_policy_change_readiness(&status, &unknown),
            PolicyChangeReadiness::SourceUnknown
        );

        assert_eq!(
            assess_policy_change_readiness(&status, &PolicyAuthorityCheck::unknown()),
            PolicyChangeReadiness::SourceUnknown
        );
    }

    #[test]
    fn an_existing_policy_value_is_never_overwritten_by_a_future_change_flow() {
        let cases = [
            WritePolicyStatus::Enabled,
            WritePolicyStatus::Disabled,
            WritePolicyStatus::UnexpectedValue(7),
        ];

        for policy in cases {
            let status = supported_status(
                policy,
                Some(true),
                WriteProtectionSupport::Supported,
            );

            assert_eq!(
                assess_policy_change_readiness(&status, &unmanaged_authority()),
                PolicyChangeReadiness::ExistingPolicyValue
            );
        }
    }

    #[test]
    fn unreadable_policy_value_and_non_elevated_process_are_blocked() {
        let unreadable = supported_status(
            WritePolicyStatus::Unknown("private error details".to_owned()),
            Some(true),
            WriteProtectionSupport::Supported,
        );
        assert_eq!(
            assess_policy_change_readiness(&unreadable, &unmanaged_authority()),
            PolicyChangeReadiness::SourceUnknown
        );

        for elevation in [Some(false), None] {
            let status = supported_status(
                WritePolicyStatus::NotConfigured,
                elevation,
                WriteProtectionSupport::Supported,
            );
            assert_eq!(
                assess_policy_change_readiness(&status, &unmanaged_authority()),
                PolicyChangeReadiness::NotElevated
            );
        }
    }
}
