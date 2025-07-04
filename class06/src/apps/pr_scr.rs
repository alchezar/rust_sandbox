// IKinder

use screenshots as scr;
use std::env;
use std::fs;
use std::io;

const TARGET_DIR: &str = "screens";

#[derive(Debug)]
pub enum PrintScreenError {
	CurrentDirectory(io::Error),
	CreateDirectory(io::Error),
	Screenshot(anyhow::Error),
}

pub fn start() -> Result<(), PrintScreenError> {
	let args = env::args().collect::<Vec<_>>();
	let dir = args
		.get(1)
		.unwrap_or(&TARGET_DIR.to_owned())
		.to_owned();
	let mut path = env::current_dir().map_err(|e| PrintScreenError::CurrentDirectory(e))?;
	path.push(&dir);

	fs::create_dir_all(&path).map_err(|e| PrintScreenError::CreateDirectory(e))?;

	rdev::grab(move |e| callback(e, &dir))
		.map_err(|er| println!("Error: {er:?}"))
		.unwrap();

	Ok(())
}

fn callback(event: rdev::Event, directory: &String) -> Option<rdev::Event> {
	match event.event_type {
		rdev::EventType::KeyPress(rdev::Key::KeyP) => {
			_ = make_screen(directory);
			None
		}
		_ => Some(event),
	}
}

fn make_screen(directory: &String) -> Result<(), PrintScreenError> {
	let screens = scr::Screen::all().unwrap();
	for screen in screens {
		let image = screen.capture().unwrap();
		let now = chrono::Utc::now();
		let name = format!("{}/{}.png", directory, now.format("%y%m%d_%H%M%S_%3f"));
		image.save(name).unwrap();
	}

	scr::Screen::all()
		.map_err(|e| PrintScreenError::Screenshot(e))?
		.iter()
		.for_each(|screen| {
			screen
				.capture()
				.unwrap()
				.save(format!("{}/{}.png", directory, chrono::Utc::now().format("%y%m%d_%H%M%S_%3f")))
				.unwrap();
		});

	Ok(())
}
