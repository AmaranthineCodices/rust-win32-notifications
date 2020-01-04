use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::guiddef::GUID;
use winapi::um::shellapi::{self, Shell_NotifyIconW, NOTIFYICONDATAW};

fn create_notification(title: String, body: String) -> Result<GUID, u32> {
    let info_bytes: Vec<u16> = OsString::from(&body)
        .as_os_str()
        .encode_wide()
        .take(256)
        .collect();
    let mut info = [0u16; 256];

    let title_bytes: Vec<u16> = OsString::from(&title)
        .as_os_str()
        .encode_wide()
        .take(64)
        .collect();
    let mut title = [0u16; 64];

    unsafe {
        std::ptr::copy_nonoverlapping(
            info_bytes.as_ptr(),
            info.as_mut_ptr(),
            info_bytes.len().min(256),
        );

        std::ptr::copy_nonoverlapping(
            title_bytes.as_ptr(),
            title.as_mut_ptr(),
            title_bytes.len().min(64),
        );
    }

    let guid = {
        let mut gen_guid: GUID = Default::default();
        unsafe {
            winapi::um::combaseapi::CoCreateGuid(&mut gen_guid);
        }
        gen_guid
    };

    let mut icon_data = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        uFlags: shellapi::NIF_INFO | shellapi::NIF_GUID,
        szInfo: info,
        szInfoTitle: title,
        dwInfoFlags: shellapi::NIIF_NONE,
        guidItem: guid,
        ..Default::default()
    };

    let success = unsafe { Shell_NotifyIconW(shellapi::NIM_ADD, &mut icon_data) != 0 };

    if !success {
        let last_err = unsafe { winapi::um::errhandlingapi::GetLastError() };
        return Err(last_err);
    }

    Ok(guid)
}

fn delete_notification(guid: GUID) -> Result<(), u32> {
    let mut icon_data = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        uFlags: shellapi::NIF_INFO | shellapi::NIF_GUID,
        guidItem: guid,
        ..Default::default()
    };

    let success = unsafe { Shell_NotifyIconW(shellapi::NIM_DELETE, &mut icon_data) != 0 };

    if !success {
        let last_err = unsafe { winapi::um::errhandlingapi::GetLastError() };
        return Err(last_err);
    }

    Ok(())
}

fn main() {
    let guid =
        create_notification("Test".to_owned(), "This is a test notification".to_owned()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    delete_notification(guid).unwrap();
}
