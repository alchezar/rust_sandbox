// IKinder

// -----------------------------------------------------------------------------
// TypeState pattern: Intro
// -----------------------------------------------------------------------------

use rand::Rng;
use std::any::Any;

struct BusTicket;
struct BoardedBusTicket;

impl BusTicket {
    fn board(self) -> BoardedBusTicket {
        BoardedBusTicket
    }
}

pub fn intro1() {
    let ticket = BusTicket;
    let boarded = ticket.board();

    // Compile error. ticket is already boarded.
    //ticket.board();
}

struct File<'a>(&'a str);
impl<'a> File<'a> {
    fn read_bytes(&self) -> Vec<u8> {
        // read data
        Vec::<u8>::new()
    }
    fn delete(self) {
        // delete file
    }
}

pub fn intro2() {
    let file = File("data.txt");
    let data = file.read_bytes();
    file.delete();

    // Compile error. File is already deleted.
    //let data = file.read_bytes();
}

// -----------------------------------------------------------------------------
// TypeState pattern: Demo
// -----------------------------------------------------------------------------

struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnboardingComplete {
    score: u8,
}

#[rustfmt::skip]
impl Employee<Agreement> {
	fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			state: Agreement
		}
	}
	fn read_agreement(self) -> Employee<Signature> {
		self.transition(Signature)
	}
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        match score > 7 {
            true => Ok(self.transition(OnboardingComplete { score })),
            false => Err(self.transition(FailedTraining { score })),
        }
    }
}

pub fn demo() {
    match Employee::<Agreement>::new("John")
        .read_agreement()
        .sign()
        .train(7)
    {
        Ok(complete) => println!("{} have complete the onboarding!", complete.name),
        Err(failed) => println!("{} have failed the training!", failed.name),
    }
}

// -----------------------------------------------------------------------------
// TypeState pattern: Activity
// -----------------------------------------------------------------------------

struct LuggageId(u32);
struct Luggage(LuggageId);
impl Luggage {
    fn new(id: u32) -> CheckIn {
        CheckIn(LuggageId(id))
    }
}

struct CheckIn(LuggageId);
impl CheckIn {
    fn check(self) -> OnLoading {
        OnLoading(self.0)
    }
}

struct OnLoading(LuggageId);
impl OnLoading {
    fn load(self) -> OffLoading {
        OffLoading(self.0)
    }
}

struct OffLoading(LuggageId);
impl OffLoading {
    fn derive(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

struct AwaitingPickup(LuggageId);
impl AwaitingPickup {
    fn waiting(self) -> EndCustody {
        EndCustody(self.0)
    }
}

struct EndCustody(LuggageId);
impl EndCustody {
    fn deliver(self) -> Result<String, String> {
        match rand::random::<bool>() {
            true => Ok("Delivered".to_owned()),
            false => Err("Lost".to_owned()),
        }
    }
}

pub fn activity() {
    match Luggage::new(666)
        .check()
        .load()
        .derive()
        .waiting()
        .deliver()
    {
        Ok(lug) => println!("{}", lug),
        Err(lug) => println!("{}", lug),
    }
}
