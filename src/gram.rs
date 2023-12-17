// Adapted from https://github.com/postgres/postgres/blob/REL_15_STABLE/src/backend/parser/gram.y.

use crate::fmt::Context;
use crate::fmt::Printer;
use crate::rel_persistence::RelPersistence;
use crate::INDENT;
use pg_query::protobuf;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::protobuf::AStar;
use pg_query::protobuf::ColumnDef;
use pg_query::protobuf::DropBehavior;
use pg_query::protobuf::FunctionParameterMode;
use pg_query::protobuf::List;
use pg_query::protobuf::RangeVar;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

pub const TRIGGER_TYPE_BEFORE: usize = 1 << 1;
pub const TRIGGER_TYPE_INSERT: usize = 1 << 2;
pub const TRIGGER_TYPE_DELETE: usize = 1 << 3;
pub const TRIGGER_TYPE_UPDATE: usize = 1 << 4;
pub const TRIGGER_TYPE_TRUNCATE: usize = 1 << 5;
pub const TRIGGER_TYPE_INSTEAD: usize = 1 << 6;
pub const TRIGGER_TYPE_AFTER: usize = 0;

const NAMEDATALEN: usize = 64;
const ESCAPE_STRING_SYNTAX: char = 'E';

/// Returns the given expression if it matches any of the given patterns.
#[macro_export]
macro_rules! cast {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => Some($expression),
            _ => None
        }
    };
}

#[macro_export]
macro_rules! cast_node {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            pg_query::Node { node: Some($pattern) } $(if $guard)? => Some($expression),
            _ => None
        }
    };
}

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
pub fn string_literal(p: &mut Printer, val: &str) {
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

pub fn make_range_var_into_any_name(n: &RangeVar) -> [Node; 1] {
    [Node {
        node: Some(NodeEnum::String(protobuf::String {
            sval: n.relname.clone(),
        })),
    }]
}

impl Printer {
    pub fn a_const(&mut self, n: &AConst) {
        self.opt_val(n.val.as_ref(), &Context::Constant);
    }

    pub fn a_star(&mut self, _n: &AStar) {
        self.word("*");
    }

    pub fn any_name(&mut self, list: &[Node]) {
        for (i, part) in list.iter().enumerate() {
            if i > 0 {
                self.word(".");
            }

            self.ident(str_val(part).unwrap());
        }
    }

    pub fn expr_list(&mut self, list: &[Node]) {
        for (i, expr) in list.iter().enumerate() {
            self.node(expr);
            self.trailing_comma(i >= list.len() - 1);
        }
    }

    pub fn column_list(&mut self, list: &[Node]) {
        for (i, col) in list.iter().enumerate() {
            self.ident(str_val(col).unwrap());

            if i < list.len() - 1 {
                self.word(", ");
            }
        }
    }

    pub fn print_list(&mut self, list: &[Node]) {
        self.expr_list(list);
    }

    pub fn opt_with(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word(" with ");
            self.word("(");

            for (i, option) in list.iter().enumerate() {
                self.node(option);
                self.trailing_comma(i >= list.len() - 1);
            }

            self.word(")");
            self.nbsp();
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_clause(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("from ");

            for (i, item) in list.iter().enumerate() {
                self.node(item);
                self.trailing_comma(i >= list.len() - 1);
            }
            self.word(" ");
        }
    }

    pub fn where_clause(&mut self, n: Option<&Node>) {
        if let Some(node) = n {
            self.word("where ");
            self.node(node);
            self.word(" ");
        }
    }

    pub fn opt_collate(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("collate ");
            self.any_name(list);
            self.nbsp();
        }
    }

    pub fn reloptions(&mut self, _list: &[Node]) {
        todo!();
    }

    pub fn func_name(&mut self, list: &[Node]) {
        for (i, part) in list.iter().enumerate() {
            if i > 0 {
                self.word(".");
            }

            self.ident(str_val(part).unwrap());
        }
    }

    pub fn func_args_with_defaults(&mut self, list: &[Node]) {
        self.word("(");
        if !list.is_empty() {
            self.cbox(INDENT);
            self.hardbreak_if_nonempty();
            for (i, arg) in list.iter().enumerate() {
                self.node(arg);
                self.trailing_comma(i >= list.len() - 1);
            }
            self.space();
            self.offset(-INDENT);
            self.end();
        }
        self.word(")");
        self.space();
    }

    pub fn func_return(&mut self, n: &TypeName) {
        self.type_name(n);
        self.nbsp();
    }

    pub fn opt_createfunc_opt_list(&mut self, list: &[Node]) {
        if !list.is_empty() {
            for option in list.iter().skip(1) {
                if let NodeEnum::DefElem(node) = option.node.as_ref().unwrap() {
                    if let Some(arg) = &node.arg {
                        if node.defname == "volatility" && str_val(arg).unwrap() == "stable" {
                        } else {
                            self.word(node.defname.clone());
                            self.nbsp();
                            self.word(str_val(arg).unwrap());
                        }
                    }
                }
            }
            if let Some(option) = list.first() {
                if let NodeEnum::DefElem(node) = option.node.as_ref().unwrap() {
                    if let Some(arg) = &node.arg {
                        if node.defname == "as" {
                            self.hardbreak_if_nonempty();
                            self.word(node.defname.clone());
                            self.cbox(INDENT);
                            self.nbsp();
                            self.word("$$");
                            self.hardbreak_if_nonempty();
                            if let NodeEnum::List(List { items }) = arg.node.as_ref().unwrap() {
                                self.word(
                                    str_val(items.first().unwrap()).unwrap().trim().to_owned(),
                                );
                            }
                            self.hardbreak();
                            self.offset(-INDENT);
                            self.word("$$");
                        }
                    }
                    self.nbsp();
                }
            }
        }
    }

    pub fn opt_routine_body(&mut self, n: Option<&Node>) {
        if let Some(node) = n {
            self.word("as ");
            self.node(node);
            self.nbsp();
        }
    }

    pub fn arg_class(&mut self, n: &FunctionParameterMode) {
        self.function_parameter_mode(n);
    }

    pub fn param_name(&mut self, val: &str) {
        self.ident(val.to_string());
        self.word(" ");
    }

    pub fn func_type(&mut self, n: &TypeName) {
        self.type_name(n);
    }

    pub fn opt_drop_behavior(&mut self, n: &DropBehavior) {
        match n {
            DropBehavior::DropRestrict => {}
            DropBehavior::DropCascade => self.word("cascade "),
            _ => {}
        }
    }

    pub fn name_list(&mut self, list: &[Node]) {
        for (i, name) in list.iter().enumerate() {
            self.ident(str_val(name).unwrap());
            self.trailing_comma(i >= list.len() - 1);
        }
    }

    pub fn qual_op(&mut self, list: &[Node]) {
        if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
            self.word(str_val(list.first().unwrap()).unwrap());
        } else {
            self.word("operator(");
            self.any_operator(list);
            self.word(")");
        }
    }

    pub fn any_operator(&mut self, list: &[Node]) {
        match list.len() {
            2 => {
                self.ident(str_val(list.first().unwrap()).unwrap());
                self.word(".");
                self.word(str_val(list.last().unwrap()).unwrap());
            }
            1 => {
                self.ident(str_val(list.last().unwrap()).unwrap());
            }
            _ => unreachable!(),
        }
    }

    pub fn create_generic_options(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("options ");
            self.word("(");
            self.generic_option_list(list);
            self.word(") ");
        }
    }

    pub fn generic_option_list(&mut self, list: &[Node]) {
        self.qualified_name_list(list);
    }

    pub fn opt_temp(&mut self, relpersistence: String) {
        self.rel_persistence(&RelPersistence::from(relpersistence))
    }

    pub fn non_reserved_word_or_scont(&mut self, val: String) {
        match val.len() {
            0 => self.word("''".to_string()),
            x if x > NAMEDATALEN => string_literal(self, &val),
            _ => self.ident(val),
        }
    }

    pub fn opt_inherit(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("inherits ");
            self.word("(");
            self.qualified_name_list(list);
            self.word(") ");
        }
    }

    pub fn signed_iconst(&mut self, n: &Node) {
        self.word(format!("{}", int_val(n).unwrap()));
    }

    pub fn qualified_name_list(&mut self, list: &[Node]) {
        for (i, name) in list.iter().enumerate() {
            self.qualified_name(name);
            if i < list.len() - 1 {
                self.word(",");
                self.nbsp();
            }
        }
    }

    pub fn qualified_name(&mut self, n: &Node) {
        self.node(n);
    }

    pub fn name(&mut self, name: String) {
        self.ident(name);
    }

    pub fn opt_or_replace(&mut self, replace: bool) {
        if replace {
            self.word("or replace ");
        }
    }

    pub fn opt_table_func_element_list(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.table_func_element_list(list)
        }
    }

    pub fn table_func_element_list(&mut self, list: &[Node]) {
        for (i, n) in list.iter().enumerate() {
            let n = cast_node!(n, NodeEnum::ColumnDef(n)).unwrap();
            self.table_func_element(n);
            self.trailing_comma(i >= list.len() - 1)
        }
    }

    pub fn table_func_element(&mut self, n: &ColumnDef) {
        self.col_id(&n.colname);
        self.nbsp();
        if let Some(type_name) = &n.type_name {
            self.type_name(type_name);
        }
        self.opt_collate_clause(n.coll_clause.as_deref());
    }

    pub fn col_id(&mut self, s: &str) {
        self.ident(s.to_owned())
    }

    pub fn opt_as(&mut self) {
        self.word(" as ")
    }
}
