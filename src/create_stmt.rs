use crate::fmt;
use crate::fmt::DeparseNodeContext;
use crate::fmt::Print;
use crate::fmt::Printer;
use crate::rel_persistence::RelPersistence;
use crate::utils::a_const_int_val;
use crate::utils::int_val;
use crate::utils::is_op;
use crate::utils::str_val;
use crate::INDENT;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::CreateStmt;
use pg_query::protobuf::Integer;
use pg_query::protobuf::RangeVar;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

const MONTH: i32 = 1;
const YEAR: i32 = 2;
const DAY: i32 = 3;
const HOUR: i32 = 10;
const MINUTE: i32 = 11;
const SECOND: i32 = 12;

const INTERVAL_FULL_RANGE: i32 = 0x7FFF;
const INTERVAL_FULL_PRECISION: i32 = 0xFFFF;

impl fmt::Print for CreateStmt {
    fn print_in_context(&self, p: &mut Printer, ctx: &fmt::Context) -> fmt::Option {
        p.cbox(INDENT);
        p.keyword("create ");

        if ctx.is_foreign_table {
            p.keyword("foreign ");
        }

        RelPersistence::try_from(self.relation.as_ref().unwrap().relpersistence.clone())
            .ok()?
            .print(p)?;

        p.keyword("table ");

        if self.if_not_exists {
            p.keyword("if not exists ");
        }

        node_range_var(p, self.relation.as_ref().unwrap(), DeparseNodeContext::None);
        p.nbsp();

        if let Some(of_typename) = &self.of_typename {
            p.keyword("of ");
            of_typename.print(p)?;
            p.space();
        }

        if self.partbound.is_some() {
            p.keyword("partition of ");
            node_range_var(
                p,
                self.inh_relations
                    .first()
                    .and_then(|node| match node.node.as_ref().unwrap() {
                        NodeEnum::RangeVar(node) => Some(node),
                        _ => None,
                    })
                    .unwrap(),
                DeparseNodeContext::None,
            );
            p.word(" ");
        }

        if !self.table_elts.is_empty() {
            p.word("(");
            p.hardbreak_if_nonempty();
            for (i, elt) in self.table_elts.iter().enumerate() {
                node_table_element(p, elt);
                if i < self.table_elts.len() - 1 {
                    p.word(",");
                }
                p.hardbreak();
            }
            p.offset(-INDENT);
            p.end();
            p.word(")");
        } else if self.partbound.is_none() && self.of_typename.is_none() {
            p.word("()");
        };

        if let Some(partbound) = &self.partbound {
            partbound.print(p)?;
            p.word(" ");
        } else {
            node_opt_inherit(p, &self.inh_relations);
        }

        node_opt_with(p, &self.options);

        self.oncommit().print(p)?;

        if !self.tablespacename.is_empty() {
            p.keyword("tablespace ");
            p.ident(self.tablespacename.clone());
        }

        p.hardbreak();

        Some(())
    }
}

fn node_opt_inherit(_str: &mut Printer, list: &[Node]) {
    if !list.is_empty() {
        todo!("{:?}", list)
    }
}

pub fn node_expr_list(str: &mut Printer, list: &[Node]) {
    for (i, expr) in list.iter().enumerate() {
        expr.print(str);
        str.comma(i >= list.len() - 1);
    }
}

pub fn node_range_var(str: &mut Printer, node: &RangeVar, _context: DeparseNodeContext) {
    str.ident(node.relname.clone());
}

pub fn node_numeric_only(str: &mut Printer, val: &Val) {
    match val {
        Val::Ival(val) => str.word(format!("{}", val.ival)),
        Val::Fval(val) => str.word(val.fval.clone()),
        _ => unreachable!(),
    }
}

pub fn node_qual_op(str: &mut Printer, list: &[Node]) {
    if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
        str.word(str_val(list.first().unwrap()).unwrap());
    } else {
        str.word("operator(");
        node_any_operator(str, list);
        str.word(")");
    }
}

fn node_any_operator(str: &mut Printer, list: &[Node]) {
    match list.len() {
        2 => {
            str.ident(str_val(list.first().unwrap()).unwrap());
            str.word(".");
            str.word(str_val(list.last().unwrap()).unwrap());
        }
        1 => str.ident(str_val(list.last().unwrap()).unwrap()),
        _ => unreachable!(),
    }
}

pub fn node_col_label(str: &mut Printer, node: &str) {
    str.ident(node.to_owned());
}

pub fn node_opt_indirection(_str: &mut Printer, _list: &[Node], _offset: usize) {
    // for (i, item) in list.iter().enumerate().skip(offset) {}
}

pub fn node_signed_iconst(str: &mut Printer, node: &Node) {
    str.word(format!("{}", int_val(node).unwrap()));
}

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L3774.
pub fn node_interval_typmods(str: &mut Printer, node: &TypeName) {
    let interval_fields = node
        .typmods
        .first()
        .and_then(a_const_int_val)
        .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
        .map(|node| Node { node })
        .as_ref()
        .map(int_val)
        .unwrap()
        .unwrap();

    match interval_fields {
        x if x == 1 << YEAR => str.word(" year"),
        x if x == 1 << MONTH => str.word(" month"),
        x if x == 1 << DAY => str.word(" day"),
        x if x == 1 << HOUR => str.word(" hour"),
        x if x == 1 << MINUTE => str.word(" minute"),
        x if x == 1 << SECOND => str.word(" second"),
        x if x == 1 << YEAR | 1 << MONTH => str.word(" year to month"),
        x if x == 1 << DAY | 1 << HOUR => str.word(" day to hour"),
        x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE => str.word(" day to minute"),
        x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE | 1 << SECOND => str.word(" day to second"),
        x if x == 1 << HOUR | 1 << MINUTE => str.word(" hour to minute"),
        x if x == 1 << HOUR | 1 << MINUTE | 1 << SECOND => str.word(" hour to second"),
        x if x == 1 << MINUTE | 1 << SECOND => str.word(" minute to second"),
        INTERVAL_FULL_RANGE => {}
        _ => unreachable!(),
    };

    if node.typmods.len() == 2 {
        let precision = node
            .typmods
            .last()
            .and_then(a_const_int_val)
            .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
            .map(|node| Node { node })
            .as_ref()
            .map(int_val)
            .unwrap()
            .unwrap();

        if precision != INTERVAL_FULL_PRECISION {
            str.word(format!(" ({})", precision));
        }
    }
}

pub fn node_create_generic_options(_str: &mut Printer, _list: &[Node]) {
    todo!()
}

pub fn node_column_list(str: &mut Printer, list: &[Node]) {
    for (i, column) in list.iter().enumerate() {
        str.ident(str_val(column).unwrap());
        if i < list.len() - 1 {
            str.word(", ");
        }
    }
}

pub fn node_opt_with(str: &mut Printer, list: &[Node]) {
    if !list.is_empty() {
        str.keyword(" with ");
        node_rel_options(str, list);
        str.nbsp();
    }
}

fn node_rel_options(str: &mut Printer, list: &[Node]) {
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
                    node_def_arg(str, arg, false);
                }
            }
            _ => unreachable!(),
        }

        str.comma(i >= list.len() - 1);
    }

    str.word(")");
}

fn node_def_arg(str: &mut Printer, node: &Node, _is_operator_def_arg: bool) {
    match node.node.as_ref().unwrap() {
        NodeEnum::Integer(ref val) => Option::<Val>::print(&Some(Val::Ival(val.clone())), str),
        _ => todo!(),
    };
}

fn node_table_element(str: &mut Printer, node: &Node) {
    match node.node.as_ref().unwrap() {
        NodeEnum::ColumnDef(node) => node.print(str),
        NodeEnum::Constraint(node) => node.print(str),
        NodeEnum::IndexElem(_) => todo!(),
        NodeEnum::DefElem(_) => todo!(),
        _ => unreachable!(),
    };
}

pub fn node_any_name(str: &mut Printer, list: &[Node]) -> fmt::Option {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            str.word(".");
        }
        str.ident(str_val(part).unwrap());
    }

    Some(())
}
