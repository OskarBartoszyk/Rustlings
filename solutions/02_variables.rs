// ╔══════════════════════════════════════════════════════════╗
// ║  02 - VARIABLES (that refuse to change)                  ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python:  x = 5;  x = 6      # whatever, change it, nobody cares
// Rust:    variables are IMMUTABLE by default. They are like carved stone. 🪨
//
//     let x = 5;
//     x = 6;          // ❌ "cannot assign twice to immutable variable"
//
// Want a variable you can change? Say it out loud: `mut`.
//
//     let mut y = 5;
//     y = 6;          // ✅
//
// Why? Because when you read code and see a plain `let`, you KNOW it never
// changes. Less guessing, fewer bugs.
//
// More tricks:
//  * Types are inferred, but fixed. A variable can't change type later.
//  * SHADOWING: declare the same name again with `let`. It's a brand-new
//    variable that hides the old one (and may even have a different type!).
//  * `const` = a true constant. Needs a type and UPPER_SNAKE_CASE name.

const MAX_USERS: u32 = 100;

fn demo() {
    let x = 5;
    println!("x = {x}");

    let mut y = 5;
    y += 1; // fine, we said `mut`
    println!("y = {y}");

    let x = x * 2; // shadowing: a NEW x, the old one is hidden
    let x = format!("now I'm a string: {x}"); // new type? No problem!
    println!("{x}");

    println!("max users: {MAX_USERS}");
}

fn main() {
    demo();

    // 🎯 MISSION 1
    // Ferris is keeping score in a card game. Oh no, someone declared the
    // score as a plain stone-carved `let`, so it can't go up!
    // Declare `score` so that it STARTS at 10 and then goes up by 5.
    // (Your code goes between the two comment lines. `+=` adds in place.)
    // [[
    let mut score = 10;
    score += 5;
    // ]]
    assert_eq!(score, 15);

    // 🎯 MISSION 2
    // `spaces` is a bunch of spaces as text ("   "). But Ferris wants to know
    // HOW MANY there are, as a number. Don't invent a new name: SHADOW `spaces`
    // so that it becomes the number of characters.
    // Hint: "abc".len() gives 3.
    let spaces = "   ";
    // [[
    let spaces = spaces.len();
    // ]]
    assert_eq!(spaces, 3);

    println!("✅ Exercise 02 passed! Ferris is up 15 points. 🃏");
}
