use crate::fmt::Printer;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::Node;
use pg_query::NodeEnum;

const ESCAPE_STRING_SYNTAX: char = 'E';

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
pub fn deparse_string_literal(p: &mut Printer, val: &str) {
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
