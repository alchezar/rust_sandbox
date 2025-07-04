// IKinder

// -----------------------------------------------------------------------------
// Generic functions: Intro
// -----------------------------------------------------------------------------

trait Trait1 {}
trait Trait2 {}

fn function<T1, T2>(param1: T1, param2: T2)
where
	T1: Trait1,
	T2: Trait1 + Trait2,
{
	// Body.
}

trait Move {
	fn move_to(&self, x: i32, y: i32);
}

fn make_move_traits(thing: impl Move, x: i32, y: i32) {
	thing.move_to(x, y)
}

fn make_move_generic1<T: Move>(thing: T, x: i32, y: i32) {
	thing.move_to(x, y)
}

fn make_move_generic2<T>(thing: T, x: i32, y: i32)
where
	T: Move,
{
	thing.move_to(x, y)
}

pub fn run() {}

// -----------------------------------------------------------------------------
// Generic functions: Demo
// -----------------------------------------------------------------------------

trait CheckIn {
	fn check_in(&self);
	fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
	fn check_in(&self) {
		println!("Checked in as pilot")
	}
	fn process(&self) {
		println!("Pilot enters the cockpit")
	}
}
struct Passenger;
impl CheckIn for Passenger {
	fn check_in(&self) {
		println!("Checked in as passenger")
	}
	fn process(&self) {
		println!("Passenger takes a seat")
	}
}
struct Cargo;
impl CheckIn for Cargo {
	fn check_in(&self) {
		println!("Cargo checked in")
	}
	fn process(&self) {
		println!("Cargo moved to storage")
	}
}

fn process_item<T: CheckIn>(item: T) {
	item.check_in();
	item.process();
}

pub fn demo() {
	let paul = Passenger;
	let kathy = Pilot;
	let bag = Cargo;
	let case = Cargo;

	process_item(paul);
	process_item(kathy);
	process_item(bag);
	process_item(case);
}
// -----------------------------------------------------------------------------
// Generic functions: Activity
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum ServicePriority {
	High,
	Standard,
}

trait Priority {
	fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
	fn get_priority(&self) -> ServicePriority {
		ServicePriority::High
	}
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
	fn get_priority(&self) -> ServicePriority {
		ServicePriority::Standard
	}
}

fn print_guest_priority<T>(guest: T)
where
	T: Priority + std::fmt::Debug,
{
	println!("{:?} is {:?} priority", guest, guest.get_priority());
}

pub fn activity() {
	let guest = Guest;
	let vip = ImportantGuest;

	print_guest_priority(guest);
	print_guest_priority(vip);
}
