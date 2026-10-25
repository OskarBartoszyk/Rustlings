// ╔══════════════════════════════════════════════════════════╗
// ║  11 - STRINGS AND SLICES                                 ║
// ╚══════════════════════════════════════════════════════════╝
//
// Python has one `str`. Rust has two main flavours:
//
//   String   an OWNED, growable piece of text (lives on the heap)
//   &str     a BORROWED VIEW into some text, called a "string slice".
//            String literals like "hello" are &str.
//
//     let lit: &str = "hello";
//     let owned: String = String::from(lit);   // also: lit.to_string()
//     let view: &str = &owned[0..2];           // "he" - no copying, just a window 🪟
//
// * In function parameters PREFER `&str`: it accepts literals AND &String.
// * Strings are UTF-8, so `s[0]` is NOT allowed (one char may be several bytes,
//   think "ł" or "🦀"). Use .chars(), or slice by byte ranges like &s[0..5].
// * Slices exist for arrays/Vecs too: `&[i32]` is a window into a sequence.
// * Useful methods: len, is_empty, trim, split, split_whitespace, contains,
//   starts_with, replace, to_uppercase, chars, parse.

fn demo() {
    let owned = String::from("hello world");
    let hello: &str = &owned[0..5];
    let world = &owned[6..];
    println!("{hello} | {world}");

    for w in "a b  c".split_whitespace() {
        println!("word: {w}");
    }
    println!("{}", "  padded ".trim());
    println!("{}", "abc".to_uppercase());

    let nums = [1, 2, 3, 4, 5];
    let middle: &[i32] = &nums[1..4];
    println!("{:?}", middle); // {:?} = "debug print", shows lists and structs
}

// 🎯 MISSION 1
// Ferris reads customer notes and only cares about the first word
// ("hello world" -> "hello"). For an empty note ("   ") return "".
// Hint: `s.split_whitespace().next()` gives an Option (the word, or nothing).
// Add `.unwrap_or("")` to say "or just use an empty string".
fn first_word(s: &str) -> &str {
    // [[
    s.split_whitespace().next().unwrap_or("")
    // ]]
}

// 🎯 MISSION 2
// Ferris loves palindromes: words that read the same backwards ("racecar"). 🏎️
// Return true if `s` is one.
// Hint: `s.chars()` is an iterator over letters, `.rev()` flips it, and
// iterators have an `.eq(other)` method to compare element by element.
fn is_palindrome(s: &str) -> bool {
    // [[
    s.chars().eq(s.chars().rev())
    // ]]
}

fn main() {
    demo();
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("   "), "");
    assert!(is_palindrome("racecar"));
    assert!(!is_palindrome("rust"));
    println!("✅ Exercise 11 passed! Ferris reads backwards now. 🪞");
}
