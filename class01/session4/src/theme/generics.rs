// IKinder

pub fn main() {
    crate::show_name(file!());

    simple_generics();
    from_into();
    bucket();
}

fn just_print_it<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

fn just_print_two<T, U>(s1: T, s2: U)
where
    T: ToString,
    U: ToString,
{
    println!("{} {}", s1.to_string(), s2.to_string());
}

fn simple_generics() {
    just_print_it("Hello!");
    just_print_it(666);
}

/// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
struct Degrees(f32);
#[derive(Debug, Copy, Clone)]
struct Radians(f32);
impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Self(rad.0 * 180.0 / std::f32::consts::PI)
    }
}
impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Self(deg.0 * std::f32::consts::PI / 180.0)
    }
}

fn sin(angle: impl Into<Radians>) -> f32 {
    let angle = angle.into();
    angle.0.sin()
}

fn from_into() {
    let behind = Degrees(180.0);
    let mut behind_rad = Radians::from(behind);
    println!("{:?}", behind_rad);
    behind_rad = behind.into();
    println!("{:?}", behind_rad)
}

/// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct HashMapBucket<K, V> {
    pub map: std::collections::HashMap<K, Vec<V>>,
}
impl<K, V> HashMapBucket<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: K, val: V) {
        let vec = self.map.entry(key).or_insert(Vec::new());
        vec.push(val)
    }
}

fn bucket() {
    let mut bucket = HashMapBucket::new();
    bucket.insert("Hello", 1);
    bucket.insert("Hello", 2);
    bucket.insert("World", 3);
    println!("{:?}", bucket);
}
