// ╔══════════════════════════════════════════════════════════╗
// ║  05 - IF / ELSE (no parentheses, no truthiness)          ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python:  if x > 3:  ...  elif ...:  ...  else:  ...
// Rust:    if x > 3 { ... } else if ... { ... } else { ... }
//
// * No parentheses around the condition. The braces ARE required.
// * The condition must be a real `bool`. No "truthy" values!
//       if 1 { }        // ❌ nope
//       if "" { }       // ❌ nope
//       if x != 0 { }   // ✅ say what you mean
// * In Rust `if` is an EXPRESSION: it produces a value, like Python's ternary.
//       let label = if n < 0 { "negative" } else { "not negative" };
//   Both branches must produce the same type, or the compiler gets grumpy.

fn demo() {
    let n = 7;
    if n % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
    let label = if n < 0 { "negative" } else if n == 0 { "zero" } else { "positive" };
    println!("{n} is {label}");
}

// 🎯 MISSION
// The Crab Cafe's lucky-number game! Customer number `n` wins a prize:
//   divisible by 15  ->  "FizzBuzz"   (the grand prize!)
//   divisible by 3   ->  "Fizz"
//   divisible by 5   ->  "Buzz"
//   anything else    ->  the number itself, as text
//
// Oh no, the order matters: check 15 FIRST, or 15 will be called just "Fizz"!
// Hint: `n.to_string()` turns a number into a String.
// Since `if` is an expression, you can just make the whole chain the
// function's last line.
fn fizzbuzz(n: u32) -> String {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(9), "Fizz");
    assert_eq!(fizzbuzz(10), "Buzz");
    assert_eq!(fizzbuzz(7), "7");
    println!("✅ Exercise 05 passed! Prizes for everyone! 🎁");
}
