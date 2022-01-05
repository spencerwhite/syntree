//! Helper utilities for pretty-printing trees.

use std::fmt::Debug;
use std::io::{Error, Write};

use crate::{Event, Kind, Span, Tree};

/// Pretty-print a tree without a source.
///
/// This will replace all source references with `+`. If you have a source
/// available you can use [print_with_source] instead.
///
/// # Examples
///
/// ```
/// #[derive(Debug)]
/// enum Syntax {
///     NUMBER,
///     WHITESPACE,
///     OPERATOR,
///     PLUS,
/// }
///
/// use Syntax::*;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tree = syntree::tree! {
///     NUMBER => {
///         (NUMBER, 3),
///     },
///     (WHITESPACE, 1),
///     OPERATOR => {
///         (PLUS, 1)
///     },
///     (WHITESPACE, 1),
///     NUMBER => {
///         (NUMBER, 2),
///     },
/// };
///
/// let mut s = Vec::new();
/// syntree::print::print(&mut s, &tree)?;
/// # let s = String::from_utf8(s)?;
/// # assert_eq!(s, "NUMBER@0..3\n  NUMBER@0..3 +\nWHITESPACE@3..4 +\nOPERATOR@4..5\n  PLUS@4..5 +\nWHITESPACE@5..6 +\nNUMBER@6..8\n  NUMBER@6..8 +\n");
/// # Ok(()) }
/// ```
///
/// This would write:
///
/// ```text
/// NUMBER@0..3
///   NUMBER@0..3 +
/// WHITESPACE@3..4 +
/// OPERATOR@4..5
///   PLUS@4..5 +
/// WHITESPACE@5..6 +
/// NUMBER@6..8
///   NUMBER@6..8 +
/// ```
pub fn print<O, T>(o: O, tree: &Tree<T>) -> Result<(), Error>
where
    O: Write,
    T: Debug,
{
    print_with_lookup(o, tree, |_| None)
}

/// Pretty-print a tree with the source spans printed.
///
/// # Examples
///
/// ```
/// #[derive(Debug)]
/// enum Syntax {
///     NUMBER,
///     WHITESPACE,
///     OPERATOR,
///     PLUS,
/// }
///
/// use Syntax::*;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let source = "128 + 64";
///
/// let tree = syntree::tree! {
///     NUMBER => {
///         (NUMBER, 3),
///     },
///     (WHITESPACE, 1),
///     OPERATOR => {
///         (PLUS, 1)
///     },
///     (WHITESPACE, 1),
///     NUMBER => {
///         (NUMBER, 2),
///     },
/// };
///
/// let mut s = Vec::new();
/// syntree::print::print_with_source(&mut s, &tree, source)?;
/// # let s = String::from_utf8(s)?;
/// # assert_eq!(s, "NUMBER@0..3\n  NUMBER@0..3 \"128\"\nWHITESPACE@3..4 \" \"\nOPERATOR@4..5\n  PLUS@4..5 \"+\"\nWHITESPACE@5..6 \" \"\nNUMBER@6..8\n  NUMBER@6..8 \"64\"\n");
/// # Ok(()) }
/// ```
///
/// This would write:
///
/// ```text
/// NUMBER@0..3
///   NUMBER@0..3 "128"
/// WHITESPACE@3..4 " "
/// OPERATOR@4..5
///   PLUS@4..5 "+"
/// WHITESPACE@5..6 " "
/// NUMBER@6..8
///   NUMBER@6..8 "64"
/// ```
pub fn print_with_source<O, T>(o: O, tree: &Tree<T>, source: &str) -> Result<(), Error>
where
    O: Write,
    T: Debug,
{
    print_with_lookup(o, tree, |span| source.get(span.range()))
}

fn print_with_lookup<'a, O, T>(
    mut o: O,
    tree: &Tree<T>,
    source: impl Fn(Span) -> Option<&'a str>,
) -> Result<(), Error>
where
    O: Write,
    T: Debug,
{
    let mut depth = 0usize;

    for (event, node) in tree.walk_events() {
        // We ignore up events because we're not interested in rendering closing
        // elements.
        if matches!(event, Event::Up) {
            depth = depth.checked_sub(1).expect("depth underflow");
            continue;
        }

        if matches!(event, Event::First) {
            depth = depth.checked_add(1).expect("depth overflow");
        }

        // Indentation is one level down.
        let n = depth * 2;

        let data = node.data();

        if let Kind::Token = node.kind() {
            if let Some(source) = source(node.span()) {
                writeln!(o, "{:n$}{:?}@{} {:?}", "", data, node.span(), source, n = n)?;
            } else {
                writeln!(o, "{:n$}{:?}@{} +", "", data, node.span(), n = n)?;
            }

            continue;
        }

        if node.is_empty() {
            writeln!(o, "{:n$}{:?}@{}", "", data, node.span(), n = n)?;
            continue;
        }

        writeln!(o, "{:n$}{:?}@{}", "", data, node.span(), n = n)?;
    }

    Ok(())
}
