// ╔══════════════════════════════════════════════════════════╗
// ║  16 - OPTION: Rust has no null!                          ║
// ╚══════════════════════════════════════════════════════════╝
//
// In Python any variable might secretly be `None`, and you only find out
// at 3 a.m. with  AttributeError: 'NoneType' object has no attribute...  😱
//
// Rust has NO null. "Maybe there's a value" is a plain enum:
//
//     enum Option<T> { Some(T), None }
//
// A function that might not find anything returns `Option<T>`, and the
// compiler FORCES you to handle `None` before touching the value.
//
// Ways to unwrap the box 🎁:
//   match opt { Some(v) => ..., None => ... }     the full version
//   if let Some(v) = opt { ... }                   when you only care about Some
//   opt.unwrap_or(default)                         value, or a fallback
//   opt.unwrap()                                   value, or CRASH (use sparingly!)
//   opt.map(|v| v + 1)                             change what's inside, if anything
//
// You'll see Options everywhere: vec.first(), vec.get(i), map.get(k),
// "abc".chars().next(), ...

fn demo() {
    let some: Option<i32> = Some(5);
    let none: Option<i32> = None;

    match some {
        Some(v) => println!("got {v}"),
        None => println!("nothing"),
    }
    if let Some(v) = none {
        println!("never printed {v}");
    }
    println!("{}", none.unwrap_or(0));
    println!("{:?}", some.map(|v| v * 2));
}

// 🎯 MISSION 1
// Ferris is looking for the first EVEN-numbered fish tank. 🐠 Return
// `Some(number)` for the first even number in the slice, or `None` if there
// isn't one. (Loop over `&x in v`, `return Some(x)` early, `None` at the end.)
fn find_first_even(v: &[i32]) -> Option<i32> {
    todo!() // <-- replace this with your code
}

// 🎯 MISSION 2
// A customer may or may not have ordered. If they did, DOUBLE the order,
// otherwise stay `None`. Use a `match` with both arms:
//   Some(v) => ...,   None => ...
fn double_opt(x: Option<i32>) -> Option<i32> {
    todo!() // <-- replace this with your code
}

// 🎯 MISSION 3
// Give back the number inside, or -1 if there is nothing. This time use
// `if let Some(v) = x { ... } else { ... }` (no match!).
fn value_or_minus_one(x: Option<i32>) -> i32 {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    assert_eq!(find_first_even(&[1, 3, 4, 6]), Some(4));
    assert_eq!(find_first_even(&[1, 3]), None);
    assert_eq!(double_opt(Some(4)), Some(8));
    assert_eq!(double_opt(None), None);
    assert_eq!(value_or_minus_one(Some(9)), 9);
    assert_eq!(value_or_minus_one(None), -1);
    println!("✅ Exercise 16 passed! Not a single null pointer in sight. 🚫");
}
