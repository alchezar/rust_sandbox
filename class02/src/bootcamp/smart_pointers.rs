// IKinder

#![allow(dead_code, unused_variables)]

// ----------------------------
// Box pointer (c++ unique_ptr)
// ----------------------------

trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Container {}

pub fn run1() {
    let button_a = Button {
        text: "button a ".to_owned(),
    };
    let button_b = Box::new(Button {
        text: "button b ".to_owned(),
    });

    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}

// ----------------------------------------
// Reference count pointer (c++ shared_ptr)
// ----------------------------------------

use std::rc::Rc;

struct Database {
    max_connections: u32,
}
struct AuthService {
    db: Rc<Database>,
}
struct ContentService {
    db: Rc<Database>,
}

pub fn run2() {
    let db = Rc::new(Database {
        max_connections: 100,
    });
    let auth_service = AuthService { db: Rc::clone(&db) };
    let contents_service = ContentService { db: Rc::clone(&db) };
}

// -----------------------
// Ref cell smart pointer.
// -----------------------

use std::cell::RefCell;

struct Database1 {
    max_connections: u32,
}
struct AuthService1 {
    db: Rc<RefCell<Database1>>,
}
struct ContentService1 {
    db: Rc<RefCell<Database1>>,
}

pub fn run() {
    let db = Rc::new(RefCell::new(Database1 {
        max_connections: 100,
    }));
    let auth_service = AuthService1 { db: Rc::clone(&db) };
    let contents_service = ContentService1 { db: Rc::clone(&db) };

    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut();
    r1.max_connections = 200;
}
