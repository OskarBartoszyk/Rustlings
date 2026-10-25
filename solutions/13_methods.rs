// ╔══════════════════════════════════════════════════════════╗
// ║  13 - METHODS (impl blocks)                              ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python classes mix data and behavior. Rust splits them: the struct holds
// data, and an `impl` block attaches functions to it.
//
//     impl Rectangle {
//         fn area(&self) -> u32 { self.width * self.height }   // a METHOD
//         fn square(size: u32) -> Self { ... }                  // an ASSOCIATED FN
//     }
//
// The first parameter decides what the method may do to the value:
//   &self      just look at it                (read-only borrow)
//   &mut self  look AND change it             (mutable borrow)
//   self       take it and consume it         (rare)
//   (no self)  not a method at all: an "associated function", called with
//              `Type::name()`. This is how constructors work. There's no
//              __init__; by convention you write a function called `new`.
//
// * `Self` (capital S) means "the type this impl is for". Lazy-friendly!
// * Call methods with a dot:  rect.area()   Rust borrows for you automatically.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Already written for you (a constructor):
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // 🎯 MISSION 1: Ferris is designing fish ponds. 🐟
    // A square pond is a rectangle whose sides are equal. Write the
    // constructor: both width and height are `size`.
    fn square(size: u32) -> Self {
        // [[
        Rectangle { width: size, height: size }
        // ]]
    }

    // 🎯 MISSION 2: how much water fits? Return width * height.
    fn area(&self) -> u32 {
        // [[
        self.width * self.height
        // ]]
    }

    // 🎯 MISSION 3: can this pond hold the OTHER pond inside it?
    // Only if self is strictly bigger than `other` in BOTH width and height.
    fn can_hold(&self, other: &Rectangle) -> bool {
        // [[
        self.width > other.width && self.height > other.height
        // ]]
    }

    // 🎯 MISSION 4: expansion time! Multiply both dimensions by `factor`
    // IN PLACE. (Look at the signature: `&mut self` means you may change fields.)
    fn scale(&mut self, factor: u32) {
        // [[
        self.width *= factor;
        self.height *= factor;
        // ]]
    }
}

fn main() {
    let r = Rectangle::new(3, 4);
    println!("{:?} square? {}", r, r.is_square());

    let sq = Rectangle::square(5);
    assert!(sq.is_square());
    assert_eq!(r.area(), 12);
    assert!(sq.can_hold(&r));
    assert!(!r.can_hold(&sq));

    let mut m = Rectangle::new(2, 3);
    m.scale(10);
    assert_eq!((m.width, m.height), (20, 30));
    println!("✅ Exercise 13 passed! The ponds are splendid. 🏞️");
}
