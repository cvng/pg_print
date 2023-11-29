#![allow(unused)]

mod algorithm;
mod convenience;
mod create_stmt;
mod create_table_as_stmt;
mod define_stmt;
mod node;
mod ring;
mod tree;

use crate::algorithm::Printer;
use pg_query::protobuf;
use pg_query::Result;

const MARGIN: isize = 89;
const INDENT: isize = 4;
const MIN_SPACE: isize = 60;

/// Converts a parsed tree back into a pretty-printed string.
pub fn unparse(protobuf: &protobuf::ParseResult) -> Result<String> {
    let mut p = Printer::new();
    p.tree(protobuf);
    Ok(p.eof())
}
