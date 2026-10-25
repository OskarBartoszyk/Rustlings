// ╔══════════════════════════════════════════════════════════╗
// ║  03 - TYPES: Rust likes to know exactly what you mean    ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python: "int" is one type, and it grows forever.
// Rust:   there are MANY integer types, sized in bits:
//
//   signed (can be negative):  i8  i16  i32  i64  i128  isize
//   unsigned (0 and up):       u8  u16  u32  u64  u128  usize
//   floats:                    f32  f64
//   plus:                      bool,  char  ('a' - SINGLE quotes!)
//
// * The default integer is `i32`, the default float is `f64`.
// * `usize` is the type for sizes and indexes.
// * NO implicit conversions! `1 + 2.5` is an error. Convert on purpose with `as`:
//        let a: i32 = 7;
//        let b = a as f64;      // 7.0
// * A u8 holds 0..=255. Try to store 256 and Rust panics (in debug builds).
//   No silent wrap-around surprises.
//
// Containers of a fixed size:
//  * TUPLE  (1, "two", 3.0)   mixed types.  Access with t.0, or destructure:
//                             let (a, b, c) = t;
//  * ARRAY  [10, 20, 30]      same type, fixed length (Python's list is closer to
//                             `Vec`, which we meet later).

fn demo() {
    let a: i32 = 7;
    let b: u8 = 255;
    let c = 2.5; // f64
    let d = 'z';
    let e = true;
    println!("{a} {b} {c} {d} {e}");

    let t = (1, "two", 3.0);
    let (x, y, z) = t; // unpack, just like Python!
    println!("{x} {y} {z} / first = {}", t.0);

    let arr = [10, 20, 30, 40, 50];
    println!("len = {}, arr[2] = {}", arr.len(), arr[2]);

    let big = a as f64 * c; // convert first, THEN mix
    println!("{big}");
}

// 🎯 MISSION 1
// Ferris's cafe sells two kinds of seaweed tea, priced as whole numbers (a, b).
// The owner wants the AVERAGE price, with decimals (3 and 4 -> 3.5).
// Oh no! `a / b` on integers would chop off the decimals!
// Convert both to f64 with `as f64` BEFORE dividing.
// 📤 EXPECTED RESULT: Create a new variable `result` of type f64 holding the average of a and b, and RETURN it (no println!). average(3, 4) must return 3.5.
fn average(a: i32, b: i32) -> f64 {
    todo!() // <-- replace this with your code
}

// 🎯 MISSION 2
// A pair of crabs (a, b) stands in the wrong order. Return a tuple with them swapped.
// Hint: destructure it first with `let (a, b) = pair;`
// 📤 EXPECTED RESULT: Return a tuple (i32, i32) with the two values in reverse order. swap((1, 2)) must return (2, 1).
fn swap(pair: (i32, i32)) -> (i32, i32) {
    todo!() // <-- replace this with your code
}

// 🎯 MISSION 3
// Five buckets of fish in a row. Ferris only wants the FIRST bucket plus the LAST one.
// Arrays are indexed from 0 (just like Python). The last index of 5 items is 4.
// 📤 EXPECTED RESULT: Return ONE i32: the first element plus the last element. first_plus_last([1, 2, 3, 4, 10]) must return 11.
fn first_plus_last(arr: [i32; 5]) -> i32 {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    assert_eq!(average(3, 4), 3.5);
    assert_eq!(swap((1, 2)), (2, 1));
    assert_eq!(first_plus_last([1, 2, 3, 4, 10]), 11);
    println!("✅ Exercise 03 passed! The cafe's books are in order. 📒");
}
