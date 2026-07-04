// IKinder

pub fn if_let() {
    let maybe_user = Some("Kinder");
    match maybe_user {
        Some(user) => println!("user: {}", user),
        None => println!("no user"),
    }

    let maybe_user: Option<&str> = None;
    if let Some(user) = maybe_user {
        println!("user: {}", user);
    } else {
        println!("no user");
    }

    enum Color {
        Red,
        Green,
        Blue,
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("it's red");
    }
}

pub fn while_let() {
    let mut data = Some(3);
    while let Some(i) = data {
        println!("loop");
        data = None;
    }

    let numbers = vec![1, 2, 3];
    let mut numbers_iter = numbers.iter();
    while let Some(num) = numbers_iter.next() {
        println!("num: {num}")
    }
}
