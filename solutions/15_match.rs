// ╔══════════════════════════════════════════════════════════╗
// ║  15 - MATCH                                              ║
// ╚══════════════════════════════════════════════════════════╝
//
// `match` is like a super-powered if/else chain. And it is EXHAUSTIVE: the
// compiler makes sure you handled EVERY possible case. Forget one variant
// and your program won't even compile. (Goodbye, forgotten `else` branches!)
//
//     match shape {
//         Shape::Circle(r)     => 3.14 * r * r,    // `r` grabs the data inside
//         Shape::Rect { w, h } => w * h,
//         Shape::Point         => 0.0,
//     }
//
// * `match` is an expression, so it returns a value.
// * `_` means "everything else".
// * Patterns:  1 | 2 (either),  3..=9 (range),  n if n < 0 (a "guard" = extra condition)
// * Matching through a reference (&Shape) works the same way.

use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Point,
}

fn demo() {
    let n = 7;
    let text = match n {
        0 => "zero",
        1 | 2 => "one or two",
        3..=9 => "small",
        _ => "big",
    };
    println!("{text}");
}

// 🎯 MISSION 1
// The cookie workshop needs to know how much dough each cookie uses. 🍪
// Oh no, `area` is empty! Write a `match` over the shape:
//   Circle(r)      -> PI * r * r
//   Rect { w, h }  -> w * h
//   Point          -> 0.0
fn area(shape: &Shape) -> f64 {
    // [[
    match shape {
        Shape::Circle(r) => PI * r * r,
        Shape::Rect { w, h } => w * h,
        Shape::Point => 0.0,
    }
    // ]]
}

// 🎯 MISSION 2
// Ferris describes the temperature outside using one word:
//   below zero -> "negative"   (hint: a guard! `x if x < 0 => ...`)
//   exactly 0  -> "zero"
//   1 to 9     -> "small"      (hint: range `1..=9`)
//   otherwise  -> "big"        (hint: `_`)
// Order matters: patterns are tried top to bottom.
fn describe(n: i32) -> &'static str {
    // [[
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        1..=9 => "small",
        _ => "big",
    }
    // ]]
}

fn main() {
    demo();
    assert_eq!(area(&Shape::Rect { w: 2.0, h: 3.0 }), 6.0);
    assert_eq!(area(&Shape::Point), 0.0);
    assert!((area(&Shape::Circle(1.0)) - PI).abs() < 1e-9);
    assert_eq!(describe(-5), "negative");
    assert_eq!(describe(0), "zero");
    assert_eq!(describe(5), "small");
    assert_eq!(describe(50), "big");
    println!("✅ Exercise 15 passed! Every case covered. 🧥");
}
