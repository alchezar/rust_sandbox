// IKinder

use screenshots as scr;
use std::env;
use std::fs;
use std::io;

const TARGET_DIR: &str = "screens";

pub fn start() -> io::Result<()> {
	let args = env::args().collect::<Vec<_>>();
	let dir = args
		.get(1)
		.unwrap_or(&TARGET_DIR.to_owned())
		.to_owned();
	let mut path = env::current_dir()?;
	path.push(&dir);

	if !fs::exists(&path)? {
		fs::create_dir_all(&path)?;
	}

	rdev::grab(move |e| callback(e, &dir))
		.map_err(|er| println!("Error: {er:?}"))
		.unwrap();

	Ok(())
}

fn callback(event: rdev::Event, directory: &String) -> Option<rdev::Event> {
	match event.event_type {
		rdev::EventType::KeyPress(rdev::Key::KeyP) => {
			make_screen(directory);
			return None;
		}
		_ => Some(event),
	}
}

fn make_screen(directory: &String) {
	let screens = scr::Screen::all().unwrap();
	for screen in screens {
		let image = screen.capture().unwrap();
		let now = chrono::Utc::now();
		let name = format!("{}/{}.png", directory, now.format("%y%m%d_%H%M%S_%3f"));
		image.save(name).unwrap();
	}
}
