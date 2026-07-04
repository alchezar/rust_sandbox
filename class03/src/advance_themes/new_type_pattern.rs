// IKinder

// -----------------------------------------------------------------------------
// New type pattern: Demo
// -----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

impl NeverZero {
    fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("Zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    a / b.0
}

pub fn demo() {
    if let Ok(divider) = NeverZero::new(0) {
        println!("{}", divide(100, divider))
    }
}

// -----------------------------------------------------------------------------
// New type pattern: Activity
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}
#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}
#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoes_color(color: ShoesColor) {
    println!("Shoes color: {:?}", color)
}

fn print_shirt_color(color: ShirtColor) {
    println!("Shirt color: {:?}", color)
}

fn print_pants_color(color: PantsColor) {
    println!("Pants color: {:?}", color)
}

pub fn activity() {
    let shoes = ShoesColor::new(Color::Brown);
    let shirt = ShirtColor::new(Color::Gray);
    let pants = PantsColor::new(Color::Custom("Orange".to_owned()));

    print_shoes_color(shoes);
    print_shirt_color(shirt);
    print_pants_color(pants);
}
