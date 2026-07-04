// IKinder

#![allow(dead_code, unused_variables)]

use std::ops::Deref;

struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

pub fn run() {
    let validator1 =
        |username: &str, password: &str| -> bool { !username.is_empty() && !password.is_empty() };
    let validator2 = |username: &str, password: &str| -> bool {
        !password.is_empty()
            && !username.is_empty()
            && password.len() >= 6
            && password.contains(['#', '@', '(', '%', '?', '$', '0'])
    };
    let creds = Credentials {
        username: "Ivan".to_owned(),
        password: "Howard@".to_owned(),
        validator: validator2,
    };
    println!("{}", creds.is_valid());

    let password_validator = get_password_validator(8, true);
    let default_creds = get_default_creds(password_validator.deref());

    // ---
    let greater_than = |x: &i32| *x > 10;
    //let less_than = |x: &i32| *x < 20;
    println!("{}", are_both_true(greater_than, less_than, &10));
    println!("{}", are_both_fn_true(greater_than, less_than, &11));
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

fn get_default_creds<T>(f: T) -> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    Credentials {
        username: "guest".to_owned(),
        password: "password123".to_owned(),
        validator: f,
    }
}

fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            !password.len() >= min_len && password.contains(['#', '@', '(', '%', '?', '$', '0'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !password.len() >= min_len)
    }
}

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
where
    T: Fn(&V) -> bool,
    U: Fn(&V) -> bool,
{
    f1(item) && f2(item)
}

fn are_both_fn_true<T>(f1: fn(&T) -> bool, f2: fn(&T) -> bool, item: &T) -> bool {
    f1(item) && f2(item)
}

fn less_than(x: &i32) -> bool {
    *x < 20
}
