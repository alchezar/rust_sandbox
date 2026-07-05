// IKinder

#![allow(
    clippy::redundant_closure,
    clippy::redundant_guards,
    clippy::useless_vec
)]

// -----------------------------------------------------------------------------
// Match guards: Demo
// -----------------------------------------------------------------------------

use std::cmp::PartialEq;

enum Status {
    Error(i32),
    Info,
    Warn,
}

pub fn demo1() {
    let status = Status::Error(5);
    match status {
        Status::Error(s @ 3) => println!("Error three"),
        Status::Error(s @ 5..=6) => println!("Error 5-6"),
        Status::Error(s @ 10 | s @ 19) => println!("Error 10|19"),
        Status::Error(s) => println!("Error: {}", s),
        Status::Warn => println!("Warn"),
        Status::Info => println!("Info"),
    }
}

enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird {
    age: usize,
    species: Species,
}

pub fn demo2() {
    let hawk = Bird {
        age: 13,
        species: Species::Hawk,
    };

    match hawk {
        Bird { age: 4, .. } => println!("4 year old bird"),
        Bird {
            age: 4..=10 | 15..=20,
            ..
        } => println!("4-10 or 15-20 year old bird"),
        Bird {
            species: Species::Finch,
            ..
        } => println!("Finch"),
        Bird { .. } => println!("Other bird"),
    }
}

#[derive(PartialEq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub fn demo3() {
    let stage = 5;
    let diff = Difficulty::Normal;
    match stage {
        s if s == 5 && diff == Difficulty::Easy => println!("Easy mode stage {}", 5),
        s if diff == Difficulty::Normal => println!("Normal mode stage {}", 5),
        s @ 10 | s @ 15 => println!("Stage 10|15"),
        s => println!("stage {}", s),
    }
}

struct Vehicle {
    km: usize,
    year: usize,
}

pub fn demo4() {
    let car = Vehicle {
        year: 2006,
        km: 800_000,
    };

    match car {
        Vehicle { km, year } if km == 0 && year >= 2020 => println!("new 2020 car"),
        Vehicle { km, .. } if km <= 50_000 => println!("under 50k km"),
        Vehicle { km, .. } if km >= 600_000 => println!("at leas huge as fuck"),
        Vehicle { year, .. } if year == 2006 => println!("old as fuck"),
        Vehicle { .. } => println!("Other mileage"),
    }
}

// -----------------------------------------------------------------------------
// Match guards: Activity
// -----------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug, PartialEq)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: &Tile) {
    use BrickStyle::*;
    use Tile::*;

    match tile {
        Brick(brick @ Gray | brick @ Red) => println!("The brick color is {brick:?}"),
        Brick(other) => println!("{other:?} brick"),
        Grass | Dirt | Sand => println!("Ground tile"),
        Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: 100..,
        }) => println!("Lots of gold!"),
        Water(pressure) if pressure.0 < 10 => println!("Water pressure level: {}", pressure.0),
        Water(high) => println!("High water pressure!"),
        other => println!("any message for {other:?}"),
    }
}

pub fn activity() {
    use BrickStyle::*;
    use Tile::*;
    use TreasureItem::*;

    vec![
        Brick(Red),
        Brick(Gray),
        Brick(Dungeon),
        Dirt,
        Grass,
        Sand,
        Wood,
        Treasure(TreasureChest {
            content: Gold,
            amount: 600,
        }),
        Treasure(TreasureChest {
            content: SuperPower,
            amount: 3,
        }),
        Water(Pressure(20)),
        Water(Pressure(5)),
    ]
    .iter()
    .for_each(|tile| print_tile(tile));
}
