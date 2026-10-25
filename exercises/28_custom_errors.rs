// ╔══════════════════════════════════════════════════════════╗
// ║  28 - CUSTOM ERRORS AND THE `From` TRAIT                 ║
// ╚══════════════════════════════════════════════════════════╝
//
// Real programs fail in different ways: bad input, missing file, your own
// business rules... The Rust way is ONE enum with a variant per failure:
//
//     enum AppError { Parse(ParseIntError), Negative(i32) }
//
// And the `?` operator has a hidden superpower: when it returns an error, it
// calls `From::from` to convert it into YOUR error type. So once you write
//
//     impl From<ParseIntError> for AppError { ... }
//
// you can use `s.parse::<i32>()?` inside a function returning
// `Result<_, AppError>` and the conversion happens automatically. ✨
//
// (Shortcuts you'll meet later: `Box<dyn std::error::Error>` swallows any error;
// the crates `thiserror` and `anyhow` write this boilerplate for you.)

use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Negative(i32),
}

// 🎯 MISSION 1
// The `?` operator knows how to turn a ParseIntError into an AppError... only
// if we TELL it how. Oh no, nobody did! Wrap `e` into the right variant.
impl From<ParseIntError> for AppError {
    // 📤 EXPECTED RESULT: Return an AppError (Self) built from `e`, using the matching variant.
    fn from(e: ParseIntError) -> Self {
        todo!() // <-- replace this with your code
    }
}

// 🎯 MISSION 2
// The cafe only accepts POSITIVE order sizes. 🥧
//   1. Parse `s` into an i32 using `?` (the From impl above does the conversion).
//   2. If the number is negative: `return Err(AppError::Negative(n));`
//   3. Otherwise return `Ok(n)`.
// 📤 EXPECTED RESULT: Return Ok(n) for a positive number; Err(AppError::Negative(n)) for a negative one; a parse error becomes an AppError via `?`.
fn parse_positive(s: &str) -> Result<i32, AppError> {
    todo!() // <-- replace this with your code
}

fn demo() {
    println!("{:?}", parse_positive("5"));
    println!("{:?}", parse_positive("x"));
}

fn main() {
    demo();
    assert!(matches!(parse_positive("12"), Ok(12)));
    assert!(matches!(parse_positive("abc"), Err(AppError::Parse(_))));
    assert!(matches!(parse_positive("-3"), Err(AppError::Negative(-3))));
    println!("✅ Exercise 28 passed! Errors are classified and under control. 🗂️");
}
