//! # Doc Comment
//!
//! `doc_comment` demonstrates amazing documentation
//! capabilities of the `Rust` language.
//! For more details, see
//! [Chapter 14](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
//! of the `Rust Book`.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = doc_comment::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
