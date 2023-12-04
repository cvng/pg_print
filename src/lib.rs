mod collate_clause;
mod column_def;
mod column_ref;
mod constraint;
mod create_extension_stmt;
mod create_foreign_table_stmt;
mod create_function_stmt;
mod create_schema_stmt;
mod create_stmt;
mod create_table_as_stmt;
mod create_trig_stmt;
mod def_elem;
mod define_stmt;
mod domain;
mod execute_stmt;
mod expr;
mod fmt;
mod function_parameter;
mod gram;
mod index_elem;
mod index_stmt;
mod integer;
mod interval_fields;
mod into_clause;
mod list;
mod name;
mod stmt;
mod object_type;
mod on_commit_action;
mod param_ref;
mod partition;
mod privilege;
mod range_var;
mod rel_persistence;
mod res_target;
mod role_spec;
mod select_stmt;
mod string;
mod tree;
mod type_name;
mod val;
mod view_stmt;
mod with_clause;
mod node;

use crate::fmt::Printer;
use pg_query::protobuf::ParseResult;

const INDENT: isize = 4;

/// Converts a parsed tree back into a pretty-printed string.
pub fn unparse(protobuf: &ParseResult) -> pg_query::Result<String> {
    let mut p = Printer::new();
    p.tree(protobuf);
    Ok(p.eof())
}
