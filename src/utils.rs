use crate::fmt;
use crate::fmt::Print;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::Node;
use pg_query::NodeEnum;

const ESCAPE_STRING_SYNTAX: char = 'E';

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
pub fn deparse_string_literal(p: &mut fmt::Printer, val: &str) -> fmt::Result {
    if val.contains('\\') {
        p.word(ESCAPE_STRING_SYNTAX.to_string());
    }

    p.word('\''.to_string());

    for c in val.chars() {
        if c == '\'' || c == '\\' {
            p.word(c.to_string());
        }

        p.word(c.to_string());
    }

    p.word('\''.to_string());

    Ok(())
}

pub fn is_op(val: Option<String>) -> bool {
    val.unwrap().chars().all(|cp| {
        cp == '~'
            || cp == '!'
            || cp == '@'
            || cp == '#'
            || cp == '^'
            || cp == '&'
            || cp == '|'
            || cp == '`'
            || cp == '?'
            || cp == '+'
            || cp == '-'
            || cp == '*'
            || cp == '/'
            || cp == '%'
            || cp == '<'
            || cp == '>'
            || cp == '='
    })
}

pub fn str_val(node: &Node) -> Option<String> {
    match node.node.as_ref().unwrap() {
        NodeEnum::String(val) => Some(val.sval.clone()),
        _ => None,
    }
}

pub fn int_val(node: &Node) -> Option<i32> {
    match node.node.as_ref().unwrap() {
        NodeEnum::Integer(val) => Some(val.ival),
        _ => None,
    }
}

pub fn a_const_int_val(node: &Node) -> Option<i32> {
    match node.node.as_ref().unwrap() {
        NodeEnum::AConst(AConst {
            val: Some(Val::Ival(val)),
            ..
        }) => Some(val.ival),
        _ => None,
    }
}

pub fn print_expr_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, expr) in list.iter().enumerate() {
        expr.print(p)?;
        p.comma(i >= list.len() - 1);
    }

    Ok(())
}

pub fn print_column_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, col) in list.iter().enumerate() {
        p.ident(str_val(col).unwrap());

        if i < list.len() - 1 {
            p.word(", ");
        }
    }

    Ok(())
}

pub fn print_opt_with(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if !list.is_empty() {
        p.keyword(" with ");
        p.word("(");

        for (i, option) in list.iter().enumerate() {
            option.print(p)?;
            p.comma(i >= list.len() - 1);
        }

        p.word(")");
        p.nbsp();
    }

    Ok(())
}

pub fn print_any_name(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            p.word(".");
        }

        p.ident(str_val(part).unwrap());
    }

    Ok(())
}
