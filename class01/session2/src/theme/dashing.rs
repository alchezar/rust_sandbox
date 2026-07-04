// IKinder

use dashmap::DashMap;
use once_cell::sync::Lazy;

static SHARED: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

pub fn main() {
    crate::show_name(file!());

    for num in 0..100 {
        std::thread::spawn(move || {
            loop {
                if let Some(mut value) = SHARED.get_mut(&num) {
                    *value += 1;
                } else {
                    SHARED.insert(num, num);
                }
            }
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("{:#?}", SHARED);
}
