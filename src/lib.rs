mod a_const;
mod a_expr;
mod a_star;
mod collate_clause;
mod column_def;
mod column_ref;
mod constraint;
mod create_domain_stmt;
mod create_schema_stmt;
mod create_stmt;
mod create_table_as_stmt;
mod def_elem;
mod define_stmt;
mod execute_stmt;
mod fmt;
mod index_elem;
mod index_stmt;
mod integer;
mod interval_fields;
mod into_clause;
mod name;
mod node;
mod object_type;
mod on_commit_action;
mod param_ref;
mod parse_result;
mod partition_bound_spec;
mod partition_strategy;
mod range_var;
mod raw_stmt;
mod rel_persistence;
mod res_target;
mod role_spec;
mod select_stmt;
mod type_name;
mod utils;
mod val;
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

    protobuf
        .print(&mut p)
        .map_err(|_| Error::Parse(String::new()))?;

    Ok(p.eof())
}
