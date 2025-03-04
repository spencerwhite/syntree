//! Helper utilities for pretty-printing trees.

use core::fmt;
use std::io::{Error, Write};

use crate::span::{self, Span};
use crate::tree::{Kind, Tree};

/// Pretty-print a tree without a source.
///
/// This will replace all source references with `+`. If you have a source
/// available you can use [`print_with_source`] instead.
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
/// # Ok::<_, Box<dyn std::error::Error>>(())
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
pub fn print<O, T, S>(o: O, tree: &Tree<T, S>) -> Result<(), Error>
where
    O: Write,
    T: fmt::Debug,
    S: span::TreeSpan + fmt::Display,
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
/// # Ok::<_, Box<dyn std::error::Error>>(())
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
pub fn print_with_source<O, T>(o: O, tree: &Tree<T, Span>, source: &str) -> Result<(), Error>
where
    O: Write,
    T: fmt::Debug,
{
    print_with_lookup(o, tree, |span| source.get(span.range()))
}

fn print_with_lookup<'a, O, T, S>(
    mut o: O,
    tree: &Tree<T, S>,
    source: impl Fn(&S) -> Option<&'a str>,
) -> Result<(), Error>
where
    O: Write,
    T: fmt::Debug,
    S: span::TreeSpan + fmt::Display,
{
    for (depth, node) in tree.walk().with_depths() {
        let n = depth * 2;
        let data = node.value();
        let span = node.span();

        match node.kind() {
            Kind::Token => {
                if let Some(source) = source(span) {
                    writeln!(o, "{:n$}{:?}@{} {:?}", "", data, span, source, n = n)?;
                } else {
                    writeln!(o, "{:n$}{:?}@{} +", "", data, span, n = n)?;
                }
            }
            Kind::Node => {
                writeln!(o, "{:n$}{:?}@{}", "", data, span, n = n)?;
            }
        }
    }

    Ok(())
}
