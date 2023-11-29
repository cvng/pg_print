use crate::fmt;
use crate::fmt::Print;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::Node;
use pg_query::NodeEnum;

const ESCAPE_STRING_SYNTAX: char = 'E';

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
pub fn deparse_string_literal(p: &mut fmt::Printer, val: &str) {
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

pub fn print_expr_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Option {
    for (i, node) in list.iter().enumerate() {
        node.print(p);
        p.comma(i >= list.len() - 1);
    }

    Some(())
}

pub fn print_column_list(p: &mut fmt::Printer, list: &[Node]) {
    for (i, column) in list.iter().enumerate() {
        p.ident(str_val(column).unwrap());
        if i < list.len() - 1 {
            p.word(", ");
        }
    }
}

pub fn print_opt_with(str: &mut fmt::Printer, list: &[Node]) {
    if !list.is_empty() {
        str.keyword(" with ");
        node_rel_options(str, list);
        str.nbsp();
    }
}

fn node_rel_options(str: &mut fmt::Printer, list: &[Node]) {
    str.word("(");

    for (i, option) in list.iter().enumerate() {
        match option.node.as_ref().unwrap() {
            NodeEnum::DefElem(node) => {
                if !node.defnamespace.is_empty() {
                    str.ident(node.defnamespace.clone());
                    str.word(".");
                }
                str.ident(node.defname.clone());
                if let Some(arg) = &node.arg {
                    str.word(" = ");
                    print_def_arg(str, arg, false);
                }
            }
            _ => unreachable!(),
        }

        str.comma(i >= list.len() - 1);
    }

    str.word(")");
}

fn print_def_arg(str: &mut fmt::Printer, node: &Node, _is_operator_def_arg: bool) {
    match node.node.as_ref().unwrap() {
        NodeEnum::Integer(ref val) => Option::<Val>::print(&Some(Val::Ival(val.clone())), str),
        _ => todo!(),
    };
}
