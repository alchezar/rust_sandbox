// IKinder

#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::iter::Iterator;

/// Iterator pattern 1

// trait Iterator {
// 	type Item;
// 	fn next(&mut self) -> Option<Self::Item>;
// }
//
// struct MyStruct {}
//
// impl Iterator for MyStruct {
// 	type Item = String;
// 	fn next(&mut self) -> Option<Self::Item> {
// 		None
// 	}
// }

/// Iterator pattern 2

struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }
        Some(self.values.remove(0))
    }
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.first_name, self.last_name, self.occupation].into_iter()
    }
}

/// Combinators
#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

pub fn run() {
    // Iterator pattern 1
    // let mut m = MyStruct {};
    // let item = m.next();

    // Iterator pattern 2
    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Chill".to_owned(),
        occupation: "Programmer".to_owned(),
    };

    for item in p {
        println!("{item}");
    }

    // Iterator pattern 3
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 4);
    scores.insert("green team".to_owned(), 3);

    let mut scores_iter = (&scores).into_iter();
    let first = scores_iter.next();

    for (team, score) in &mut scores {
        println!("{team} got: {score} points");
    }

    // Combinators
    let students = vec!["Bogdan 3.1", "Kyle 3.9", "Wallace 2.3"];
    combinators_nested_ifs(&students);
    combinators_base(&students);
    combinators_shorter(&students);
}

fn combinators_nested_ifs(students: &Vec<&str>) {
    println!("--- nested if's ---");

    let mut good_students = vec![];
    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>();

            if let Ok(gpa) = gpa {
                if gpa >= 3.0 {
                    good_students.push(Student { name, gpa });
                }
            }
        }
    }

    for s in good_students {
        println!("{:?}", s);
    }
}

/// Approach 2, combinators
fn combinators_base(students: &Vec<&str>) {
    println!("--- combinators ---");

    let good_students = students
        .iter()
        .map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            Some(Student { name, gpa })
        })
        .flatten()
        .filter(|s| s.gpa >= 3.0)
        .collect::<Vec<Student>>();

    for s in &good_students {
        println!("{:?}", s);
    }
}

/// Approach 3, filter map
fn combinators_shorter(students: &Vec<&str>) {
    println!("--- filter-map ---");

    let lambda = |s: &&str| {
        let mut s = s.split(' ');
        let name = s.next()?.to_owned();
        let gpa = s.next()?.parse::<f32>().ok()?;

        if gpa < 3.0 {
            return None;
        }

        Some(Student { name, gpa })
    };

    let good_students = students.iter().filter_map(lambda).collect::<Vec<Student>>();

    for s in &good_students {
        println!("{:?}", s);
    }
}
