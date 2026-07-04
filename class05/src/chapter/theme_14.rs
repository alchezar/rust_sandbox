// IKinder

pub mod _2 {
    pub mod _2 {
        pub mod front_of_house {
            pub mod hosting {
                pub fn add_to_waitlist() {}
                pub fn seat_at_table() -> String {
                    "sit down please".into()
                }
            }
            pub mod serving {
                fn take_order() {}
                pub fn serve_order() {}
                fn take_payment() {}
                fn complain() {}
            }
        }
        pub fn eat_at_restaurant() -> String {
            front_of_house::hosting::add_to_waitlist();
            back_of_house::cook_order();
            "yummy yummy!".into()
        }
        mod back_of_house {
            pub fn cook_order() {}
            fn fix_incorrect_order() {
                use super::front_of_house::serving;

                cook_order();
                serving::serve_order();
            }
        }
        pub fn main() {
            assert_eq!(eat_at_restaurant().as_str(), "yummy yummy!");
            assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
        }
    }
    pub mod _3 {
        use std::collections::{BTreeMap, HashMap, HashSet};
        use std::fmt::Result as FmtResult;
        use std::io::Result as IoResult;

        pub fn main() {
            let _c1: HashMap<&str, i32> = HashMap::new();
            let mut c2 = BTreeMap::new();
            c2.insert(1, "a");
            let _c3: HashSet<i32> = HashSet::new();
        }
    }
    const ZERO: i32 = 0;
}
