mod col;
mod collate_clause;
mod column_def;
mod column_ref;
mod constraint;
mod def_elem;
mod expr;
mod fmt;
mod function_parameter;
mod gram;
mod index_elem;
mod integer;
mod interval_fields;
mod into_clause;
mod list;
mod name;
mod node;
mod object_type;
mod on_commit_action;
mod param_ref;
mod parse;
mod partition;
mod privilege;
mod range_var;
mod rel_persistence;
mod res_target;
mod role_spec;
mod stmt;
mod string;
mod type_name;
mod val;
mod with_clause;

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
