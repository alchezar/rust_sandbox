// IKinder

use std::fmt;

pub fn main() {
    crate::show_name(file!());

    lifetimes();
    display_debug();
    traits_runtime();
    trains_compiletime();
    operators();
}

/// ----------------------------------------------------------------------------

fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    println!("{}", *i);
    if *i > *j { i } else { j }
}

#[derive(Debug)]
struct Cat(String);
impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0)
    }
}
struct CatFeeder<'a> {
    cat: &'a mut Cat,
}
impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn lifetimes() {
    let n = 12;
    let r = borrow(&n, &10);

    let mut cats = vec![Cat("Frodo".into()), Cat("Bilbo".into())];
    let mut feeders = cats
        .iter_mut()
        .map(|cat| CatFeeder { cat })
        .collect::<Vec<_>>();
    println!("-----");
}

/// ----------------------------------------------------------------------------

struct Point {
    x: i32,
    y: i32,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let debug_self = self as &dyn fmt::Debug;
        debug_self.fmt(f)
    }
}

fn display_debug() {
    let point = Point { x: 1, y: 2 };
    println!("{:?}", point);
    println!("{}", point);
    println!("-----");
}

/// ----------------------------------------------------------------------------

trait Animal: fmt::Debug {
    fn speak(&self);
}
#[derive(Debug)]
struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        print!("Woof ");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        print!("Meow ");
    }
}

fn speak_twice(animal: &impl Animal) {
    print!("{:?}: ", animal);
    animal.speak();
    animal.speak();
}

fn make_animal() -> impl Animal {
    Dog {}
}

fn traits_runtime() {
    let cat = Cat("Fidel".into());
    let dog = Dog;

    cat.speak();
    dog.speak();

    speak_twice(&cat);
    speak_twice(&dog);

    let animal = make_animal();
    speak_twice(&animal);

    println!();
    let mut animals = Vec::<Box<dyn Animal>>::new();
    animals.push(Box::new(cat));
    animals.push(Box::new(dog));
    animals.push(Box::new(animal));
    animals.iter().for_each(|a| a.speak());
    println!("\n-----");
}

/// ----------------------------------------------------------------------------

trait Downcastable {
    fn as_any(&self) -> &dyn std::any::Any;
}
impl Downcastable for Cat {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Downcastable for Dog {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
struct Tortoise;
impl Animal for Tortoise {
    fn speak(&self) {
        print!("hkhkhkhk ")
    }
}
impl Downcastable for Tortoise {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn trains_compiletime() {
    let cat = Cat("Fidel".into());
    let dog = Dog;
    let tortoise = Tortoise;
    let animal = make_animal();

    let mut animals = Vec::<Box<dyn Downcastable>>::new();
    animals.push(Box::new(cat));
    animals.push(Box::new(dog));
    animals.push(Box::new(tortoise));

    if let Some(tortoise) = animals
        .iter()
        .find_map(|d| d.as_any().downcast_ref::<Tortoise>())
    {
        println!("I'm tortoise!")
    }
    println!("-----");
}

/// ----------------------------------------------------------------------------

#[derive(Debug)]
struct FloatPoint {
    x: f32,
    y: f32,
}
impl std::ops::Add for FloatPoint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn operators() {
    let a = FloatPoint { x: 1.0, y: 2.0 };
    let b = FloatPoint { x: 3.0, y: 4.0 };
    let c = a + b;

    println!("{:?}", c);
}
