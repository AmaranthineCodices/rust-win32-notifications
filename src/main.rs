#[derive(Clone, Copy, Debug)]
enum NotificationKind {
    Add = 0x00000000,
    Delete = 0x00000002,
}

use winapi::shared::guiddef::GUID;

fn send_notification(
    kind: NotificationKind,
    input_title: String,
    body: String,
    guid: Option<GUID>,
) -> Result<GUID, ()> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::shellapi::{self, Shell_NotifyIconW, NOTIFYICONDATAW};

    let mut info = [0u16; 256];
    let info_bytes: Vec<u16> = OsString::from(&body)
        .as_os_str()
        .encode_wide()
        .take(256)
        .collect();

    let mut title = [0u16; 64];
    let title_bytes: Vec<u16> = OsString::from(&input_title)
        .as_os_str()
        .encode_wide()
        .take(64)
        .collect();

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

    let guid = guid.unwrap_or_else(|| {
        let mut gen_guid: GUID = Default::default();
        unsafe {
            winapi::um::combaseapi::CoCreateGuid(&mut gen_guid);
        }
        gen_guid
    });

    let mut icon_data = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        uFlags: shellapi::NIF_INFO | shellapi::NIF_GUID,
        szInfo: info,
        szInfoTitle: title,
        dwInfoFlags: shellapi::NIIF_NONE,
        guidItem: guid,
        ..Default::default()
    };

    let success = unsafe { Shell_NotifyIconW(kind as u32, &mut icon_data) != 0 };

    if !success {
        let last_err = unsafe { winapi::um::errhandlingapi::GetLastError() };
        println!("{}: {}", success, last_err);
        return Err(());
    }

    Ok(guid)
}

fn main() {
    println!("Hello, world!");
    let guid = send_notification(
        NotificationKind::Add,
        "Test".to_owned(),
        "This is a test notification".to_owned(),
        None,
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    send_notification(
        NotificationKind::Delete,
        "".to_owned(),
        "".to_owned(),
        Some(guid),
    )
    .unwrap();
}
