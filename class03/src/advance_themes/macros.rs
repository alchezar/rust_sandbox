// IKinder

// -----------------------------------------------------------------------------
// Macros: Intro
// -----------------------------------------------------------------------------

#[derive(Debug)]
enum RoomType {
    Bedroom,
    Kitchen,
}
#[derive(Debug)]
struct Room {
    dimensions: (usize, usize),
    kind: RoomType,
}

pub fn intro() {
    // Assert
    let a = 1;
    let b = 2;
    assert!(a < b, "{} ne {}", a, b);
    assert_eq!(a, b, "values should be equal");
    assert_ne!(a, b, "values should not be equal");

    debug_assert!(a < b, "{} ne {}", a, b);
    debug_assert_eq!(a, b, "values should be equal");
    debug_assert_ne!(a, b, "values should not be equal");

    // Debug
    let kitchen = Room {
        dimensions: (20, 20),
        kind: RoomType::Kitchen,
    };
    dbg!(&kitchen);

    // Format
    let h = "Hello";
    let w = "World";
    let greet = format!("{}, {}!", h, w);
    println!("{}", greet);
}
