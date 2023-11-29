mod create_stmt;
mod create_table_as_stmt;
mod define_stmt;
mod execute_stmt;
mod fmt;
mod into_clause;
mod node;
mod object_type;
mod parse_result;
mod raw_stmt;
mod select_stmt;
mod with_clause;

use fmt::Print;
use fmt::Printer;
use pg_query::protobuf;
use pg_query::Error;
use pg_query::Result;

const INDENT: isize = 4;

/// Converts a parsed tree back into a pretty-printed string.
pub fn unparse(protobuf: &protobuf::ParseResult) -> Result<String> {
    let mut p = Printer::new();

    protobuf.print(&mut p).ok_or(Error::Parse(String::new()))?;

    Ok(p.eof())
}
