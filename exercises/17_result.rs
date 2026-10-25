// ╔══════════════════════════════════════════════════════════╗
// ║  17 - RESULT AND THE `?` OPERATOR                        ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python handles errors with exceptions: something blows up and flies up the
// call stack unless someone catches it. Rust has NO exceptions. A function
// that can fail says so in its return type:
//
//     enum Result<T, E> { Ok(T), Err(E) }
//
//     "42".parse::<i32>()   ->  Ok(42)
//     "abc".parse::<i32>()  ->  Err(ParseIntError)
//
// You can't ignore an error by accident: the compiler nags if you drop a Result.
//
// * match / if let work just like with Option.
// * `.unwrap()` / `.expect("msg")` crash on Err. Fine for tests, bad for real code.
// * THE `?` OPERATOR ✨ - inside a function that returns Result:
//        let n = s.parse::<i32>()?;
//   means: "if this is an Err, return that error from MY function right now;
//   if it is Ok, hand me the value". It's exception-style propagation, but
//   visible in the types.
// * `panic!` is for "this should never happen" bugs, not for everyday errors.

use std::num::ParseIntError;

fn demo() {
    match "42".parse::<i32>() {
        Ok(n) => println!("parsed {n}"),
        Err(e) => println!("error: {e}"),
    }
    let bad: Result<i32, _> = "abc".parse::<i32>();
    println!("{:?}", bad);
}

// 🎯 MISSION 1
// Ferris receives orders as TEXT ("21") and must double them. 🥐 Sometimes
// customers type nonsense ("croissant"). Parse `s` into an i32 using `?`
// (so errors are returned automatically), then return `Ok(n * 2)`.
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    todo!() // <-- replace this with your code
}

// 🎯 MISSION 2
// Oh no, the cafe's calculator exploded when someone divided by zero! 💥
// Make `safe_div` return Err(String::from("division by zero")) when b == 0,
// and Ok(a / b) otherwise.
fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    assert_eq!(parse_and_double("21"), Ok(42));
    assert!(parse_and_double("nope").is_err());
    assert_eq!(safe_div(10, 2), Ok(5));
    assert_eq!(safe_div(1, 0), Err(String::from("division by zero")));
    println!("✅ Exercise 17 passed! No more exploding calculators. 🧮");
}
