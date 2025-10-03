use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::MSG;

use win_wrap as ww;

#[derive(Debug)]
pub enum PeepError {
	EventHook,
}

pub fn peep() -> Result<(), PeepError> {
	// Hook to track out the change of active window.
	let hook = ww::set_win_event_hook(Some(ww::window_change_callback)).ok_or(PeepError::EventHook)?;

	// Show current process on launch.
	let window_handle = ww::get_foreground_window();
	on_window_changed(window_handle);

	// Main loop.
	let mut message = MSG::default();
	while ww::get_message_wide(&mut message) {
		ww::translate_message(&message);
		ww::dispatch_message_wide(&message);
	}
	ww::unhook_win_event(hook);

	Ok(())
}

// Callback функція, що викликається при зміні активного вікна
pub fn on_window_changed(window_handle: HWND) {
	let Some(process_id) = ww::get_window_thread_process_id(window_handle) else {
		return;
	};

	if let Some(name) = get_process_name(process_id) {
		classify_process(&name);
	}
}

// Отримує ім'я процесу за його PID
fn get_process_name(process_id: u32) -> Option<String> {
	// Try to open the process with minimal rights first.
	let process_handle = ww::open_process_limited(process_id)
		.or_else(|_| ww::open_process_full(process_id))
		.ok()?;

	#[rustfmt::skip]
	// Try GetModuleBaseNameW first.
	let process_name = ww::get_module_base_name_wide(process_handle)
		.or_else(|| ww::query_full_process_image_name_wide(process_handle))?;

	// Release the opened process handle.
	ww::close_handle(process_handle);

	Some(process_name)
}

// Класифікує процес за його іменем
fn classify_process(name: &str) {
	let new_name = name
		.trim()
		.strip_suffix(".exe")
		.unwrap()
		.to_lowercase();

	println!("Активний процес: {}", new_name);

	match new_name.as_str() {
		// Браузери
		name if name.contains("chrome") => println!("  → Google Chrome (браузер)"),
		name if name.contains("firefox") => println!("  → Mozilla Firefox (браузер)"),
		name if name.contains("msedge") => println!("  → Microsoft Edge (браузер)"),
		"zen" => println!("  → Zen Browser (браузер)"),
		"opera" => println!("  → Opera (браузер)"),

		// IDE та редактори
		"rustrover64" => println!("  → JetBrains RustRover (IDE)"),
		"code" => println!("  → Visual Studio Code (редактор)"),
		"notepad++" => println!("  → Notepad++ (редактор)"),

		// Термінали
		"warp" => println!("  → Warp Terminal (термінал)"),
		"cmd" => println!("  → Command Prompt (термінал)"),
		"powershell" => println!("  → PowerShell (термінал)"),
		"windowsterminal" => println!("  → Windows Terminal (термінал)"),

		// Системні додатки
		name if name.contains("commander") => println!("  → One Commander (файловий менеджер)"),
		"explorer" => println!("  → Windows Explorer (файловий менеджер)"),
		"searchhost" => println!("  → Windows Start menu (файловий менеджер)"),
		"winrar" => println!("  → WinRAR (архіватор)"),
		"notepad" => println!("  → Notepad (текстовий редактор)"),

		// Месенджери
		name if name.contains("discord") => println!("  → Discord (чат)"),
		name if name.contains("telegram") => println!("  → Telegram (месенджер)"),

		// Музика
		name if name.contains("spotify") => println!("  → Spotify (музика)"),
		name if name.contains("vlc") => println!("  → VLC Media Player (відео плеєр)"),

		// Невідомі процеси
		_ => println!("  → Невідомий додаток"),
	}

	// Порожній рядок для кращої читабельності
	println!();
}

// -----------------------------------------------------------------------------

mod win_wrap {
	use windows::Win32::Foundation::*;
	use windows::Win32::System::ProcessStatus::K32GetModuleBaseNameW;
	use windows::Win32::System::Threading::{
		OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ,
		QueryFullProcessImageNameW,
	};
	use windows::Win32::UI::Accessibility::*;
	use windows::Win32::UI::WindowsAndMessaging::*;
	use windows::core::PWSTR;

	pub fn set_win_event_hook(callback: WINEVENTPROC) -> Option<HWINEVENTHOOK> {
		let hook = unsafe {
			SetWinEventHook(
				EVENT_SYSTEM_FOREGROUND,
				EVENT_SYSTEM_FOREGROUND,
				None,
				callback,
				0,
				0,
				WINEVENT_OUTOFCONTEXT,
			)
		};

		Some(hook).filter(|h| !h.is_invalid())
	}

	pub fn get_window_thread_process_id(hwnd: HWND) -> Option<u32> {
		// Ignore invalid windows.
		if hwnd.is_invalid() {
			return None;
		}

		let mut process_id = 0;
		unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };

		if process_id == 0 {
			return None;
		}

		Some(process_id)
	}

	pub fn open_process_limited(process_id: u32) -> windows_result::Result<HANDLE> {
		unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) }
	}

	pub fn open_process_full(process_id: u32) -> windows_result::Result<HANDLE> {
		unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, process_id) }
	}

	pub fn get_message_wide(message: &mut MSG) -> bool {
		unsafe { GetMessageW(message, None, 0, 0).as_bool() }
	}

	pub fn translate_message(message: &MSG) -> bool {
		unsafe { TranslateMessage(message).as_bool() }
	}

	pub fn dispatch_message_wide(message: &MSG) -> isize {
		unsafe { DispatchMessageW(message).0 }
	}

	pub fn unhook_win_event(hook: HWINEVENTHOOK) -> bool {
		unsafe { UnhookWinEvent(hook).as_bool() }
	}

	pub unsafe extern "system" fn window_change_callback(
		_hook: HWINEVENTHOOK,
		_event: u32,
		hwnd: HWND,
		_id_obj: i32,
		_id_child: i32,
		_id_event_thread: u32,
		_time: u32,
	) {
		super::on_window_changed(hwnd)
	}

	pub fn get_module_base_name_wide(process_handle: HANDLE) -> Option<String> {
		let mut buffer = [0u16; MAX_PATH as usize];
		let length = unsafe { K32GetModuleBaseNameW(process_handle, None, &mut buffer) };

		(length > 0).then(|| String::from_utf16_lossy(&buffer[..length as usize]))
	}

	pub fn query_full_process_image_name_wide(process_handle: HANDLE) -> Option<String> {
		let mut buffer = [0u16; MAX_PATH as usize];
		let mut size = MAX_PATH;

		unsafe {
			QueryFullProcessImageNameW(process_handle, PROCESS_NAME_WIN32, PWSTR(buffer.as_mut_ptr()), &mut size)
		}
		.ok()?;

		String::from_utf16_lossy(&buffer[..size as usize])
			.split('\\')
			.last()
			.map(|s| s.to_owned())
	}

	pub fn close_handle(process_handle: HANDLE) {
		let _ = unsafe { CloseHandle(process_handle) };
	}

	pub fn get_foreground_window() -> HWND {
		unsafe { GetForegroundWindow() }
	}
}
