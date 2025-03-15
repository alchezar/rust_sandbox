//  IKinder
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

mod themes;

fn main() {
	themes::modules::run();
}

fn all_aps(word: &str) -> String {
	word.to_uppercase()
}
