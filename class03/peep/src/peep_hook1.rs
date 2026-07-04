use windows::Win32::Foundation::HWND;
use windows::Win32::System::ProcessStatus::K32GetModuleBaseNameW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::Accessibility::{HWINEVENTHOOK, SetWinEventHook};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, EVENT_SYSTEM_FOREGROUND, GetMessageW, GetWindowThreadProcessId, MSG,
    TranslateMessage, WINEVENT_OUTOFCONTEXT,
};

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub fn peep() -> Result<(), String> {
    unsafe {
        let hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(window_change_callback),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        if hook.is_invalid() {
            return Err("Windows Event Hook".into());
        }

        // Main loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        Ok(())
    }
}

unsafe extern "system" fn window_change_callback(
    _hook: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _id_obj: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _time: u32,
) {
    if hwnd.is_invalid() {
        return;
    }

    let mut pid = 0;
    let _thread_id = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };

    if pid == 0 {
        return;
    }

    // OpenProcess -> Result<HANDLE>
    let process =
        match unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) } {
            Ok(handle) => handle,
            Err(_) => return, // не змогли відкрити процес
        };

    let mut buf = [0u16; 260];
    let len = unsafe { K32GetModuleBaseNameW(process, None, &mut buf) };

    if len == 0 {
        return;
    }

    let exe = OsString::from_wide(&buf[..len as usize])
        .to_string_lossy()
        .into_owned();

    println!("Активний процес: {}", exe);

    if exe.eq_ignore_ascii_case("zen.exe") {
        println!("-> браузер");
    } else if exe.eq_ignore_ascii_case("rustrover64.exe") {
        println!("-> це RustRover IDE");
    } else if exe.eq_ignore_ascii_case("warp.exe") {
        println!("-> це термінал");
    }
}
