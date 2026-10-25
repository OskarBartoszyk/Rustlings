// ╔══════════════════════════════════════════════════════════╗
// ║  18 - VEC: Rust's list                                   ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python list  ->  Rust `Vec<T>`. It grows and shrinks, but EVERY element has
// the same type (no mixing ints and strings in one Vec).
//
//     let mut v = vec![1, 2, 3];     // or Vec::new() for an empty one
//     v.push(4);                     // append
//     v.pop();                       // removes the last -> returns an Option!
//     v.len();
//     v[0]                           // index. PANICS if out of range 💥
//     v.get(10)                      // safe version -> Option<&T>
//
// Looping:
//     for x in &v { }          // borrow each item, v stays usable
//     for x in &mut v { }      // borrow each item mutably
//     for x in v { }           // consume v (it's moved into the loop!)
//
// In function parameters prefer `&[T]` (a slice) over `&Vec<T>`: it accepts
// Vecs, arrays and sub-slices.

fn demo() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?} len={}", v, v.len());
    println!("{:?} {:?}", v.get(1), v.get(99));
    let last = v.pop();
    println!("{:?} {:?}", last, v);
    for x in &v {
        print!("{x} ");
    }
    println!();
}

// 🎯 MISSION 1
// Ferris sorts the lottery tickets and keeps only the EVEN ones. 🎟️
// `out` is an empty Vec waiting for you. Loop over `v`, and `push` every
// even number into `out`.
// 📤 EXPECTED RESULT: Return the existing Vec `out` filled with the even numbers of `v`, in order. [1, 2, 3, 4] -> [2, 4].
fn evens(v: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    todo!() // <-- replace this with your code
    out
}

// 🎯 MISSION 2
// What was the LAST ticket in the pile? If the pile is empty, answer 0.
// `v.last()` returns an Option<&i32>. Handle both cases with `match`.
// (Hint: the pattern `Some(&x)` unpacks the reference for you.)
// 📤 EXPECTED RESULT: Return an i32: the last element of `v`, or 0 if `v` is empty. [1, 2, 3] -> 3, [] -> 0.
fn last_or_zero(v: &[i32]) -> i32 {
    todo!() // <-- replace this with your code
}

fn main() {
    demo();
    assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    assert_eq!(last_or_zero(&[1, 2, 9]), 9);
    assert_eq!(last_or_zero(&[]), 0);
    println!("✅ Exercise 18 passed! Winning tickets sorted. 🍀");
}
