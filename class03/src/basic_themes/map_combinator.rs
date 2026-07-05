// IKinder

#![allow(clippy::explicit_auto_deref)]

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}
impl User {
    fn new(user_id: i32, name: &str) -> Self {
        Self {
            user_id,
            name: name.to_owned(),
        }
    }
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn map_combinators() {
    let names: Vec<&str> = vec!["Sam", "Matt", "Katie", "John"];
    let users: Vec<Option<User>> = names
        .iter()
        .map(|name| find_user(name).map(|id| User::new(id, *name)))
        .collect();

    for user in users.iter() {
        match user {
            Some(u) => println!("Found user: {:?}", u),
            None => println!("User not found"),
        }
    }
}

pub fn run() {}
