use crate::peep_hook3::PeepError;
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND, MAX_PATH};
use windows::Win32::System::ProcessStatus::K32GetModuleBaseNameW;
use windows::Win32::System::Threading::{
	OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ,
	QueryFullProcessImageNameW,
};
use windows::Win32::UI::Accessibility::{HWINEVENTHOOK, SetWinEventHook, UnhookWinEvent, WINEVENTPROC};
use windows::Win32::UI::WindowsAndMessaging::{
	DispatchMessageW, EVENT_SYSTEM_FOREGROUND, GetForegroundWindow, GetMessageW, GetWindowThreadProcessId, MSG,
	TranslateMessage, WINEVENT_OUTOFCONTEXT,
};
use windows::core::PWSTR;

pub struct Peeper {
	hook: HWINEVENTHOOK,
	window_handle: HWND,
	message: MSG,
	process_id: u32,
}

impl Peeper {
	pub fn new() -> Result<Self, PeepError> {
		let hook = HWINEVENTHOOK::default();
		let window_handle = HWND::default();
		let message = MSG::default();
		let process_id = 0;

		let mut peep = Self { hook, window_handle, message, process_id };
		peep.set_win_event_hook(Some(Self::window_change_callback))?;
		peep.get_foreground_window();

		Ok(peep)
	}

	pub fn peep(mut self) {
		while self.get_message_wide() {
			self.translate_message();
			self.dispatch_message_wide();
		}
		self.unhook_win_event();
	}

	unsafe extern "system" fn window_change_callback(
		_hook: HWINEVENTHOOK,
		_event: u32,
		window_handle: HWND,
		_id_obj: i32,
		_id_child: i32,
		_id_event_thread: u32,
		_time: u32,
	) {
		let Some(process_id) = Self::get_window_thread_process_id(window_handle) else {
			return;
		};

		if let Some(name) = Self::get_process_name(process_id) {
			Self::classify_process(&name);
		}
	}

	fn set_win_event_hook(&mut self, callback: WINEVENTPROC) -> Result<(), PeepError> {
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

		self.hook = Some(hook)
			.filter(|h| !h.is_invalid())
			.ok_or(PeepError::EventHook)?;

		Ok(())
	}

	pub fn get_foreground_window(&mut self) {
		self.window_handle = unsafe { GetForegroundWindow() };
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

	pub fn get_message_wide(&mut self) -> bool {
		unsafe { GetMessageW(&mut self.message, None, 0, 0).as_bool() }
	}

	pub fn translate_message(&self) -> bool {
		unsafe { TranslateMessage(&self.message).as_bool() }
	}

	pub fn dispatch_message_wide(&self) -> isize {
		unsafe { DispatchMessageW(&self.message).0 }
	}

	pub fn unhook_win_event(self) -> bool {
		unsafe { UnhookWinEvent(self.hook).as_bool() }
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

	fn get_process_name(process_id: u32) -> Option<String> {
		// Try to open the process with minimal rights first.
		let process_handle = Self::open_process_limited(process_id)
			.or_else(|_| Self::open_process_full(process_id))
			.ok()?;

		#[rustfmt::skip]
	// Try GetModuleBaseNameW first.
	let process_name = Self::get_module_base_name_wide(process_handle)
		.or_else(|| Self::query_full_process_image_name_wide(process_handle))?;

		// Release the opened process handle.
		Self::close_handle(process_handle);

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
	}
}
