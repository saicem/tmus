use crate::engine::ENGINE;
use windows::core::PWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn set_event_hook() {
    println!("Set event hook");
    unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(on_foreground_changed),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        )
    };
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn on_foreground_changed(
    _: HWINEVENTHOOK,
    _: u32,
    hwnd: HWND,
    _: i32,
    _: i32,
    _: u32,
    _: u32,
) {
    let process_path = process_path(&hwnd);

    println!("foreground change: {}", process_path.clone());

    ENGINE.get().unwrap().on_focus(&process_path);
}

pub fn process_path(hwnd: &HWND) -> String {
    let mut text: [u16; 1024] = [0; 1024];
    let mut process_name_length: u32 = 1024;
    let mut process_id: u32 = 0;

    unsafe {
        GetWindowThreadProcessId(*hwnd, Some(&mut process_id));
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, true, process_id)
            .expect("win32 OpenProcess failed.");
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(text.as_mut_ptr()),
            &mut process_name_length,
        )
        .expect("win32 QueryFullProcessImageNameW failed.")
    }
    String::from_utf16_lossy(&text[..process_name_length as usize])
}
