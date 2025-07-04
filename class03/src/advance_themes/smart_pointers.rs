// IKinder

// -----------------------------------------------------------------------------
// Smart pointers: Intro
// -----------------------------------------------------------------------------

use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
	vin: String,
}

#[derive(Debug)]
struct Door {
	vehicle: Rc<Vehicle>,
}

pub fn intro1() {
	let car = Rc::new(Vehicle { vin: "123".to_owned() });
	let left_door = Door { vehicle: Rc::clone(&car) };
	let right_door = Door { vehicle: Rc::clone(&car) };
	drop(car);

	println!("Vehicle = {:?}", left_door.vehicle);
}

// -----------------------------------------------------------------------------
// Interior Mutability: Sell & RefCell: Intro
// -----------------------------------------------------------------------------

use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Book {
	signed: Cell<bool>,
}

impl Book {
	fn sign(&self) {
		self.signed.set(true);
	}
	fn signed(&self) -> bool {
		self.signed.get()
	}
}

#[derive(Debug)]
struct Person {
	name: RefCell<String>,
}

pub fn intro2() {
	let my_book = Book { signed: Cell::new(false) };
	println!("Signed: {}", my_book.signed());
	my_book.sign();
	println!("Signed: {}", my_book.signed());
}

pub fn intro3() {
	let my_book = Book { signed: Cell::new(false) };
	println!("Signed: {}", my_book.signed());
	my_book.sign();
	println!("Signed: {}", my_book.signed());

	let name = "Amy".to_owned();
	let person = Person { name: RefCell::new(name) };

	let name = person.name.borrow();
	println!("Name: {}", name);
	drop(name);

	let mut name = person.name.borrow_mut();
	*name = "Tim".to_owned();
	drop(name);
	println!("Name: {}", person.name.borrow());

	person.name.replace("John".to_owned());
	println!("Name: {}", person.name.borrow());

	let name = person.name.try_borrow();
	let name = person.name.try_borrow_mut();
}

// -----------------------------------------------------------------------------
// Smart Pointers & RefCell: Demo
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum MenuItem {
	Drink,
	Salad,
}

#[derive(Debug)]
struct ItemOrder {
	item: MenuItem,
	quantity: u32,
}

#[derive(Debug)]
struct TableOrder {
	items: Vec<ItemOrder>,
}

fn new_table_order() -> TableOrder {
	TableOrder {
		items: vec![ItemOrder { item: MenuItem::Drink, quantity: 1 }],
	}
}

type Order = Rc<RefCell<Vec<TableOrder>>>;

#[derive(Debug)]
struct Chef(Order);
#[derive(Debug)]
struct WaitStaff(Order);
#[derive(Debug)]
struct Accounting(Order);

pub fn demo() {
	let orders = Rc::new(RefCell::new(vec![]));

	let chef = Chef(Rc::clone(&orders));
	let wait_staff = WaitStaff(Rc::clone(&orders));
	let accounting = Accounting(Rc::clone(&orders));

	let order = new_table_order();
	orders.borrow_mut().push(order);

	dbg!(chef.0.borrow());
	drop(chef);
	dbg!(wait_staff.0.borrow());
	drop(wait_staff);
	dbg!(accounting.0.borrow());
}

// -----------------------------------------------------------------------------
// Smart Pointers & RefCell: Activity
// -----------------------------------------------------------------------------

// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * 1. Corporate must be able to access the rentals at a storefront
// * 2. Storefronts must be able to rent out vehicles
// * 3. Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

#[derive(Debug)]
enum CorpVehicle {
	Car,
	Truck,
}

#[derive(Debug, Hash, PartialOrd, PartialEq)]
enum Status {
	Available,
	Unavailable,
	Rented,
	Maintenance,
}

#[derive(Debug)]
struct Rental {
	status: Status,
	vehicle: CorpVehicle,
	vin: String,
}

type Rentals = Rc<RefCell<Vec<Rental>>>;

struct Corporate(Rentals);

struct StoreFront(Rentals);

pub fn activity() {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn update_status() {
		let vehicles = vec![
			Rental {
				status: Status::Available,
				vehicle: CorpVehicle::Car,
				vin: "123".to_owned(),
			},
			Rental {
				status: Status::Rented,
				vehicle: CorpVehicle::Truck,
				vin: "234".to_owned(),
			},
		];
		let vehicles: Rentals = Rc::new(RefCell::new(vehicles));

		let corporate = Corporate(Rc::clone(&vehicles));
		let store_front = StoreFront(Rc::clone(&vehicles));

		let mut rentals = store_front.0.borrow_mut();
		if let Some(car) = rentals.get_mut(0) {
			assert_eq!(car.status, Status::Available, "Car status should be available");
			car.status = Status::Rented;
		}
		drop(rentals);

		let mut rentals = corporate.0.borrow_mut();
		if let Some(truck) = rentals.get_mut(1) {
			assert_eq!(truck.status, Status::Rented, "Truck status should be rented");
			truck.status = Status::Available;
		}
		drop(rentals);

		let rentals = store_front.0.borrow();
		if let Some(car) = rentals.get(0) {
			assert_eq!(car.status, Status::Rented, "Now car should be rented");
		}
		if let Some(truck) = rentals.get(1) {
			assert_eq!(truck.status, Status::Available, "Now truck should be rented");
		}
	}
}
