// IKinder

// -----------------------------------------------------------------------------
// Iterator: Intro
// -----------------------------------------------------------------------------

// trait Iterator {
// 	type Item;
// 	fn next(&mut self) -> Option<Self::Item>;
// }

struct Odd {
    number: isize,
    max: isize,
}

impl Odd {
    fn new(max: isize) -> Self {
        Self { number: -1, max }
    }
}

impl Iterator for Odd {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.number += 2;
        if self.number <= self.max {
            Some(self.number)
        } else {
            None
        }
    }
}

pub fn intro() {
    let mut odds = Odd::new(7);

    while let Some(i) = odds.next() {
        println!("{i:?}");
    }

    let mut odds = Odd::new(7);
    for i in odds.map(|o| o + 1) {
        println!("{i:?}");
    }
}

// -----------------------------------------------------------------------------
// Into Iterator: Intro
// -----------------------------------------------------------------------------

// trait IntoIterator {
// 	type Item;
// 	type IntoIter;
// 	fn into_iter(self) -> Self::IntoIter;
// }

#[derive(Clone)]
struct Friends {
    names: Vec<String>,
}

impl Friends {
    fn iter(&self) -> std::slice::Iter<'_, String> {
        self.names.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, String> {
        self.names.iter_mut()
    }
}

// Ability to iterate through the owned vec.
impl IntoIterator for Friends {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}
// Ability to iterate through the borrowed vec.
impl<'a> IntoIterator for &'a Friends {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}
// Ability to iterate through the mutable borrowed vec.
impl<'a> IntoIterator for &'a mut Friends {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}

pub fn intro1() {
    let mut friends = Friends {
        names: vec!["Ivan".to_owned(), "John".to_owned(), "Martin".to_owned()],
    };
    let mut friends1 = friends.clone();

    for f in &mut friends {
        println!("{f:?}");
    }
    for f in &friends {
        println!("{f:?}");
    }
    for f in friends {
        println!("{f:?}");
    }

    for f in friends1.iter_mut() {
        println!("{f:?}");
    }
    for f in friends1.iter() {
        println!("{f:?}");
    }
}

// -----------------------------------------------------------------------------
// Into Iterator: Demo
// -----------------------------------------------------------------------------

use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Fruit {
    Apple,
    Banana,
    Orange,
}

struct FruitStand {
    fruit: HashMap<Fruit, u32>,
}

impl IntoIterator for FruitStand {
    type Item = (Fruit, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.into_iter()
    }
}
impl<'a> IntoIterator for &'a FruitStand {
    type Item = (&'a Fruit, &'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter()
    }
}
impl<'a> IntoIterator for &'a mut FruitStand {
    type Item = (&'a Fruit, &'a mut u32);
    type IntoIter = std::collections::hash_map::IterMut<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter_mut()
    }
}

pub fn demo() {
    let mut fruit = HashMap::new();
    fruit.insert(Fruit::Apple, 1);
    fruit.insert(Fruit::Banana, 2);
    fruit.insert(Fruit::Orange, 3);

    let fruit = fruit;
    let mut store = FruitStand { fruit };

    for (fruit, stock) in &store {
        println!("{fruit:?}: {stock:?}");
    }
    for (fruit, stock) in &mut store {
        *stock += 1;
        println!("{fruit:?}: {stock:?}");
    }
    for (fruit, stock) in store.into_iter() {
        println!("{fruit:?}: {stock:?}");
    }
}

// -----------------------------------------------------------------------------
// Iterator: Activity
// -----------------------------------------------------------------------------

// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game power-ups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 power-up obtained), 5, 7, 9, ...
//
// Requirements:
// * 1. Write a program that uses an iterator to generate a score multiplier
// * 2. The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through power-ups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct ScoreMultiplier {
    amount: usize,
    per_iter_amount: usize,
    per_iter_bonus: usize,
}

impl Default for ScoreMultiplier {
    fn default() -> Self {
        Self {
            amount: 0,
            per_iter_amount: 1,
            per_iter_bonus: 0,
        }
    }
}

impl Iterator for ScoreMultiplier {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.amount += self.per_iter_amount + self.per_iter_bonus;
        Some(self.amount)
    }
}

pub fn activity() {
    let mut multiplier = ScoreMultiplier::default();

    for _ in 0..3 {
        match multiplier.next() {
            Some(amount) => println!("{amount:?}"),
            None => break,
        }
    }
    multiplier.per_iter_bonus = 2;

    for _ in 0..3 {
        match multiplier.next() {
            Some(amount) => println!("{amount:?}"),
            None => break,
        }
    }
}
// -----------------------------------------------------------------------------
// Into Iterator: Activity
// -----------------------------------------------------------------------------

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Iterate through moved data.

struct ColorIntoIter {
    color: Color,
    position: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 10,
            g: 20,
            b: 30,
        }
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = ColorIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        ColorIntoIter {
            color: self,
            position: 0,
        }
    }
}

impl Iterator for ColorIntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };
        self.position += 1;
        next
    }
}

// Iterate through borrowed data.

struct ColorIter<'a> {
    color: &'a Color,
    position: u8,
}

impl<'a> IntoIterator for &'a Color {
    type Item = u8;
    type IntoIter = ColorIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ColorIter {
            color: &self,
            position: 0,
        }
    }
}

impl<'a> Iterator for ColorIter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };
        self.position += 1;
        next
    }
}

pub fn activity1() {
    let color = Color::default();

    // Borrowed
    for c in &color {
        println!("{c:?}")
    }
    // Moved
    for c in color {
        println!("{c:?}")
    }
}
