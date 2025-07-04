// IKinder

// -----------------------------------------------------------------------------
// Generic functions: Intro
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum OldLockError {
	MechanicalError(i32),
	NetworkError,
	NotAuthorized,
}

use std::error;
impl error::Error for OldLockError {}

use std::fmt;
impl fmt::Display for OldLockError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::MechanicalError(id) => write!(f, "Mechanical Error {}", id),
			Self::NetworkError => write!(f, "Network Error"),
			Self::NotAuthorized => write!(f, "Not Authorized"),
		}
	}
}

#[derive(Debug, thiserror::Error)]
enum NetworkError {
	#[error("Connection timed out")]
	TimeOut,
	#[error("Unreachable")]
	Unreachable,
}

#[derive(Debug, thiserror::Error)]
enum LockError {
	#[error("Mechanical Error: {0}")]
	MechanicalError(i32),
	#[error("Network Error")]
	NetworkError(#[from] NetworkError),
	#[error("NotAuthorized")]
	NotAuthorized,
}

fn lock_door() -> Result<(), LockError> {
	Err(LockError::MechanicalError(666))
}

// -----------------------------------------------------------------------------
// Generic functions: Demo
// -----------------------------------------------------------------------------

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use thiserror::Error;

struct SubwayPass {
	id: usize,
	funds: isize,
	expires: DateTime<Utc>,
}

#[derive(Debug, Error)]
enum PassError {
	#[error("Expired pass")]
	PassExpired,
	#[error("Insufficient funds: {0}")]
	InsufficientFunds(isize),
	#[error("Pass read error: {0}")]
	ReadError(String),
}

fn swipe_card() -> Result<SubwayPass, PassError> {
	match rand::random::<bool>() {
		true => Ok(SubwayPass {
			id: 0,
			funds: 200,
			expires: Utc::now() + Duration::weeks(52),
		}),
		false => Err(PassError::ReadError("Mag strip failed to read".to_owned())),
	}
}

fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
	if Utc::now() > pass.expires {
		Err(PassError::PassExpired)
	} else if cost > pass.funds {
		Err(PassError::InsufficientFunds(pass.funds))
	} else {
		pass.funds -= cost;
		Ok(())
	}
}

pub fn demo() {
	let pass_status = swipe_card().and_then(|mut pass| use_pass(&mut pass, 3));
	match pass_status {
		Ok(_) => println!("Ok to board"),
		Err(e) => match e {
			PassError::ReadError(s) => println!("Err: {}", s),
			PassError::PassExpired => println!("PassExpired"),
			PassError::InsufficientFunds(f) => println!("InsufficientFunds: {}", f),
		},
	}
}

// -----------------------------------------------------------------------------
// Generic functions: Activity
// -----------------------------------------------------------------------------

#[derive(Debug, Error)]
enum ProgramError {
	#[error(transparent)]
	Menu(#[from] MenuError),
	#[error(transparent)]
	Math(#[from] MathError),
}

#[derive(Debug, Error)]
enum MenuError {
	#[error("menu item not found")]
	NotFound,
}

#[derive(Debug, Error)]
enum MathError {
	#[error("divide by zero error")]
	DivideByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError> {
	match choice {
		"1" => Ok(1),
		"2" => Ok(2),
		"3" => Ok(3),
		_ => Err(MenuError::NotFound),
	}
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
	if b != 0 {
		Ok(a / b)
	} else {
		Err(MathError::DivideByZero)
	}
}

fn run(step: i32) -> Result<(), ProgramError> {
	if step == 1 {
		pick_menu("4")?;
	} else if step == 2 {
		divide(1, 0)?;
	}
	Ok(())
}

pub fn activity() {
	println!("{:?}", run(1));
	println!("{:?}", run(2));
}

static GLOBAL: i32 = 100;
const LOCAL: i32 = 200;
