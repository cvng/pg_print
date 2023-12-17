mod expr;
mod fmt;
mod gram;
mod interval_fields;
mod name;
mod node;
mod parse;
mod partition;
mod rel_persistence;
mod stmt;

use crate::fmt::Printer;
#[cfg(feature = "unstable")]
use parser::Parse;
#[cfg(not(feature = "unstable"))]
use pg_query::protobuf::ParseResult as Parse;

const INDENT: isize = 4;

/// Converts a parsed tree back into a pretty-printed string.
pub fn unparse(parse: &Parse) -> String {
    let mut p = Printer::new();
    p.parse(parse);
    p.eof()
}
