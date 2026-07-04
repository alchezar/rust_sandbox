// IKinder

// -----------------------------------------------------------------------------
// Ordering: Enum
// -----------------------------------------------------------------------------

#[derive(PartialEq, PartialOrd)]
enum Floor {
    ClientServices,
    Marketing,
    Ops,
}

fn is_below(this: &Floor, other: &Floor) -> bool {
    this < other
}

#[derive(PartialEq, PartialOrd)]
enum Tax {
    Flat(f64),
    None,
    Percentage(f64),
}

fn smallest_amount(tax: Tax, other: Tax) -> Tax {
    if tax < other { tax } else { other }
}

pub fn intro1() {
    let first = Floor::ClientServices;
    let second = Floor::Marketing;
    if first == second {
        unreachable!();
    }

    let no_tax = Tax::None;
    let flat_tax = Tax::Flat(5.5);

    // flat_tax will be less than no_tax, because it higher in enum declaration.
    smallest_amount(no_tax, flat_tax);
}

// -----------------------------------------------------------------------------
// Ordering: Struct
// -----------------------------------------------------------------------------

#[derive(PartialEq, PartialOrd)]
struct User {
    id: i32,
    name: String,
}

#[derive(PartialEq)]
struct AnotherUser {
    id: i32,
    name: String,
}

use std::cmp::Ordering;
impl PartialOrd for AnotherUser {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.name < other.name {
            Some(Ordering::Less)
        } else if self.name > other.name {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }

        // or
        // Some(self.name.cmp(&other.name))
    }
}

pub fn intro2() {
    let a = User {
        id: 1,
        name: "a".to_owned(),
    };
    let b = User {
        id: 2,
        name: "b".to_owned(),
    };

    if a == b {
        unreachable!()
    }

    // Only first field (id) is compared by default!
    if a < b {
        // ...
    }

    let a = AnotherUser {
        id: 1,
        name: "a".to_owned(),
    };
    let b = AnotherUser {
        id: 2,
        name: "b".to_owned(),
    };

    // Now the `name` field will be compared.
    if a < b {
        // ...
    }
}
// -----------------------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------------------

use std::ops::Add;
struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Speed(self.0 + rhs.0)
    }
}
impl Add<u32> for Speed {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Speed(self.0 + rhs)
    }
}

// ---

struct Letter(char);
impl Add<Self> for Letter {
    type Output = String;
    fn add(self, rhs: Self) -> Self::Output {
        format!("{}{}", self.0, rhs.0)
    }
}

pub fn intro3() {
    let l1 = Letter('a');
    let l2 = Letter('b');
    let sum = l1 + l2;
}

// ---

use std::ops::Index;

struct Hvac {
    current_temp: i16,
    max_temp: i16,
    min_temp: i16,
}

enum Temp {
    Current,
    Max,
    Min,
}

impl Index<Temp> for Hvac {
    type Output = i16;
    fn index(&self, temp: Temp) -> &Self::Output {
        match temp {
            Temp::Current => &self.current_temp,
            Temp::Max => &self.max_temp,
            Temp::Min => &self.min_temp,
        }
    }
}

pub fn intro4() {
    let env = Hvac {
        current_temp: 10,
        max_temp: 20,
        min_temp: 3,
    };

    let current = env[Temp::Current];
}
