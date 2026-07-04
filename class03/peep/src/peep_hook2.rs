use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::*;
use windows::Win32::System::ProcessStatus::K32GetModuleBaseNameW;
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION,
    PROCESS_VM_READ, QueryFullProcessImageNameW,
};
use windows::Win32::UI::Accessibility::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PWSTR;

macro_rules! trust {
	($($code:tt)*) => {
		unsafe { $($code)* }
	};
}

pub fn peep() -> Result<(), Box<dyn std::error::Error>> {
    println!("Запуск моніторингу активних процесів (Ctrl+C для виходу)...\n");

    // Hook to track out the change of active window.
    let hook = trust! {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(window_change_callback),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        )
    };

    if hook.is_invalid() {
        return Err("Не вдалося встановити Windows Event Hook".into());
    }

    println!("Hook встановлено успішно. Очікування змін активного вікна...\n");

    // Показуємо поточний активний процес при запуску
    show_current_active_process();

    // Основний цикл обробки повідомлень
    let mut message = MSG::default();
    trust! {
        while GetMessageW(&mut message, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }

        // Видаляємо хук при завершенні (хоча це автоматично відбудеться)
        let _ = UnhookWinEvent(hook);
    }

    Ok(())
}

// Показує поточний активний процес при запуску програми
fn show_current_active_process() {
    unsafe {
        let window_handle = GetForegroundWindow();
        if window_handle.is_invalid() {
            let mut process_id = 0;
            GetWindowThreadProcessId(window_handle, Some(&mut process_id));
            if let Some(process_name) = get_process_name(process_id) {
                println!("Поточний активний процес: {}", process_name);
                classify_process(&process_name);
                println!();
            }
        }
    }
}

// Callback функція, що викликається при зміні активного вікна
unsafe extern "system" fn window_change_callback(
    _hook: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _id_obj: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _time: u32,
) {
    // Ignore invalid windows.
    if hwnd.is_invalid() {
        return;
    }

    let mut process_id = 0;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };
    if process_id == 0 {
        return;
    }

    if let Some(exe_name) = get_process_name(process_id) {
        println!("Активний процес: {}", exe_name);
        classify_process(&exe_name);
        println!(); // Порожній рядок для кращої читабельності
    }
}

// Отримує ім'я процесу за його PID
fn get_process_name(process_id: u32) -> Option<String> {
    // Спробуємо відкрити процес з мінімальними правами
    let process_handle = match trust! { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) }
    {
        Ok(handle) => handle,
        Err(_) => {
            // Якщо не вдалося з обмеженими правами, спробуємо з повними
            match unsafe {
                OpenProcess(
                    PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                    false,
                    process_id,
                )
            } {
                Ok(handle) => handle,
                Err(_) => return None,
            }
        }
    };

    let mut buffer = [0u16; MAX_PATH as usize];

    // Try GetModuleBaseNameW first.
    let length = trust! { K32GetModuleBaseNameW(process_handle, None, &mut buffer)};
    let exe_name = if length > 0 {
        OsString::from_wide(&buffer[..length as usize])
            .to_string_lossy()
            .into_owned()
    } else {
        // Якщо не вдалося, пробуємо QueryFullProcessImageNameW
        let mut size = buffer.len() as u32;
        match trust! {QueryFullProcessImageNameW(process_handle, PROCESS_NAME_WIN32, PWSTR(buffer.as_mut_ptr()), &mut size)}
        {
            Ok(_) if size > 0 => {
                let path = String::from_utf16_lossy(&buffer[..size as usize]);
                // Витягуємо тільки ім'я файлу з повного шляху
                path.split('\\').last().unwrap_or(&path).to_string()
            }
            _ => return None,
        }
    };

    // Закриваємо handle процесу
    let _ = trust! {CloseHandle(process_handle)};

    Some(exe_name)
}

// Класифікує процес за його іменем
fn classify_process(exe_name: &str) {
    match exe_name.to_lowercase().as_str() {
        // Браузери
        name if name.contains("chrome") => println!("  → Google Chrome (браузер)"),
        name if name.contains("firefox") => println!("  → Mozilla Firefox (браузер)"),
        name if name.contains("msedge") => println!("  → Microsoft Edge (браузер)"),
        "zen.exe" => println!("  → Zen Browser (браузер)"),
        "opera.exe" => println!("  → Opera (браузер)"),

        // IDE та редактори
        "rustrover64.exe" => println!("  → JetBrains RustRover (IDE)"),
        "code.exe" => println!("  → Visual Studio Code (редактор)"),
        "devenv.exe" => println!("  → Visual Studio (IDE)"),
        "notepad++.exe" => println!("  → Notepad++ (редактор)"),

        // Термінали
        "warp.exe" => println!("  → Warp Terminal (термінал)"),
        "cmd.exe" => println!("  → Command Prompt (термінал)"),
        "powershell.exe" => println!("  → PowerShell (термінал)"),
        "windowsterminal.exe" => println!("  → Windows Terminal (термінал)"),

        // Системні додатки
        "explorer.exe" => println!("  → Windows Explorer (файловий менеджер)"),
        "winrar.exe" => println!("  → WinRAR (архіватор)"),
        "notepad.exe" => println!("  → Notepad (текстовий редактор)"),

        // Інші популярні додатки
        name if name.contains("discord") => println!("  → Discord (чат)"),
        name if name.contains("telegram") => println!("  → Telegram (месенджер)"),
        name if name.contains("spotify") => println!("  → Spotify (музика)"),
        name if name.contains("vlc") => println!("  → VLC Media Player (відео плеєр)"),

        // Невідомі процеси
        _ => println!("  → Невідомий додаток"),
    }
}
