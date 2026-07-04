use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn peep() {
    println!("Моніторинг активного процесу (Ctrl+C для виходу)...\n");

    let mut last_process = String::new();

    loop {
        match get_active_process_name() {
            Ok(process_name) => {
                if process_name != last_process {
                    println!("Активний процес: {}", process_name);
                    last_process = process_name;
                }
            }
            Err(e) => {
                eprintln!("Помилка: {}", e);
            }
        }

        // Перевіряємо кожну секунду
        thread::sleep(Duration::from_millis(1000));
    }
}

fn get_active_process_name() -> Result<String, String> {
    unsafe {
        // Отримуємо handle активного вікна
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return Err("Не вдалося отримати активне вікно".into());
        }

        // Отримуємо ID процесу
        let mut process_id = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        // Відкриваємо процес
        let process_handle =
            OpenProcess(PROCESS_QUERY_INFORMATION, false, process_id).map_err(|e| e.to_string())?;

        // Отримуємо ім'я виконуваного файлу
        let mut buffer = [0u16; 260]; // MAX_PATH
        let mut size = buffer.len() as u32;

        let result = QueryFullProcessImageNameW(
            process_handle,
            PROCESS_NAME_WIN32,
            windows::core::PWSTR(buffer.as_mut_ptr()),
            &mut size,
        );

        CloseHandle(process_handle).map_err(|e| e.to_string())?;

        match result {
            Ok(_) => {
                // Конвертуємо з UTF-16 в String
                let len = size as usize;
                if len > 0 && len <= buffer.len() {
                    let path = String::from_utf16_lossy(&buffer[..len]);

                    // Витягуємо тільки ім'я файлу
                    if let Some(name) = path.split('\\').last() {
                        Ok(name.to_string())
                    } else {
                        Ok(path)
                    }
                } else {
                    Err("Некоректна довжина імені процесу".into())
                }
            }
            Err(_) => Err("Не вдалося отримати ім'я процесу".into()),
        }
    }
}
