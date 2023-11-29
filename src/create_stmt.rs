use crate::fmt;
use crate::fmt::Print;
use crate::fmt::Printer;
use crate::rel_persistence::RelPersistence;
use crate::utils::int_val;
use crate::utils::str_val;
use crate::INDENT;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::CreateStmt;
use pg_query::Node;
use pg_query::NodeEnum;

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

        self.relation.as_ref()?.print(p);
        p.nbsp();

        if let Some(of_typename) = &self.of_typename {
            p.keyword("of ");
            of_typename.print(p)?;
            p.space();
        }

        if self.partbound.is_some() {
            p.keyword("partition of ");
            self.inh_relations
                .first()
                .and_then(|node| match node.node.as_ref().unwrap() {
                    NodeEnum::RangeVar(node) => Some(node),
                    _ => None,
                })
                .unwrap()
                .print(p)?;
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

pub fn node_col_label(str: &mut Printer, node: &str) {
    str.ident(node.to_owned());
}

pub fn node_opt_indirection(_str: &mut Printer, _list: &[Node], _offset: usize) {
    // for (i, item) in list.iter().enumerate().skip(offset) {}
}

pub fn node_signed_iconst(str: &mut Printer, node: &Node) {
    str.word(format!("{}", int_val(node).unwrap()));
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
