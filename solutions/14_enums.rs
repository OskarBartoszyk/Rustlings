// ╔══════════════════════════════════════════════════════════╗
// ║  14 - ENUMS (the superpower Python doesn't have)         ║
// ╚══════════════════════════════════════════════════════════╝
//
// A Python Enum is a list of named constants. A Rust `enum` is MUCH stronger:
// every variant can CARRY ITS OWN DATA.
//
//     enum Shape {
//         Circle(f64),                  // a circle carries a radius
//         Rect { w: f64, h: f64 },      // a rectangle carries two numbers
//         Point,                        // a point carries nothing
//     }
//     let s = Shape::Circle(2.0);
//
// A `Shape` is EXACTLY ONE of those variants at a time. So you can model
// "this OR that OR the other" precisely, and impossible states (like a circle
// with a width) simply cannot exist. 🎯
//
// Enums can have `impl` blocks too. To look inside one you use `match`,
// which is the next lesson. For now: just build them.

#[derive(Debug, PartialEq)]
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Point,
}

enum Direction {
    Up,
    Down,
}

fn demo() {
    let a = Shape::Circle(1.5);
    let b = Shape::Rect { w: 2.0, h: 3.0 };
    let c = Shape::Point;
    println!("{:?} {:?} {:?}", a, b, c);
    let _d = Direction::Up;
    let _e = Direction::Down;
}

// 🎯 MISSIONS
// Ferris runs a cookie-cutter workshop. 🍪 Customers order shapes, but the
// factory functions below are empty. Build the right variant in each:

// A round cookie of the given radius.
fn circle(radius: f64) -> Shape {
    // [[
    Shape::Circle(radius)
    // ]]
}

// A rectangular cookie (struct-like variant: use `{ w, h }`).
fn rect(w: f64, h: f64) -> Shape {
    // [[
    Shape::Rect { w, h }
    // ]]
}

// A single crumb. It has no data at all.
fn point() -> Shape {
    // [[
    Shape::Point
    // ]]
}

fn main() {
    demo();
    assert_eq!(circle(1.0), Shape::Circle(1.0));
    assert_eq!(rect(2.0, 3.0), Shape::Rect { w: 2.0, h: 3.0 });
    assert_eq!(point(), Shape::Point);
    println!("✅ Exercise 14 passed! The cookies are baking. 🍪");
}
