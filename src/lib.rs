mod algo;
mod conv;
mod expr;
mod gram;
mod stmt;
mod tree;

use crate::algo::Printer;
#[cfg(feature = "unstable")]
use parser::Parse as ParseResult;
#[cfg(not(feature = "unstable"))]
use pg_query::protobuf::ParseResult;

const INDENT: isize = 4;

/// Converts a parsed tree back into a pretty-printed string.
pub fn unparse(tree: &ParseResult) -> String {
    let mut p = Printer::new();
    p.tree(tree);
    p.eof()
}

/// Parses the given SQL statement into the given abstract syntax tree.
pub fn parse(statement: &str) -> Option<ParseResult> {
    #[cfg(feature = "unstable")]
    return parser::parse_source(statement);
    #[cfg(not(feature = "unstable"))]
    return pg_query::parse(statement)
        .map(|result| result.protobuf)
        .ok();
}
