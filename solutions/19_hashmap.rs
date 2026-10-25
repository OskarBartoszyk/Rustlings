// ╔══════════════════════════════════════════════════════════╗
// ║  19 - HASHMAP: Rust's dict                               ║
// ╚══════════════════════════════════════════════════════════╝
//
//     use std::collections::HashMap;
//     let mut m = HashMap::new();
//     m.insert("apples".to_string(), 3);   // one key type, one value type
//     m.get("apples")                      // -> Option<&V>  (no KeyError, hooray!)
//     m.contains_key("apples");  m.remove("apples");
//
// THE ENTRY TRICK 🪄 (this is Python's defaultdict / setdefault):
//
//     *m.entry(key).or_insert(0) += 1;
//
//   Read it from the inside out:
//     m.entry(key)       "find the slot for this key"
//     .or_insert(0)      "if it's empty, put 0 there" -> gives back a `&mut` to the value
//     *... += 1          "follow the reference and add one"
//
// Looping: `for (k, v) in &m { }`. The ORDER IS RANDOM, don't rely on it.

use std::collections::HashMap;

fn demo() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("apples".to_string(), 3);
    *m.entry("apples".to_string()).or_insert(0) += 2;
    *m.entry("pears".to_string()).or_insert(0) += 1;
    println!("{:?}", m.get("apples"));
    println!("{:?}", m.get("nope"));
    for (k, v) in &m {
        println!("{k} -> {v}");
    }
}

// 🎯 MISSION 1
// The cafe's gossip column! 🗞️ Count how many times each word appears in `text`.
// `counts` is ready. Loop over `text.split_whitespace()` and for every word
// do the entry trick: `*counts.entry(w.to_string()).or_insert(0) += 1;`
fn count_words(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    // [[
    for w in text.split_whitespace() {
        *counts.entry(w.to_string()).or_insert(0) += 1;
    }
    // ]]
    counts
}

// 🎯 MISSION 2
// How popular is `key`? Return its count, or 0 if the word never showed up.
// `m.get(key)` gives an Option<&u32>. Use `.unwrap_or(&0)` for the fallback
// and a `*` in front to turn the reference into a plain number.
fn lookup_or_zero(m: &HashMap<String, u32>, key: &str) -> u32 {
    // [[
    *m.get(key).unwrap_or(&0)
    // ]]
}

fn main() {
    demo();
    let counts = count_words("the cat saw the other cat and the dog");
    assert_eq!(lookup_or_zero(&counts, "the"), 3);
    assert_eq!(lookup_or_zero(&counts, "cat"), 2);
    assert_eq!(lookup_or_zero(&counts, "dog"), 1);
    assert_eq!(lookup_or_zero(&counts, "bird"), 0);
    println!("✅ Exercise 19 passed! The gossip is fully indexed. 🤫");
}
