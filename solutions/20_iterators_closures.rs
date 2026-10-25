// ╔══════════════════════════════════════════════════════════╗
// ║  20 - CLOSURES AND ITERATORS                             ║
// ╚══════════════════════════════════════════════════════════╝
//
// CLOSURES are Rust's lambdas, and they can span many lines:
//     let add = |a, b| a + b;
//     let add_typed = |a: i32, b: i32| -> i32 { a + b };
// They can also grab variables from their surroundings.
//
// ITERATORS replace list comprehensions. They are LAZY (nothing happens until
// you ask for a result) and compile down to loops as fast as handwritten ones.
//
//   Python:  [x*x for x in v if x % 2 == 0]
//   Rust:    v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).collect()
//
// A pipeline has 3 parts 🚰:
//   source:     .iter() (borrow items)   .into_iter() (take them)   0..10
//   adaptors:   .map(f)  .filter(f)  .enumerate()  .zip(o)  .rev()  .take(n)
//   consumer:   .collect()  .sum()  .count()  .max()  .min()  .any(f)  .all(f)
//
// About `|&&x|`: iter() produces `&i32`, and filter hands you a reference to
// that, so `&&x` peels both references off. Don't panic: when you get it wrong,
// the compiler tells you exactly what it expected. 🙏

fn demo() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    let total: i32 = v.iter().sum();
    let any_big = v.iter().any(|&x| x > 5);
    let max = v.iter().max();
    println!("{:?} {} {} {:?}", doubled, total, any_big, max);

    let offset = 10;
    let add_offset = |x| x + offset; // grabs `offset` from outside
    println!("{}", add_offset(5));
}

// 🎯 MISSION 1
// Ferris's factory stamps a number on every cookie tin. 🍪 He wants the
// SQUARES of only the EVEN numbers. Chain it in one line:
//   v.iter().filter(...).map(...).collect()
fn squares_of_evens(v: &[i32]) -> Vec<i32> {
    // [[
    v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).collect()
    // ]]
}

// 🎯 MISSION 2
// How many letters are there in ALL the words together?
// Chain: `.iter()`, `.map(|w| w.len())`, `.sum()`.
fn total_len(words: &[&str]) -> usize {
    // [[
    words.iter().map(|w| w.len()).sum()
    // ]]
}

fn main() {
    demo();
    assert_eq!(squares_of_evens(&[1, 2, 3, 4, 5, 6]), vec![4, 16, 36]);
    assert_eq!(total_len(&["ab", "cde", "f"]), 6);

    // 🎯 MISSION 3
    // Create a closure called `triple` that takes an i32 and multiplies it by 3.
    // Syntax: `let name = |param: type| expression;`
    // [[
    let triple = |x: i32| x * 3;
    // ]]
    assert_eq!(triple(4), 12);
    println!("✅ Exercise 20 passed! Iterators go brrr. 🚀");
}
