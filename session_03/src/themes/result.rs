// IKinder

#[derive(Clone, Copy, Debug)]
enum ProtectedLocation {
	All,
	Office,
	Warehouse,
}

impl ProtectedLocation {
	fn required_access_level(&self) -> u16 {
		match self {
			ProtectedLocation::All => 1000,
			ProtectedLocation::Office => 800,
			ProtectedLocation::Warehouse => 500,
		}
	}
}

#[derive(Debug)]
struct Database;

impl Database {
	fn connect() -> Result<Self, String> {
		Ok(Database)
	}
	fn find_employee(&self, name: &str) -> Result<Employee, String> {
		match name {
			"Anita" => Ok(Employee { name: "Anita".to_string() }),
			"Brody" => Ok(Employee { name: "Brody".to_string() }),
			"Catherine" => Ok(Employee { name: "Catherine".to_string() }),
			_ => Err(format!("{} not found", name)),
		}
	}
	fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
		match employee.name.as_str() {
			"Anita" => Ok(KeyCard { access_level: 1000 }),
			"Brody" => Ok(KeyCard { access_level: 500 }),
			other => Err(format!("{} doesn't have a key card", other)),
		}
	}
}

#[derive(Clone, Debug)]
struct Employee {
	name: String,
}

#[derive(Debug)]
struct KeyCard {
	access_level: u16,
}

#[derive(Debug)]
enum AuthorizationStatus {
	Allow,
	Deny,
}

fn authorize(employee_name: &str, location: ProtectedLocation) -> Result<AuthorizationStatus, String> {
	let db = Database::connect()?;
	let employee = db.find_employee(employee_name)?;
	let keycard = db.get_keycard(&employee)?;

	if keycard.access_level >= location.required_access_level() {
		Ok(AuthorizationStatus::Allow)
	} else {
		Ok(AuthorizationStatus::Deny)
	}
}

pub fn run() {
	let result1 = authorize("Anita", ProtectedLocation::All);
	let result2 = authorize("Brody", ProtectedLocation::Office);
	let result3 = authorize("Catherine", ProtectedLocation::Warehouse);

	println!("{result1:?}");
	println!("{result2:?}");
	println!("{result3:?}");
}
