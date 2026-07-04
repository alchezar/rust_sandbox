// IKinder

pub fn main() {
    crate::show_name(file!());

    println!("---------------------");
    let x = MyStruct::new(666);
    {
        println!("---------------------");
        let y = MyStruct::new(42);
        println!("--- Ending scope.");
        println!("---------------------");
    }
    move_me(x);
    println!("--- Back from `move_me` function.");

    let has_drop = HasDroppable {
        x: MyStruct::new(1),
    };

    let mut my_num = SmartPointer::<i32>::new();
    my_num.set(12);
    println!("my_num = {}", my_num.get());

    println!("--- Ending `main` function.");
    println!("---------------------");
}

struct MyStruct {
    n: i32,
}
impl MyStruct {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self { n }
    }
}
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Destructing {}", self.n);
    }
}

fn move_me(x: MyStruct) {
    // Do nothing.
}

struct HasDroppable {
    x: MyStruct,
}

/// Smart pointer
struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: std::alloc::Layout,
}
impl<T> SmartPointer<T> {
    fn new() -> Self {
        println!("Allocating memory for SmartPointer");
        unsafe {
            let layout = std::alloc::Layout::new::<T>();
            let ptr = std::alloc::alloc(layout);
            Self {
                ptr,
                data: ptr as *mut T,
                layout,
            }
        }
    }
    fn set(&mut self, val: T) {
        unsafe {
            *self.data = val;
        }
    }
    fn get(&self) -> &T {
        unsafe { &*self.data }
        // unsafe { self.data.as_ref().unwrap() }
    }
}
impl<T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Deallocating memory from SmartPointer");
        unsafe {
            std::alloc::dealloc(self.ptr, self.layout);
        }
    }
}
