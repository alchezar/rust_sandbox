// IKinder

pub fn run() {
    let languages = vec![
        "c++".to_owned(),
        "rust".to_owned(),
        "swift".to_owned(),
        "python".to_owned(),
    ];

    let next = next_lang(&languages, "c++");
    println!("{:?}", next);
    let last = last_lang(&languages);
    println!("{:?}", last);
    let longest = longest_lang(&languages.get(0).unwrap(), &languages.get(1).unwrap());
    println!("{:?}", longest);
}

fn next_lang<'a>(items: &'a [String], current: &str) -> &'a str {
    items
        .iter()
        .position(|item| item == current)
        .map_or(items.last(), |position| items.get(position + 1))
        .unwrap()
}

fn last_lang(items: &[String]) -> &str {
    items.last().unwrap()
}

fn longest_lang<'a>(left: &'a String, right: &'a String) -> &'a str {
    if left.len() > right.len() {
        left
    } else {
        right
    }
}
