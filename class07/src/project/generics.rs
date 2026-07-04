// IKinder

use num_traits::ToPrimitive;
use num_traits::real::Real;

pub fn run() {
    let a = 3.0f32;
    let b = 4.0f64;
    let c = a.to_f64().unwrap() + b;

    let d = solve(a, b);
    let d = solve(100, 3.0);
}

fn solve<T, U>(a: T, b: U) -> f64
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}
