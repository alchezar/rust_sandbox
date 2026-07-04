// IKinder

// -----------------------------------------------------------------------------
// Trait Objects: Intro
// -----------------------------------------------------------------------------

trait Clicky {
    fn click(&self);
}

struct Keyboard;
impl Clicky for Keyboard {
    fn click(&self) {
        println!("thock")
    }
}

fn borrow_clicky(obj: &dyn Clicky) {
    obj.click();
}

fn move_clicky(obj: Box<dyn Clicky>) {
    obj.click();
}

pub fn intro1() {
    let key = Keyboard;
    let key1: &dyn Clicky = &key;
    let key2: &dyn Clicky = &Keyboard;
    let key3: Box<dyn Clicky> = Box::new(Keyboard);

    borrow_clicky(&key);
    borrow_clicky(key1);
    borrow_clicky(key2);
    borrow_clicky(&*key3);

    let keys = vec![key1, key2, &*key3];
    for key in keys {
        borrow_clicky(key)
    }

    move_clicky(key3)
}

struct Mouse;
impl Clicky for Mouse {
    fn click(&self) {
        println!("click")
    }
}

fn make_clicks(clickers: Vec<Box<dyn Clicky>>) {
    for clicker in clickers {
        clicker.click();
    }
}

pub fn intro2() {
    let key: Box<dyn Clicky> = Box::new(Keyboard);
    let mouse: Box<dyn Clicky> = Box::new(Mouse);
    let clickers = vec![key, mouse];
    make_clicks(clickers);

    let key = Box::new(Keyboard);
    let mouse = Box::new(Mouse);
    let clickers: Vec<Box<dyn Clicky>> = vec![key, mouse];
    make_clicks(clickers);
}

// -----------------------------------------------------------------------------
// Trait Objects: Demo
// -----------------------------------------------------------------------------

trait Sale {
    fn amount(&self) -> f32;
}

struct FullSale(f32);
impl Sale for FullSale {
    fn amount(&self) -> f32 {
        self.0
    }
}
struct OneDollarOffCoupon(f32);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f32 {
        self.0 - 1.0
    }
}
struct TenPercentOffPromo(f32);
impl Sale for TenPercentOffPromo {
    fn amount(&self) -> f32 {
        self.0 * 0.9
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f32 {
    sales.iter().map(|sale| sale.amount()).sum()
}

pub fn demo() {
    let price = 20.0;

    let sale1 = Box::new(FullSale(price));
    let sale2 = Box::new(OneDollarOffCoupon(price));
    let sale3 = Box::new(TenPercentOffPromo(price));

    let sales: Vec<Box<dyn Sale>> = vec![sale1, sale2, sale3];
    println!("total revenue is {}", calculate_revenue(&sales));
}

// -----------------------------------------------------------------------------
// Trait Objects: Activity
// -----------------------------------------------------------------------------

trait Material {
    fn cost_per_square_meter(&self) -> f32;
    fn square_meters(&self) -> f32;

    fn total_cost(&self) -> f32 {
        self.cost_per_square_meter() * self.square_meters()
    }
}

struct Carpet(f32);
impl Material for Carpet {
    fn cost_per_square_meter(&self) -> f32 {
        10.0
    }
    fn square_meters(&self) -> f32 {
        self.0
    }
}
struct Tile(f32);
impl Material for Tile {
    fn cost_per_square_meter(&self) -> f32 {
        15.0
    }
    fn square_meters(&self) -> f32 {
        self.0
    }
}
struct Wood(f32);
impl Material for Wood {
    fn cost_per_square_meter(&self) -> f32 {
        20.0
    }
    fn square_meters(&self) -> f32 {
        self.0
    }
}

fn total_cost(materials: &Vec<Box<dyn Material>>) -> f32 {
    materials.iter().map(|mat| mat.total_cost()).sum()
}

pub fn activity() {
    let carpet = Box::new(Carpet(20.0));
    let tile = Box::new(Tile(10.0));
    let wood = Box::new(Wood(30.0));

    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    println!("cost of all materials is {}", total_cost(&materials))
}
