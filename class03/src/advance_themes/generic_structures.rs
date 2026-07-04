// IKinder

// -----------------------------------------------------------------------------
// Generic structs: Intro
// -----------------------------------------------------------------------------

use std::fmt::Debug;

trait Trait1 {}
trait Trait2 {}
trait Trait3 {}

struct Name1<T: Trait1 + Trait2, U: Trait3> {
    field1: T,
    field2: U,
}

struct Name2<T, U>
where
    T: Trait1 + Trait2,
    U: Trait3,
{
    field1: T,
    field2: U,
}

trait Seat {
    fn show(&self);
}
struct Ticket<T: Seat> {
    location: T,
}

#[derive(Clone, Copy, Debug)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32),
}
impl Seat for ConcertSeat {
    fn show(&self) {
        println!("{:?}", self);
    }
}

#[derive(Clone, Copy, Debug)]
enum AirlineSeat {
    FirstClass,
    BusinessClass,
    Economy,
}
impl Seat for AirlineSeat {
    fn show(&self) {
        println!("{:?}", self);
    }
}

fn ticket_info<T: Seat>(ticket: &Ticket<T>) {
    ticket.location.show()
}

pub fn run() {
    let airline = Ticket {
        location: AirlineSeat::FirstClass,
    };
    ticket_info(&airline);
    let concert = Ticket {
        location: ConcertSeat::MidSection(6),
    };
    ticket_info(&concert);

    // Impossible =(
    // let tickets = vec![airline, concert];
}

// -----------------------------------------------------------------------------
// Generic structs: impl BLocks
// -----------------------------------------------------------------------------

trait Game {
    fn name(&self) -> String;
}

#[derive(Debug)]
enum BoardGame {
    Chess,
    Monopoly,
}
impl Game for BoardGame {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
enum VideoGame {
    PlayStation,
    Xbox,
}
impl Game for VideoGame {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

struct PlayRoom<T: Game> {
    game: T,
}
impl PlayRoom<BoardGame> {
    pub fn cleanup(&self) {
        println!("Now the room is cleaned")
    }
}
impl<T> PlayRoom<T>
where
    T: Game + Debug,
{
    pub fn game_info(&self) {
        println!("{:?}", self.game);
    }
}

pub fn run1() {
    let video_room = PlayRoom {
        game: VideoGame::PlayStation,
    };
    let board_room = PlayRoom {
        game: BoardGame::Chess,
    };

    board_room.game_info();
    video_room.game_info();

    board_room.cleanup();
}

// -----------------------------------------------------------------------------
// Generic structs: Demo
// -----------------------------------------------------------------------------

struct Dimensions {
    width: f32,
    height: f32,
    depth: f32,
}

trait Convey {
    fn weight(&self) -> f32;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: Convey> {
    pub items: Vec<T>,
}
impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item)
    }
}

struct CarPart {
    width: f32,
    height: f32,
    depth: f32,
    weight: f32,
    part_number: String,
}
impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "abc".to_owned(),
        }
    }
}
impl Convey for CarPart {
    fn weight(&self) -> f32 {
        self.weight
    }
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

pub fn demo() {
    let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![] };
    belt.add(CarPart::default());
}
// -----------------------------------------------------------------------------
// Generic structs: Activity
// -----------------------------------------------------------------------------

trait Body {}
trait Color {}

#[derive(Debug)]
struct Car {}
impl Body for Car {}
#[derive(Debug)]
struct Scooter {}
impl Body for Scooter {}

#[derive(Debug)]
struct Red {}
impl Color for Red {}
#[derive(Debug)]
struct Black {}
impl Color for Black {}

#[derive(Debug)]
struct Vehicle<B, C>
where
    B: Body,
    C: Color,
{
    body: B,
    color: C,
}
impl<B, C> Vehicle<B, C>
where
    B: Body,
    C: Color,
{
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

fn show_all_vehicles<T: Debug>(vehicles: Vec<T>) {
    for vehicle in vehicles {
        println!("{:?}", vehicle)
    }
}

pub fn activity() {
    let car = Vehicle::new(Car {}, Black {});
    let vespa = Vehicle::new(Scooter {}, Red {});
    println!("{:?}", car);
    println!("{:?}", vespa);
}
