// ╔══════════════════════════════════════════════════════════╗
// ║  06 - LOOPS                                              ║
// ╚══════════════════════════════════════════════════════════╝
//
// Rust has three loops:
//
//   loop { ... }            runs forever until you `break`  (Python: while True)
//                           ...and `break value;` makes the loop RETURN a value!
//   while cond { ... }      like Python
//   for x in something { }  like Python
//
// RANGES (Python's range):
//    0..5    ->  0 1 2 3 4        (end NOT included)
//    0..=5   ->  0 1 2 3 4 5      (the `=` includes the end)
// Handy helpers:  .rev() reverses,  .enumerate() gives (index, item).
// `continue` and `break` work just like in Python.

fn demo() {
    for i in 0..3 {
        println!("i = {i}");
    }
    for i in (0..3).rev() {
        println!("countdown {i}");
    }
    for (idx, c) in "abc".chars().enumerate() {
        println!("{idx}: {c}");
    }

    let mut n = 0;
    while n < 3 {
        n += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10; // the loop is now worth 50
        }
    };
    println!("result = {result}");
}

// 🎯 MISSION 1
// Ferris is stacking fish crates: the 1st row has 1 crate, the 2nd has 2,
// ... the n-th row has n. How many crates in total?
// Oh no, the `for` loop is missing! Write one that walks over 1..=n
// (inclusive!) and adds each number to `total`.
// 📤 EXPECTED RESULT: Return a u32: the sum 1 + 2 + ... + n (use the existing `total`). sum_to(4) must return 10.
fn sum_to(n: u32) -> u32 {
    let mut total = 0;
    todo!() // <-- replace this with your code
    total
}

// 🎯 MISSION 2
// Ferris's magic bubble DOUBLES in size every second: 1, 2, 4, 8, 16...
// How big is the first bubble that is STRICTLY bigger than `n`?
// The `loop` below never ends on its own. Inside it:
//   - if `p` is bigger than `n`, leave with `break p;` (the loop returns p)
//   - otherwise double `p`.
// 📤 EXPECTED RESULT: Return a u32: the first power of two (1, 2, 4, 8...) strictly greater than n. first_power_of_two_above(5) -> 8, (8) -> 16.
fn first_power_of_two_above(n: u32) -> u32 {
    let mut p = 1;
    loop {
        todo!() // <-- replace this with your code
    }
}

fn main() {
    demo();
    assert_eq!(sum_to(10), 55);
    assert_eq!(first_power_of_two_above(10), 16);
    assert_eq!(first_power_of_two_above(0), 1);
    println!("✅ Exercise 06 passed! Crates stacked, bubbles blown. 🫧");
}
