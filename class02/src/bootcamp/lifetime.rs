// IKinder

#![allow(dead_code, unused_variables, clippy::needless_lifetimes)]

fn example_one() {
    let player1 = String::from("Player 1");
    let player2 = String::from("Player 2");

    let result = first_turn(player1.as_str(), player2.as_str());
    println!("Player going first is: {}", result);
}

fn example_two() {
    let player1 = String::from("Player 1");
    let result;
    {
        let player2 = String::from("Player 2");
        result = first_turn(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    }
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

fn second_turn(p1: &str, p2: &str) -> &'static str {
    let s: &'static str = "example";
    s
}

// ---

struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

pub fn run() {
    let mut tweet = Tweet {
        content: "Hello World!",
    };

    let old_content = tweet.replace_content("Hi World!");
    println!("{}", old_content);
    println!("{}", tweet.content);
}

fn take_and_return_content<'a, 'b>(content: &'a str, content2: &'b str) -> &'a str {
    content
}
