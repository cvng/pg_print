mod create_stmt;
mod create_table_as_stmt;
mod define_stmt;
mod fmt;
mod node_enum;
mod object_type;
mod parse_result;
mod raw_stmt;

use crate::fmt::Printer;
use fmt::Print;
use pg_query::protobuf;
use pg_query::Result;

const INDENT: isize = 4;

/// Converts a parsed tree back into a pretty-printed string.
pub fn unparse(protobuf: &protobuf::ParseResult) -> Result<String> {
    let mut p = Printer::new();
    protobuf.print(&mut p);
    Ok(p.eof())
}
