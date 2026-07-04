// IKinder

pub fn main() {
    crate::show_name(file!());

    println!("{}", size_of::<OneByte>());
    println!("{}", size_of::<TwoByte>());
    println!("{}", size_of::<ThreeByte>());
    println!("{}", size_of::<FourByte>());
}

struct OneByte {
    a: u8,
}

struct TwoByte {
    a: u16,
}

#[repr(C)]
// #[repr(packed)]
struct ThreeByte {
    a: u16,
    b: u8,
}

struct FourByte {
    a: u16,
    b: u16,
}

#[unsafe(no_mangle)]
unsafe fn no_manged_function() {}
